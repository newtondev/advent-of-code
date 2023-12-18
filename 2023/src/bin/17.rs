use aoc::read_file_input;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn get_diff(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct DistributionKey {
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

fn main() {
    let res = solve(read_file_input("17.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = shortest_path(&input, (0, 0), (input.len() - 1, input[0].len() - 1), false).unwrap();
    let p2 = shortest_path(&input, (0, 0), (input.len() - 1, input[0].len() - 1), true).unwrap();

    (p1, p2)
}

// Using Dijkstra's shortest path algorithm.
fn shortest_path(
    field: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
    use_ultra_crucible: bool,
) -> Option<usize> {
    let mut dist: HashMap<DistributionKey, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    dist.insert(
        DistributionKey {
            position: start,
            direction: Direction::Right,
            steps_direction: 0,
        },
        0,
    );
    dist.insert(
        DistributionKey {
            position: start,
            direction: Direction::Down,
            steps_direction: 0,
        },
        0,
    );
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::Right,
        steps_direction: 0,
    });

    while let Some(State {
        cost,
        position,
        direction,
        steps_direction,
    }) = heap.pop()
    {
        if position == goal {
            if !use_ultra_crucible || (use_ultra_crucible && steps_direction >= 4) {
                return Some(cost);
            }
        }

        let dist_key = DistributionKey {
            position,
            direction,
            steps_direction,
        };
        if dist.contains_key(&dist_key) && cost > dist[&dist_key] {
            continue;
        }

        // find a lower cost
        for dir in directions.iter().filter(|&d| *d != direction.opposite()) {
            let (diff_x, diff_y) = dir.get_diff();
            if position.0 as isize + diff_x < 0
                || position.1 as isize + diff_y < 0
                || position.0 as isize + diff_x >= field.len() as isize
                || position.1 as isize + diff_y >= field[0].len() as isize
            {
                continue;
            }

            let new_pos = (
                (position.0 as isize + diff_x) as usize,
                (position.1 as isize + diff_y) as usize,
            );

            let next = State {
                position: new_pos,
                direction: *dir,
                steps_direction: if *dir == direction {
                    steps_direction + 1
                } else {
                    1
                },
                cost: cost + field[new_pos.0][new_pos.1],
            };

            let dist_key = DistributionKey {
                position: new_pos,
                direction: *dir,
                steps_direction: next.steps_direction,
            };

            if use_ultra_crucible {
                if (direction == *dir || (steps_direction >= 4))
                    && next.steps_direction <= 10
                    && (!dist.contains_key(&dist_key) || next.cost < dist[&dist_key])
                {
                    heap.push(next);
                    dist.insert(dist_key, next.cost);
                    println!("next: {:?}", next.cost);
                }
            } else {
                if next.steps_direction <= 3
                    && (!dist.contains_key(&dist_key) || next.cost < dist[&dist_key])
                {
                    heap.push(next);
                    dist.insert(dist_key, next.cost);
                }
            }
        }
    }

    // not reachable.
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("17_one.txt".to_string()));
        assert_eq!(res.0, 102);
    }

    #[test]
    fn test_solve_two_a() {
        let res = solve(read_test_file_input("17_one.txt".to_string()));
        assert_eq!(res.1, 94);
    }

    #[test]
    fn test_solve_two_b() {
        let res = solve(read_test_file_input("17_two.txt".to_string()));
        assert_eq!(res.1, 71);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve(read_file_input("17.txt".to_string()));
        assert_eq!(res.0, 1155);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve(read_file_input("17.txt".to_string()));
        assert_eq!(res.1, 1286);
    }
}
