use aoc::read_file_input;
use regex::Regex;

fn main() {
    let res = solve(read_file_input("06.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (i64, i64) {
    let (times, distances) = input
        .trim()
        .split_once("\n")
        .unwrap();

    let p1: i64 = calculate(&times, &distances);
    let p2: i64 = calculate(&times.replace(" ", ""), &distances.replace(" ", ""));

    (p1, p2)
}

fn calculate(times: &str, distances: &str) -> i64 {
    let re = Regex::new(r"\d+").unwrap();

    let t: Vec<u64> = re.find_iter(times).map(|m| m.as_str().parse().unwrap()).collect();
    let d: Vec<u64> = re.find_iter(distances).map(|m| m.as_str().parse().unwrap()).collect();

    t.iter()
        .zip(d.iter())
        .fold(1, |acc, (&time, &best_distance)| {
            let n: i64 = (0..=time).filter(|&i| i * (time - i) > best_distance).count() as i64;
            acc * n
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("06_one.txt".to_string()));
        assert_eq!(
            res.0,
            288
        );
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("06_one.txt".to_string()));
        assert_eq!(
            res.1,
            71503
        );
    }
}
