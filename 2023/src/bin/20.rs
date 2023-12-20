use std::collections::VecDeque;

use aoc::read_file_input;
use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum DestinationModuleKind<'a> {
    FlipFlop { off: bool },
    Conjunction { last_pulses: Vec<(&'a str, Pulse)> },
}

#[derive(Debug)]
struct DestinationModule<'a> {
    kind: DestinationModuleKind<'a>,
    outputs: Vec<&'a str>,
}

fn main() {
    let res = solve(read_file_input("20.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u64) {
    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    (p1, p2)
}

fn parse(input: &str) -> (Vec<&str>, FnvHashMap<&str, DestinationModule>) {
    let broadcaster_targets = input
        .lines()
        .find_map(|line| {
            line.strip_prefix("broadcaster -> ")
                .map(|targets| targets.split(", ").collect_vec())
        })
        .unwrap();

    let mut rules = input
        .lines()
        .filter(|line| !line.starts_with("broadcaster"))
        .map(|line| {
            let (name, outputs) = line[1..].split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").collect_vec();

            let kind = match line.chars().nth(0).unwrap() {
                '%' => DestinationModuleKind::FlipFlop { off: true },
                '&' => DestinationModuleKind::Conjunction {
                    last_pulses: vec![],
                },
                _ => unreachable!(),
            };

            let module = DestinationModule { kind, outputs };
            (name, module)
        })
        .collect::<FnvHashMap<_, _>>();

    let input_to_outputs = rules
        .iter()
        .map(|(name, module)| (*name, module.outputs.clone()))
        .collect_vec();

    for (name, outputs) in input_to_outputs {
        outputs
            .iter()
            .filter(|s| s != &&"output")
            .for_each(|output| {
                let Some(module) = rules.get_mut(output) else {
                    return;
                };
                if let DestinationModuleKind::Conjunction { last_pulses } = &mut module.kind {
                    last_pulses.push((name, Pulse::Low));
                }
            });
    }

    (broadcaster_targets, rules)
}

fn cycle<'a>(
    queue: &mut VecDeque<(&'a str, &'a str, Pulse)>,
    rules: &mut FnvHashMap<&'a str, DestinationModule<'a>>,
    name: &'a str,
    parent: &'a str,
    pulse: Pulse,
) {
    let Some(module) = rules.get_mut(name) else {
        return;
    };

    let pulse_type = match &mut module.kind {
        DestinationModuleKind::FlipFlop { off } => {
            if pulse == Pulse::High {
                return;
            }
            let pulse_type = if *off { Pulse::High } else { Pulse::Low };
            *off = !*off;
            pulse_type
        }
        DestinationModuleKind::Conjunction { last_pulses } => {
            last_pulses
                .iter_mut()
                .find(|(input, _)| input == &parent)
                .unwrap()
                .1 = pulse;

            let all_high = last_pulses.iter().all(|(_, pulse)| *pulse == Pulse::High);
            if all_high {
                Pulse::Low
            } else {
                Pulse::High
            }
        }
    };

    for output in &module.outputs {
        queue.push_back((output, name, pulse_type));
    }
}

fn solve_part_one(input: &str) -> u32 {
    let (broadcaster_targets, mut rules) = parse(input);
    let (mut output_low, mut output_high) = (1000, 0);
    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        for target in &broadcaster_targets {
            queue.push_back((*target, "broadcaster", Pulse::Low));
        }

        while let Some((name, parent, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => output_high += 1,
                Pulse::Low => output_low += 1,
            };
            cycle(&mut queue, &mut rules, name, parent, pulse);
        }
    }

    output_low * output_high
}

// greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

// least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve_part_two(input: &str) -> u64 {
    let (broadcaster_targets, mut rules) = parse(input);

    let rx_parent = rules
        .iter()
        .find_map(|(name, module)| module.outputs.contains(&&"rx").then_some(*name))
        .unwrap();

    let rx_parent_inputs = {
        let module = &rules[rx_parent];
        let DestinationModuleKind::Conjunction { last_pulses } = &module.kind else {
            unreachable!();
        };
        last_pulses.iter().map(|(input, _)| *input).collect_vec()
    };

    let mut loop_index = rx_parent_inputs.iter().map(|_| None).collect_vec();
    let mut it = 1;
    let mut queue = VecDeque::new();
    while loop_index.iter().any(Option::is_none) {
        for target in &broadcaster_targets {
            queue.push_back((*target, "broadcaster", Pulse::Low));
        }

        while let Some((name, parent, pulse)) = queue.pop_front() {
            cycle(&mut queue, &mut rules, name, parent, pulse);
            if name == rx_parent {
                let index = rx_parent_inputs
                    .iter()
                    .position(|input| input == &parent)
                    .unwrap();

                if loop_index[index].is_none() && pulse == Pulse::High {
                    loop_index[index] = Some(it);
                }
            }
        }

        it += 1
    }

    loop_index
        .into_iter()
        .map(Option::unwrap)
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one_a() {
        let res = solve_part_one(&read_test_file_input("20_one_a.txt".to_string()));
        assert_eq!(res, 32_000_000);
    }

    #[test]
    fn test_solve_one_b() {
        let res = solve_part_one(&read_test_file_input("20_one_b.txt".to_string()));
        assert_eq!(res, 11_687_500);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve_part_one(&read_file_input("20.txt".to_string()));
        assert_eq!(res, 703_315_117);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve_part_two(&read_file_input("20.txt".to_string()));
        assert_eq!(res, 230_402_300_925_361);
    }
}
