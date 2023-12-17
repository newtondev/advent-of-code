use aoc::read_file_input;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::mem;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Default)]
struct Visited {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Visited {
    fn visit_dir(&mut self, dir: Direction) -> bool {
        let field = match dir {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        };
        mem::replace(field, true)
    }

    fn is_energized(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}

fn calculate_energy(input: &str, start: (usize, usize, Direction)) -> u32 {
    let n = input.lines().next().unwrap().len();
    let mut cursors = vec![start];
    let mut result = vec![Visited::default(); n * n];
    while let Some((x, y, dir)) = cursors.pop() {
        let (x, y) = (x, y);
        let visited = &mut result[y * n + x];
        if visited.visit_dir(dir) {
            continue;
        }

        match (input.as_bytes()[y * (n + 1) + x], dir) {
            (b'.' | b'-', Direction::Right) | (b'/', Direction::Up) | (b'\\', Direction::Down)
                if x < n - 1 =>
            {
                cursors.push((x + 1, y, Direction::Right))
            }
            (b'.' | b'-', Direction::Left) | (b'/', Direction::Down) | (b'\\', Direction::Up)
                if x > 0 =>
            {
                cursors.push((x - 1, y, Direction::Left))
            }
            (b'.' | b'|', Direction::Down)
            | (b'/', Direction::Left)
            | (b'\\', Direction::Right)
                if y < n - 1 =>
            {
                cursors.push((x, y + 1, Direction::Down))
            }
            (b'.' | b'|', Direction::Up) | (b'/', Direction::Right) | (b'\\', Direction::Left)
                if y > 0 =>
            {
                cursors.push((x, y - 1, Direction::Up))
            }
            (b'-', Direction::Up | Direction::Down) => {
                if x > 0 {
                    cursors.push((x - 1, y, Direction::Left));
                }
                if x < n - 1 {
                    cursors.push((x + 1, y, Direction::Right));
                }
            }
            (b'|', Direction::Left | Direction::Right) => {
                if y > 0 {
                    cursors.push((x, y - 1, Direction::Up));
                }
                if y < n - 1 {
                    cursors.push((x, y + 1, Direction::Down));
                }
            }
            _ => (),
        }
    }

    result.into_iter().filter(Visited::is_energized).count() as u32
}

fn calculate_most_energized_tiles(input: &str) -> u32 {
    let n = input.lines().next().unwrap().len();
    (0..n)
        .map(|x| (x, 0, Direction::Down))
        .chain((0..n).map(|y| (0, y, Direction::Right)))
        .chain((0..n).map(|x| (x, n - 1, Direction::Up)))
        .chain((0..n).map(|y| (n - 1, y, Direction::Left)))
        .collect_vec()
        .into_par_iter()
        .map(|start| calculate_energy(input, start))
        .max()
        .unwrap()
}

fn main() {
    let res = solve(read_file_input("16.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u32) {
    let p1 = calculate_energy(&input, (0, 0, Direction::Right));
    let p2 = calculate_most_energized_tiles(&input);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("16_one.txt".to_string()));
        assert_eq!(res.0, 46);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("16_one.txt".to_string()));
        assert_eq!(res.1, 51);
    }
}
