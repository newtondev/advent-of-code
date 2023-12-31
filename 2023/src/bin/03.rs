use aoc::read_file_input;
use std::{
    collections::{HashMap, HashSet},
    u32,
};

fn main() {
    let res = solve(read_file_input("03.txt".to_string()));

    // Print the results
    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u32) {
    let mut parts: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut chars: HashSet<(usize, usize)> = HashSet::new();
    let mut board: Vec<Vec<char>> = Vec::new();

    for (r, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        for (c, ch) in row.iter().enumerate() {
            if !ch.is_digit(10) && *ch != '.' {
                chars.insert((r, c));
            }
        }
        board.push(row);
    }

    // Iterate through the board to calcaulate results
    for (r, row) in board.iter().enumerate() {
        for m in regex::Regex::new(r"\d+")
            .unwrap()
            .find_iter(&row.iter().collect::<String>())
        {
            //println!("m={}", m.as_str());
            //println!("start={} | end={}", m.start().to_string(), m.end().to_string());

            // Considering we want offsets, ensure that we scan the range before and after
            let nexts: HashSet<(usize, usize)> = (-1..=1) // Consider row offsets from -1 to 1
                .flat_map(|s| {
                    (-1..=1).flat_map(move |d| {
                        (0..m.end() - m.start())
                            .map(move |c| (r as i32 + s, c as i32 + m.start() as i32 + d))
                    })
                })
                .filter(|&(row, col)| row >= 0 && col >= 0)
                .map(|(row, col)| (row as usize, col as usize))
                .collect();

            //println!("Nexts: {:?}", nexts);

            for &c in nexts.intersection(&chars) {
                parts
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(m.as_str().parse().unwrap());
            }
        }
    }

    //println!("{:?}", parts.values());

    // Calcaulate the results
    let p1: u32 = parts.values().flatten().sum();
    let p2: u32 = parts
        .values()
        .filter(|p| p.len() == 2)
        .map(|p| p.iter().product::<u32>())
        .sum();

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("03_one.txt".to_string()));
        assert_eq!(res.0, 4361);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("03_two.txt".to_string()));
        assert_eq!(res.1, 467835);
    }
}
