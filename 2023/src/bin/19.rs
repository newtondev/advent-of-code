use aoc::read_file_input;
use bstr::ByteSlice;
use fnv::FnvHashMap;
use itertools::Itertools;
use std::iter;

fn main() {
    let res = solve(read_file_input("19.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u64) {
    let p1 = solve_part_one(&input.as_bytes());
    let p2 = solve_part_two(&input.as_bytes());

    (p1, p2)
}

#[derive(Clone, Copy, Default)]
enum Operation {
    #[default]
    GreaterThan,
    LessThan,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Target {
    #[default]
    Reject,
    Accept,
    Rule([u8; 3]),
}

#[derive(Default, Clone)]
struct Ruleset {
    rules: (Rule, Option<Rule>, Option<Rule>),
    default_target: Target,
}

#[derive(Default, Clone, Copy)]
struct Rule {
    attr: u8,
    threshold: u16,
    target: Target,
    operation: Operation,
}

#[derive(Clone, Copy)]
struct ValueRange {
    min: u16,
    max: u16,
}

impl ValueRange {
    fn new(min: u16, max: u16) -> Self {
        Self { min, max }
    }

    fn combination_count(&self) -> u64 {
        (self.max + 1 - self.min) as u64
    }

    fn split(&self, threshold: u16, operation: Operation) -> (ValueRange, ValueRange) {
        match operation {
            Operation::GreaterThan => (
                ValueRange::new(self.min, threshold),
                ValueRange::new(threshold + 1, self.max),
            ),
            Operation::LessThan => (
                ValueRange::new(threshold, self.max),
                ValueRange::new(self.min, threshold - 1),
            ),
        }
    }

    fn is_empty(&self) -> bool {
        self.min > self.max
    }
}

fn pad_rule(rule: &[u8]) -> [u8; 3] {
    let mut buf = [b' '; 3];

    buf[3 - rule.len()..].copy_from_slice(rule);
    buf
}

fn parse_rules(rule_desc: &[u8]) -> FnvHashMap<[u8; 3], Ruleset> {
    rule_desc
        .lines()
        .fold(FnvHashMap::default(), |mut rules, line| {
            let line = &line[..line.len() - 1];
            let (name, instructions) = line.split_once_str("{").unwrap();
            let name = pad_rule(name.as_bytes());

            let mut rs = Ruleset::default();
            for (i, ins) in instructions.split_str(",").enumerate() {
                let Some((cond, target)) = ins.split_once_str(":") else {
                    rs.default_target = match ins {
                        b"R" => Target::Reject,
                        b"A" => Target::Accept,
                        target => Target::Rule(pad_rule(target)),
                    };
                    break;
                };

                let attr = match cond[0] {
                    b'x' => 0,
                    b'm' => 1,
                    b'a' => 2,
                    b's' => 3,
                    _ => unreachable!(),
                };

                let operation = match cond[1] {
                    b'<' => Operation::LessThan,
                    b'>' => Operation::GreaterThan,
                    _ => unreachable!(),
                };

                let threshold = unsafe { cond[2..].to_str_unchecked() }
                    .parse::<u16>()
                    .unwrap();
                let target = match target {
                    b"R" => Target::Reject,
                    b"A" => Target::Accept,
                    _ => Target::Rule(pad_rule(target)),
                };

                let r = Rule {
                    attr,
                    operation,
                    threshold,
                    target,
                };

                match i {
                    0 => rs.rules.0 = r,
                    1 => rs.rules.1 = Some(r),
                    2 => rs.rules.2 = Some(r),
                    _ => unreachable!(),
                }
            }

            rules.insert(name, rs);
            rules
        })
}

fn solve_part_one(input: &[u8]) -> u32 {
    let (rules, part) = input.split_once_str("\n\n").unwrap();
    let parts = part.lines().map(|line| {
        let line = &line[1..line.len() - 1];
        let (x, m, a, s) = line
            .split_str(",")
            .map(|part| {
                unsafe { part[2..].to_str_unchecked() }
                    .parse::<u16>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        (x, m, a, s)
    });

    let rules = parse_rules(rules);

    let mut valid = 0;
    'nextpart: for (x, m, a, s) in parts {
        let mut cursor = &rules[b" in"];
        let xmas = [x, m, a, s];

        'nextrule: loop {
            for rule in iter::once(&cursor.rules.0)
                .chain(cursor.rules.1.iter())
                .chain(cursor.rules.2.iter())
            {
                let rule_matches = match rule.operation {
                    Operation::GreaterThan => xmas[rule.attr as usize] > rule.threshold,
                    Operation::LessThan => xmas[rule.attr as usize] < rule.threshold,
                };
                if rule_matches {
                    match rule.target {
                        Target::Reject => continue 'nextpart,
                        Target::Accept => {
                            valid += xmas.iter().sum::<u16>() as u32;
                            continue 'nextpart;
                        }
                        Target::Rule(target) => {
                            cursor = &rules[&target];
                            continue 'nextrule;
                        }
                    }
                }
            }
            match cursor.default_target {
                Target::Reject => continue 'nextpart,
                Target::Accept => {
                    valid += xmas.iter().sum::<u16>() as u32;
                    continue 'nextpart;
                }
                Target::Rule(target) => {
                    cursor = &rules[&target];
                    continue 'nextrule;
                }
            }
        }
    }

    valid
}

fn solve_part_two(input: &[u8]) -> u64 {
    let (rules, _) = input.split_once_str("\n\n").unwrap();
    let rules = parse_rules(rules);

    let mut queue = vec![(
        [b' ', b'i', b'n'],
        [
            ValueRange::new(1, 4000),
            ValueRange::new(1, 4000),
            ValueRange::new(1, 4000),
            ValueRange::new(1, 4000),
        ],
    )];
    let mut out = 0;
    while let Some((id, bounds)) = queue.pop() {
        let cursor = &rules[&id];
        let mut bounds = bounds;

        for rule in iter::once(&cursor.rules.0)
            .chain(cursor.rules.1.iter())
            .chain(cursor.rules.2.iter())
        {
            let mut rule_match_bounds = bounds;
            (
                bounds[rule.attr as usize],
                rule_match_bounds[rule.attr as usize],
            ) = bounds[rule.attr as usize].split(rule.threshold, rule.operation);

            if !rule_match_bounds[rule.attr as usize].is_empty() {
                match rule.target {
                    Target::Reject => (),
                    Target::Accept => {
                        out += rule_match_bounds
                            .iter()
                            .map(ValueRange::combination_count)
                            .product::<u64>();
                        continue;
                    }
                    Target::Rule(target) => queue.push((target, rule_match_bounds)),
                }
            }

            if bounds[rule.attr as usize].is_empty() {
                continue;
            }
        }

        match cursor.default_target {
            Target::Reject => continue,
            Target::Accept => {
                out += bounds
                    .iter()
                    .map(ValueRange::combination_count)
                    .product::<u64>();
            }
            Target::Rule(target) => {
                queue.push((target, bounds));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let (res, _) = solve(read_test_file_input("19_one.txt".to_string()));
        assert_eq!(res, 19114);
    }

    #[test]
    fn test_solve_two() {
        let (_, res) = solve(read_test_file_input("19_one.txt".to_string()));
        assert_eq!(res, 167_409_079_868_000);
    }

    #[test]
    fn actual_solve_one() {
        let (res, _) = solve(read_file_input("19.txt".to_string()));
        assert_eq!(res, 367602);
    }

    #[test]
    fn actual_solve_two() {
        let (_, res) = solve(read_file_input("19.txt".to_string()));
        assert_eq!(res, 125_317_461_667_458);
    }
}
