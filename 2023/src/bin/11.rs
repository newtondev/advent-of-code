#![feature(cmp_minmax)]
use std::cmp;

use aoc::read_file_input;
use itertools::Itertools;

fn main() {
    let res = solve(&read_file_input("11.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: &str) -> (usize, usize) {
    let p1 = calculate(input, 2);
    let p2 = calculate(input, 1_000_000);

    (p1, p2)
}

fn calculate(input: &str, expansion_factor: usize) -> usize {
    let stars = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| memchr::memchr_iter(b'#', line.as_bytes()).map(move |x| (x, y)))
        .collect_vec();

    // deduplicate
    let stars_y = stars.iter().map(|(_, y)| *y).sorted().dedup().collect_vec();

    let stars_x = stars.iter().map(|(x, _)| *x).sorted().dedup().collect_vec();

    // calculate the distances
    stars
        .into_iter()
        .map(|(x, y)| {
            let x_pos = stars_x.binary_search(&x).unwrap();
            let y_pos = stars_y.binary_search(&y).unwrap();
            (x_pos, y_pos)
        })
        .tuple_combinations()
        .map(|(star1, star2)| {
            let [x1, x2] = cmp::minmax(star1.0, star2.0);
            let [y1, y2] = cmp::minmax(star1.1, star2.1);

            let distance_x = distance_between(&stars_x[x1..=x2], expansion_factor);
            let distance_y = distance_between(&stars_y[y1..=y2], expansion_factor);

            distance_x + distance_y
        })
        .sum()
}

fn distance_between(stars: &[usize], expansion_factor: usize) -> usize {
    stars
        .iter()
        .tuple_windows()
        .map(|(s1, s2)| match s2 - s1 {
            0 => 0,
            1 => 1,
            n => 1 + expansion_factor * (n - 1),
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = calculate(&read_test_file_input("11_one.txt".to_string()), 2);
        assert_eq!(res, 374);
    }

    #[test]
    fn test_solve_two_a() {
        let res = calculate(&read_test_file_input("11_one.txt".to_string()), 10);
        assert_eq!(res, 1030);
    }

    #[test]
    fn test_solve_two_b() {
        let res = calculate(&read_test_file_input("11_one.txt".to_string()), 100);
        assert_eq!(res, 8410);
    }

    #[test]
    fn actual_solve_one() {
        let res = calculate(&read_file_input("11.txt".to_string()), 2);
        assert_eq!(res, 9509330);
    }

    #[test]
    fn actual_solve_two() {
        let res = calculate(&read_file_input("11.txt".to_string()), 1_000_000);
        assert_eq!(res, 635832237682);
    }
}
