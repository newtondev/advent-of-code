use aoc::read_file_input;
use std::collections::HashMap;

fn main() {
    let res = solve(read_file_input("02.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let game_id:u32 = line
            .split(":")
            .next().unwrap()
            .split_whitespace()
            .nth(1).unwrap()
            .parse().unwrap();

        let mut min_counts: HashMap<&str, u32> = HashMap::new();
        let mut possible = true;

        for part in line.split(":").nth(1).unwrap().split(";") {
            let mut counts: HashMap<&str, u32> = HashMap::new();

            for segment in part.split(", ") {
                let parts: Vec<&str> = segment.split_whitespace().collect();
                let colour = parts[1];
                let count: u32 = parts[0].parse().unwrap();
                *counts.entry(colour).or_insert(0) += count;
            }

            for (&colour, &count) in counts.iter() {
                *min_counts.entry(colour).or_insert(0) = min_counts.get(colour).map_or(count, |&x| x.max(count));
            }

            if !(counts.get("red").map_or(0, |&x| x) <= 12
                && counts.get("green").map_or(0, |&x| x) <= 13
                && counts.get("blue").map_or(0, |&x| x) <= 14) {
                    possible = false;
                }
        }

        if possible {
            p1 += game_id;
        }

        p2 += min_counts.get("red").unwrap_or(&0) * 
              min_counts.get("green").unwrap_or(&0) *
              min_counts.get("blue").unwrap_or(&0);
    }

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("02_one.txt".to_string()));
        assert_eq!(
            res.0,
            8
        );
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("02_two.txt".to_string()));
        assert_eq!(
            res.1,
            2286
        );
    }
}
