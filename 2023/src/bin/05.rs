use std::usize;

use aoc::read_file_input;

fn main() {
    let p1 = solve_part_one(read_file_input("05.txt".to_string()));
    let p2 = solve_part_two(read_file_input("05.txt".to_string()));
    
    println!("{}", p1);
    println!("{}", p2);
}

fn solve_part_one(input: String) -> u32 {
    // Split up the subsections
    let parts: Vec<_> = input.split("\n\n").collect();

    // Collect the seeds
    let seeds = parts
        .first()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()
        .iter()
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mappings = parts
        .iter()
        .skip(1)
        .map(|mapping| {
            mapping
                .split(":\n")
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .flat_map(|s| s.split('\n').collect::<Vec<_>>())
                .map(|l| {
                    l
                        .split(' ')
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Location mapping.
    let locs = seeds
        .iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(*seed, |id, mapping| {
                    let res = mapping.iter().find(|m| {
                        m[1] <= id && id <= m[1] + m[2]
                    });
                    match res {
                        Some(m) => {
                            let offset = id - m[1];
                            m[0] + offset
                        }
                        None => id
                    }
                })
        })
        .collect::<Vec<_>>();

    *locs.iter().min().unwrap() as u32
}

#[derive(Debug, Clone)]
struct Range(i64, i64);

fn solve_part_two(input: String) -> i64 {
    // Split up the subsections
    let parts: Vec<_> = input.split("\n\n").collect();

    let seed_ranges: Vec<Range> = parts
        .first()
        .unwrap()
        .split(": ")
        .collect::<Vec<_>>()
        .iter()
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| Range(s[0], s[0] + s[1]))
        .collect::<Vec<_>>();

    let mappings = parts
        .iter()
        .skip(1)
        .map(|mapping| {
            let mut m = mapping
                .split(":\n")
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .flat_map(|s| s.split('\n').collect::<Vec<_>>())
                .map(|l| {
                    l
                        .split(' ')
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            m.sort_by(|x, y| x[1].cmp(&y[1]));
            m
        })
        .collect::<Vec<_>>();

    let mut ranges = seed_ranges.clone();
    for m in mappings.iter() {
        let mut n_ranges: Vec<Range> = vec![];

        for range in ranges.iter() {
            let mut cur = range.clone();
            for mapping in m.iter() {
                let offset = mapping[0] - mapping[1];
                if cur.0 <= cur.1 && cur.0 < mapping[1] + mapping[2] && mapping[1] <= cur.1 {
                    
                    if cur.0 < mapping[1] {
                        n_ranges.push(Range(cur.0, mapping[1] - 1));
                        cur.0 = mapping[1];

                        if cur.1 < mapping[1] + mapping[2] {
                            n_ranges.push(Range(
                                cur.0 + offset, 
                                cur.1 + offset,
                            ));
                            cur.0 = cur.1 + 1;
                        } else {
                            n_ranges.push(Range(
                                cur.0 + offset, 
                                mapping[1] + mapping[2] -1 + offset,
                            ));
                            cur.0 = mapping[1] + mapping[2];
                        }
                    } else if cur.1 < mapping[1] + mapping[2] {
                        n_ranges.push(Range(
                            cur.0 + offset,
                            cur.1 + offset,
                        ));
                        cur.0 = cur.1 + 1;
                    } else {
                        n_ranges.push(Range(
                            cur.0 + offset,
                            mapping[1] + mapping[2] -1 + offset,
                        ));
                        cur.0 = mapping[1] + mapping[2];
                    }

                }
            }

            if cur.0 <= cur.1 {
                n_ranges.push(cur)
            }
        }

        ranges = n_ranges;
    }

    ranges.sort_by(|x, y| x.0.cmp(&y.0));

    ranges
        .first()
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve_part_one(read_test_file_input("05_one.txt".to_string()));
        assert_eq!(
            res,
            35
        );
    }

    #[test]
    fn test_solve_two() {
        let res = solve_part_two(read_test_file_input("05_one.txt".to_string()));
        assert_eq!(
            res,
            46
        );
    }
}
