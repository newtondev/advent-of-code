use aoc::read_file_input;

fn main() {
    let res = solve(read_file_input("00.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(_input: String) -> (u32, u32) {
    // let mut p1 = 0;
    // let mut p2 = 0;

    // (p1, p2)
    (0, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("00_one.txt".to_string()));
        assert_eq!(res.0, 0);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("00_one.txt".to_string()));
        assert_eq!(res.1, 1);
    }
}
