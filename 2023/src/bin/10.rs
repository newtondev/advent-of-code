use aoc::read_file_input;
use pathfinding::matrix::Matrix;
use std::iter::{from_fn, once};

macro_rules! check {
    {$matrix:expr, $pos:expr, $bit:expr, $($checks:expr),*} => {
        if matches!($matrix.get($pos), Some($($checks)|*)) { 1 << $bit } else { 0 }
    }
}

fn main() {
    let res = solve(read_file_input("10.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    (p1, p2)
}

fn parse(input: &str) -> ((usize, usize), Matrix<u8>) {
    let matrix = Matrix::from_rows(input.lines().map(|s| s.bytes())).unwrap();
    let mut padded_matrix = Matrix::new(matrix.rows + 2, matrix.columns + 2, b'.');
    padded_matrix.set_slice((1, 1), &matrix);

    for (row, col) in padded_matrix.keys() {
        if padded_matrix[(row, col)] == b'S' {
            let bend = check!(padded_matrix, (row - 1, col), 3, b'|', b'F', b'7') // left
                | check!(padded_matrix, (row + 1, col), 2, b'|', b'L', b'J') // right
                | check!(padded_matrix, (row, col + 1), 1, b'-', b'7', b'J') // down
                | check!(padded_matrix, (row, col - 1), 0, b'-', b'L', b'F'); // up
            let actual = b"XXX-X7FXXJLX|XXX"[bend];
            padded_matrix[(row, col)] = actual;
            return ((row, col), padded_matrix);
        }
    }
    panic!("Cannot parse input")
}

fn walk(start: (usize, usize), grid: &Matrix<u8>) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut position = start;
    let mut direction = match grid[start] {
        b'|' | b'F' | b'7' => (1, 0), // right
        b'J' | b'L' => (-1, 0),       // left
        _ => (0, 1),                  // down
    };
    once(start).chain(from_fn(move || {
        position = grid.move_in_direction(position, direction).unwrap();
        (position != start).then(|| {
            direction = match (direction, grid[position]) {
                ((1, 0), b'L') | ((-1, 0), b'F') => (0, 1),  // down
                ((1, 0), b'J') | ((-1, 0), b'7') => (0, -1), // up
                ((0, 1), b'7') | ((0, -1), b'F') => (1, 0),  // right
                ((0, 1), b'J') | ((0, -1), b'L') => (-1, 0), // left
                _ => direction,
            };
            position
        })
    }))
}

fn solve_part_one(input: &str) -> usize {
    let (start, grid) = parse(input);
    walk(start, &grid).count() / 2
}

fn solve_part_two(input: &str) -> usize {
    let (start, grid) = parse(input);
    let mut blockers = Matrix::new(grid.rows, grid.columns, b'.');
    for position in walk(start, &grid) {
        blockers[position] = grid[position];
    }
    let mut inside = false;
    blockers
        .items()
        .filter(|&((_, col), &i)| {
            inside &= col != 0;
            inside ^= matches!(i, b'|' | b'J' | b'L');
            inside && i == b'.'
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one_a() {
        let res = solve_part_one(&read_test_file_input("10_one_a.txt".to_string()));
        assert_eq!(res, 4);
    }

    #[test]
    fn test_solve_one_b() {
        let res = solve_part_one(&read_test_file_input("10_one_b.txt".to_string()));
        assert_eq!(res, 8);
    }

    #[test]
    fn test_solve_two_a() {
        let res = solve_part_two(&read_test_file_input("10_two_a.txt".to_string()));
        assert_eq!(res, 4);
    }

    #[test]
    fn test_solve_two_b() {
        let res = solve_part_two(&read_test_file_input("10_two_b.txt".to_string()));
        assert_eq!(res, 4);
    }

    #[test]
    fn test_solve_two_c() {
        let res = solve_part_two(&read_test_file_input("10_two_c.txt".to_string()));
        assert_eq!(res, 10);
    }
}
