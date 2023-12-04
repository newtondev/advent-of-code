use aoc::read_file_input;
use std::{collections::HashSet, u32};

fn main() {
    let res = solve(read_file_input("04.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (i32, i32) {
    let mut p1: i32 = 0;
    let mut matches: Vec<[i32; 2]> = Vec::new();

    for line in input.lines() {
        let s: Vec<HashSet<i32>> = line
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .map(|s| s.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect();

        let s2 = s[0].intersection(&s[1]).count() as i32;
        matches.push([1, s2]);

        if s2 > 0 {
            p1 += 2_i32.pow(s2 as u32 -1);
        }
    }

    // Sole problem two    
    for i in 0..matches.len() {
        for j in 1..=matches[i][1] {
            matches[i + j as usize][0] += matches[i][0];
        }
    }
    let p2: i32 = matches.iter().map(|&m| m[0]).sum();

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("04_one.txt".to_string()));
        assert_eq!(
            res.0,
            13
        );
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("04_one.txt".to_string()));
        assert_eq!(
            res.1,
            30
        );
    }
}
