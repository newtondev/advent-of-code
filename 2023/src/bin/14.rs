use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use aoc::read_file_input;
use itertools::{izip, Itertools};

fn main() {
    let res = solve(read_file_input("14.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    (p1, p2)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
}

type RowVec = Vec<Option<Rock>>;
type DishVec = Vec<RowVec>;

fn solve_part_one(input: &str) -> usize {
    let num_rows = input.lines().count();

    let mut dish: DishVec = vec![vec![None; num_rows]; num_rows];
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            dish[j][i] = match ch {
                '.' => None,
                'O' => Some(Rock::Round),
                '#' => Some(Rock::Cube),
                _ => unreachable!(),
            }
        }
    }

    dish.into_iter()
        .map(|col| {
            izip!(col.into_iter(), (1..=num_rows).rev())
                .filter_map(|(c, i)| Some((c?, i)))
                .scan(num_rows + 1, |last_pos, (rock, i)| match rock {
                    Rock::Round => {
                        *last_pos -= 1;
                        Some(*last_pos)
                    }
                    Rock::Cube => {
                        *last_pos = i;
                        Some(0)
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn solve_part_two(input: &str) -> usize {
    let mut cache: Vec<(u64, DishVec)> = Vec::new();
    let mut dish: DishVec = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => None,
                    'O' => Some(Rock::Round),
                    '#' => Some(Rock::Cube),
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    for i in 0..1e9 as _ {
        spin_cycle(&mut dish);
        let mut hasher = BuildHasherDefault::<fnv::FnvHasher>::default().build_hasher();
        dish.hash(&mut hasher);
        let dish_hash = hasher.finish();

        if let Some(spin_cycle_start) = cache
            .iter()
            .position(|(h, m)| h == &dish_hash && m == &dish)
        {
            let spin_cycle_len = i - spin_cycle_start;
            let remaining_spins = 1e9 as usize - i - 1;
            let rem = remaining_spins % spin_cycle_len;

            return calculate_load(&cache[spin_cycle_start + rem].1);
        }

        cache.push((dish_hash, dish.clone()))
    }
    calculate_load(&dish)
}

fn spin_cycle(dish: &mut [RowVec]) {
    for _ in 0..4 {
        turn(dish);
    }
}

fn turn(dish: &mut [RowVec]) {
    let n = dish.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let tmp = dish[i][j];
            dish[i][j] = dish[n - j - 1][i];
            dish[n - j - 1][i] = dish[n - i - 1][n - j - 1];
            dish[n - i - 1][n - j - 1] = dish[j][n - i - 1];
            dish[j][n - i - 1] = tmp;
        }
    }

    for row in dish.iter_mut() {
        let mut last_pos = n;
        for x in (0..n).rev() {
            match row[x] {
                None => (),
                Some(Rock::Round) => {
                    last_pos -= 1;
                    row[x] = None;
                    row[last_pos] = Some(Rock::Round);
                }
                Some(Rock::Cube) => {
                    last_pos = x;
                }
            }
        }
    }
}

fn calculate_load(dish: &[RowVec]) -> usize {
    izip!(dish.iter(), (1..=dish.len()).rev())
        .map(|(col, val)| {
            val * (col
                .iter()
                .filter(|c| matches!(c, Some(Rock::Round)))
                .count())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve_part_one(&read_test_file_input("14_one.txt".to_string()));
        assert_eq!(res, 136);
    }

    #[test]
    fn test_solve_two() {
        let res = solve_part_two(&read_test_file_input("14_one.txt".to_string()));
        assert_eq!(res, 64);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve_part_one(&read_file_input("14.txt".to_string()));
        assert_eq!(res, 107053);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve_part_two(&read_file_input("14.txt".to_string()));
        assert_eq!(res, 88371);
    }
}
