use aoc::read_file_input;
use itertools::{izip, Itertools};

fn main() {
    let res = solve(read_file_input("13.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let p1 = calculate(&input, false);
    let p2 = calculate(&input, true);

    (p1, p2)
}

fn calculate(input: &str, with_smudges: bool) -> usize {
    input
        .split("\n\n")
        .map(|part| {
            let cols = {
                let mut res = vec![0; part.lines().next().unwrap().len()];
                for (i, row) in part.lines().enumerate() {
                    for (j, ch) in row.chars().enumerate() {
                        res[j] |= u32::from(ch == '#') << i;
                    }
                }
                res
            };

            let rows = {
                part.lines()
                    .map(|line| {
                        line.chars()
                            .enumerate()
                            .fold(0, |acc, (i, ch)| acc | (u32::from(ch == '#') << i))
                    })
                    .collect_vec()
            };

            palindrome(&cols, with_smudges) + palindrome(&rows, with_smudges) * 100
        })
        .sum::<usize>()
}

fn palindrome(input: &Vec<u32>, with_smudges: bool) -> usize {
    (1..input.len())
        .find(|&center| {
            let (left, right) = input.split_at(center);
            let mut is_smudged = false;
            izip!(left.iter().rev(), right.iter()).all(|(left, right)| match left == right {
                true => true,
                false if with_smudges && !is_smudged => {
                    let can_smudge = (left ^ right).count_ones() == 1;
                    if can_smudge {
                        is_smudged = true;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }) && (!with_smudges || is_smudged)
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("13_one.txt".to_string()));
        assert_eq!(res.0, 405);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("13_one.txt".to_string()));
        assert_eq!(res.1, 400);
    }
}
