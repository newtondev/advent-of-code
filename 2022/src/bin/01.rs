// See: https://adventofcode.com/2022/day/1
use aoc::read_file_input;

fn main() {
    let res = solve(read_file_input("01.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (i32, i32) {
    let mut current = 0;
    let mut max: (i32, i32, i32) = (0, 0, 0);

    for line in input.lines() {
        if line.is_empty() {
            max = calculate(current, max);
            current = 0;
        } else {
            current += line.trim().parse::<i32>().unwrap()
        }
    }

    max = calculate(current, max);

    (max.0, max.0 + max.1 + max.2)
}

fn calculate(current: i32, max: (i32, i32, i32)) -> (i32, i32, i32) {
    let mut max = max;
    if current > max.0 {
        max.2 = max.1;
        max.1 = max.0;
        max.0 = current;
    } else if current > max.1 {
        max.2 = max.1;
        max.1 = current;
    } else if current > max.2 {
        max.2 = current;
    }
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("01_one.txt".to_string()));
        assert_eq!(res.0, 24000);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("01_one.txt".to_string()));
        assert_eq!(res.1, 45000);
    }
}
