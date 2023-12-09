use aoc::read_file_input;
use itertools::Itertools;

fn main() {
    let res = solve(read_file_input("09.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn calulate_differences(history: &Vec<Vec<i64>>) -> Vec<Vec<Vec<i64>>> {
    history
        .iter()
        .map(|h| {
            let mut diffs = vec![];
            diffs.push(h.to_owned());
            while diffs.last().unwrap().iter().any(|a| *a != 0) {
                let diff = diffs
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
                diffs.push(diff);
            }
            diffs
        })
        .collect()
}

fn solve_part(diffs: &Vec<Vec<Vec<i64>>>, reverse: bool) -> i64 {
    diffs
        .iter()
        .map(|diff| {
            diff.iter().rev().fold(0, |a, b| {
                if reverse {
                    b.first().unwrap() - a
                } else {
                    a + b.last().unwrap()
                }
            })
        })
        .sum()
}

fn solve(input: String) -> (i64, i64) {
    let history: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();
    //println!("{:?}", history);

    let diffs = calulate_differences(&history);
    //println!("{:?}", diffs);

    let p1: i64 = solve_part(&diffs, false);
    let p2: i64 = solve_part(&diffs, true);
    //println!("p1={} | p2={}", p1, p2);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("09_one.txt".to_string()));
        assert_eq!(res.0, 114);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("09_one.txt".to_string()));
        assert_eq!(res.1, 2);
    }
}
