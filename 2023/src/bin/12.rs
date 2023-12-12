use std::collections::HashMap;

use aoc::read_file_input;

fn main() {
    let res = solve(read_file_input("12.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let p1 = sum_arrangements(&input, false);
    let p2 = sum_arrangements(&input, true);

    (p1, p2)
}

type Cache = HashMap<(Vec<usize>, Vec<char>), usize>;

fn sum_arrangements(input: &str, unfold: bool) -> usize {
    let mut cache: Cache = HashMap::new();

    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            //println!("{:?}", parts);

            let springs: Vec<char>;
            let groups: Vec<usize>;

            if unfold {
                springs = unfold_input(parts[0].split('?').collect())
                    .join("?")
                    .chars()
                    .collect();

                groups = unfold_input(parts[1].split(',').collect())
                    .join(",")
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
            } else {
                springs = parts[0].chars().collect();
                //println!("{:?}", springs);

                groups = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
            }

            (springs, groups)
        })
        .collect::<Vec<(Vec<char>, Vec<usize>)>>()
        .iter()
        .map(|(springs, groups)| calculate(springs, groups, &mut cache))
        .sum()
}

fn unfold_input(input: Vec<&str>) -> Vec<&str> {
    input
        .iter()
        .cycle()
        .take(input.len() * 5)
        .cloned()
        .collect::<Vec<&str>>()
}

fn calculate(chars: &Vec<char>, group_sizes: &Vec<usize>, cache: &mut Cache) -> usize {
    if chars.is_empty() {
        if group_sizes.is_empty() {
            return 1;
        };
        return 0;
    }

    match chars[0] {
        '.' => calculate(&chars[1..].to_vec(), group_sizes, cache),
        '#' => calculate_hashed(group_sizes, chars, cache),
        '?' => {
            calculate(&chars[1..].to_vec(), group_sizes, cache)
                + calculate_hashed(group_sizes, chars, cache)
        }
        _ => panic!("Problem calculating"),
    }
}

fn calculate_hashed(group_sizes: &Vec<usize>, chars: &Vec<char>, cache: &mut Cache) -> usize {
    if let Some(&res) = cache.get(&(group_sizes.clone(), chars.clone())) {
        return res;
    }

    if group_sizes.is_empty() {
        return 0;
    }

    let ps = group_sizes[0] as usize;
    if chars.len() < ps {
        return 0;
    }
    for i in 0..ps {
        if chars[i] == '.' {
            return 0;
        }
    }
    if chars.len() == ps {
        if group_sizes.len() == 1 {
            return 1;
        }
        return 0;
    }
    if chars[ps] == '#' {
        return 0;
    }

    let res = calculate(
        &chars[(ps + 1)..].to_vec(),
        &group_sizes[1..].to_vec(),
        cache,
    );
    cache.insert((group_sizes.clone(), chars.clone()), res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one_a() {
        let res = sum_arrangements(&read_test_file_input("12_one_a.txt".to_string()), false);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_solve_one_b() {
        let res = sum_arrangements(&read_test_file_input("12_one_b.txt".to_string()), false);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_solve_two() {
        let res = sum_arrangements(&read_test_file_input("12_one_b.txt".to_string()), true);
        assert_eq!(res, 525_152);
    }

    #[test]
    fn actual_solve_one() {
        let res = sum_arrangements(&read_file_input("12.txt".to_string()), false);
        assert_eq!(res, 7_090);
    }

    #[test]
    fn actual_solve_two() {
        let res = sum_arrangements(&read_file_input("12.txt".to_string()), true);
        assert_eq!(res, 6_792_010_726_878);
    }
}
