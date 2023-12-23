use aoc::read_file_input;
use itertools::Itertools;
use petgraph::{algo, prelude::*};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Heading {
    Forward,
    Backward,
    Both,
    Impassable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = read_file_input("23.txt".to_string());

    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    println!("{}", p1);
    println!("{}", p2);
}

fn solve_part_one(input: &str) -> usize {
    find_longest_hike(&input, false)
}

fn solve_part_two(input: &str) -> usize {
    find_longest_hike(&input, true)
}

fn find_longest_hike(input: &str, ignore_slopes: bool) -> usize {
    let mut graph = DiGraphMap::new();
    let row_count = input.lines().count();

    let mut queue = VecDeque::new();
    queue.push_back((
        [1, 1],           // Position
        [0, 1],           // Previous position
        1,                // Path length
        Heading::Both,    // Heading (is it directed?)
        Direction::Right, // Direction
    ));

    while let Some((position, mut previous, mut path_len, heading, direction)) = queue.pop_front() {
        let (x, y) = (position[0], position[1]);

        if position == [row_count - 2, row_count - 1] {
            graph.add_edge(previous, position, path_len);
            continue;
        }

        if graph.contains_node(position) {
            if ignore_slopes || matches!(heading, Heading::Both | Heading::Forward) {
                graph.add_edge(previous, position, path_len);
            }

            if ignore_slopes || matches!(heading, Heading::Both | Heading::Backward) {
                graph.add_edge(position, previous, path_len);
            }

            continue;
        }

        let mut next = Vec::with_capacity(3);
        let input = input.as_bytes();

        // Up
        if direction != Direction::Down && y > 0 && input[(y - 1) * (row_count + 1) + x] != b'#' {
            let heading = match input[(y - 1) * (row_count + 1) + x] {
                b'^' => Heading::Forward,
                b'v' => Heading::Backward,
                _ => heading,
            };
            next.push(([x, y - 1], heading, Direction::Up));
        }

        // Down
        if direction != Direction::Up && input[(y + 1) * (row_count + 1) + x] != b'#' {
            let heading = match input[(y + 1) * (row_count + 1) + x] {
                b'^' => Heading::Backward,
                b'v' => Heading::Forward,
                _ => heading,
            };
            next.push(([x, y + 1], heading, Direction::Down));
        }

        // Left
        if direction != Direction::Right && input[y * (row_count + 1) + x - 1] != b'#' {
            let heading = match input[y * (row_count + 1) + x - 1] {
                b'<' => Heading::Forward,
                b'>' => Heading::Backward,
                _ => heading,
            };
            next.push(([x - 1, y], heading, Direction::Left));
        }

        // Right
        if direction != Direction::Left && input[y * (row_count + 1) + x + 1] != b'#' {
            let heading = match input[y * (row_count + 1) + x + 1] {
                b'<' => Heading::Backward,
                b'>' => Heading::Forward,
                _ => heading,
            };
            next.push(([x + 1, y], heading, Direction::Right));
        }

        // Junction point
        if next.len() >= 2 && position != [1, 1] {
            if ignore_slopes || matches!(heading, Heading::Both | Heading::Forward) {
                graph.add_edge(previous, position, path_len);
            }

            if ignore_slopes || matches!(heading, Heading::Both | Heading::Backward) {
                graph.add_edge(position, previous, path_len);
            }

            path_len = 0;
            previous = position;
        }

        // increment the path length
        path_len += 1;

        for (position, mut is_directed, direction) in next {
            if heading == Heading::Impassable || heading != Heading::Both && is_directed != heading
            {
                is_directed = Heading::Impassable;
            }
            queue.push_back((position, previous, path_len, is_directed, direction));
        }
    }

    // Calculate all the paths
    algo::all_simple_paths(
        &graph,                         // Graph
        [0, 1],                         // From
        [row_count - 2, row_count - 1], // To
        0,                              // Minimum intermediate nodes
        None,                           // Maximum intermediate nodes
    )
    .map(|x: Vec<_>| {
        x.iter()
            .tuple_windows()
            .map(|(a, b)| graph.edge_weight(*a, *b).unwrap())
            .sum::<usize>()
    })
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve_part_one(&read_test_file_input("23_one.txt".to_string()));
        assert_eq!(res, 94);
    }

    #[test]
    fn test_solve_two() {
        let res = solve_part_two(&read_test_file_input("23_one.txt".to_string()));
        assert_eq!(res, 154);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve_part_one(&read_file_input("23.txt".to_string()));
        assert_eq!(res, 2222);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve_part_two(&read_file_input("23.txt".to_string()));
        assert_eq!(res, 6590);
    }
}
