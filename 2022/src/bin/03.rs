use aoc::read_file_input;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let res = solve(read_file_input("03.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    (solve_part_one(&input), solve_part_two(&input))
}

fn solve_part_one(input: &str) -> usize {
    let mut sum: usize = 0;

    for bag in input.lines() {
        let compartments = bag.split_at(bag.len() / 2);

        for char in compartments.0.chars() {
            match compartments.1.find(char) {
                Some(_) => match ALPHABET.find(char) {
                    Some(index) => {
                        sum += index + 1;
                        break;
                    }
                    None => {}
                },
                None => {}
            };
        }
    }

    sum
}

fn solve_part_two(input: &str) -> usize {
    let mut sum: usize = 0;
    let mut lines = input.lines();

    while let (Some(elf1_bag), Some(elf2_bag), Some(elf3_bag)) =
        (lines.next(), lines.next(), lines.next())
    {
        for char in elf1_bag.chars() {
            match elf2_bag.find(char) {
                Some(_) => {
                    match elf3_bag.find(char) {
                        Some(_) => match ALPHABET.find(char) {
                            Some(index) => {
                                sum += index + 1;
                                break;
                            }
                            None => {}
                        },
                        None => {}
                    };
                }
                None => {}
            };
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::{read_file_input, read_test_file_input};

    #[test]
    fn test_solve_one() {
        let res = solve_part_one(&read_test_file_input("03_one.txt".to_string()));
        assert_eq!(res, 157);
    }

    #[test]
    fn test_solve_two() {
        let res = solve_part_two(&read_test_file_input("03_one.txt".to_string()));
        assert_eq!(res, 70);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve_part_one(&read_file_input("03.txt".to_string()));
        assert_eq!(res, 7848);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve_part_two(&read_file_input("03.txt".to_string()));
        assert_eq!(res, 2616);
    }
}
