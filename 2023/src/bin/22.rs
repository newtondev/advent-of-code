use std::collections::{HashMap, HashSet};

use aoc::read_file_input;

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone)]
struct Brick {
    start: Position,
    end: Position,
}

fn main() {
    let res = solve(read_file_input("22.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    (p1, p2)
}

fn solve_part_one(input: &str) -> usize {
    let mut bricks = parse(input);
    let mut supports = HashMap::new();
    fall_into_place(&mut bricks, Some(&mut supports));

    // calculate disintegration
    (0..bricks.len())
        .filter(|brick| {
            !supports
                .values()
                .any(|support| support.len() == 1 && support.contains(brick))
        })
        .count()
}

fn solve_part_two(input: &str) -> usize {
    let mut bricks = parse(input);
    fall_into_place(&mut bricks, None);

    // calculate number of bricks that would fall
    (0..bricks.len())
        .map(|brick_index| {
            let mut bricks = bricks.clone();
            bricks.remove(brick_index);
            fall_into_place(&mut bricks, None)
        })
        .sum()
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split('~').map(|pos| {
                let mut pos = pos.split(',').map(|coord| coord.parse().unwrap()); // x,y,z

                Position {
                    x: pos.next().unwrap(),
                    y: pos.next().unwrap(),
                    z: pos.next().unwrap(),
                }
            });

            Brick {
                start: line.next().unwrap(),
                end: line.next().unwrap(),
            }
        })
        .collect::<Vec<Brick>>()
}

fn intersects((min1, max1): (usize, usize), (min2, max2): (usize, usize)) -> bool {
    (min1..=max1).contains(&min2)
        || (min1..=max1).contains(&max2)
        || (min2..=max2).contains(&min1)
        || (min2..=max2).contains(&max1)
}

fn fall_into_place(
    bricks: &mut [Brick],
    mut supports: Option<&mut HashMap<usize, Vec<usize>>>,
) -> usize {
    bricks.sort_unstable_by_key(|brick| brick.start.z);

    let mut in_place: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut count = HashSet::new();
    for i in 0..bricks.len() {
        while bricks[i].start.z > 1 {
            let mut falling = true;

            let start_x = bricks[i].start.x;
            let start_y = bricks[i].start.y;
            let start_z = bricks[i].start.z;

            let end_x = bricks[i].end.x;
            let end_y = bricks[i].end.y;

            for &j in in_place.get(&(start_z - 1)).unwrap_or(&vec![]) {
                let next_start_x = bricks[j].start.x;
                let next_start_y = bricks[j].start.y;

                let next_end_x = bricks[j].end.x;
                let next_end_y = bricks[j].end.y;

                if intersects((start_x, end_x), (next_start_x, next_end_x))
                    && intersects((start_y, end_y), (next_start_y, next_end_y))
                {
                    if let Some(support) = &mut supports {
                        support.entry(i).or_default().push(j);
                    }
                    falling = false;
                }
            }

            if falling {
                let start_z = &mut bricks[i].start.z;
                let end_z = &mut bricks[i].end.z;
                *start_z -= 1;
                *end_z -= 1;
                count.insert(i);
            } else {
                break;
            }
        }
        in_place.entry(bricks[i].end.z).or_default().push(i);
    }
    count.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("22_one.txt".to_string()));
        assert_eq!(res.0, 5);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("22_one.txt".to_string()));
        assert_eq!(res.1, 7);
    }
}
