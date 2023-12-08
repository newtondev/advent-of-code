use std::collections::HashMap;

use aoc::read_file_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Default)]
struct Node<'a> {
    id: &'a str,
    left_id: &'a str,
    right_id: &'a str,
    left_idx: usize,
    right_idx: usize,
}

fn main() {
    let res = solve(read_file_input("08.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (usize, usize) {
    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    (p1, p2)
}

fn solve_part_one<'a>(input: &'a str) -> usize {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut instructions_iter = instructions_str.chars().cycle();
    let mut nodes: Vec<Node> = Vec::new();
    let mut indices: HashMap<&'a str, usize> = HashMap::new();
    let mut iterations = 0;
    let mut current = 0;

    for line in nodes_str.lines() {
        let idx = nodes.len();
        let id = &line[0..3];
        let left_id = &line[7..10];
        let right_id = &line[12..15];

        nodes.push(Node {
            id,
            left_id,
            right_id,
            left_idx: 0,
            right_idx: 0,
        });
        indices.insert(id, idx);

        if id == "AAA" {
            current = idx;
        }
    }

    for node in nodes.iter_mut() {
        node.left_idx = *indices.get(node.left_id).unwrap();
        node.right_idx = *indices.get(node.right_id).unwrap();
    }

    while nodes[current].id != "ZZZ" {
        iterations += 1;

        let instruction = instructions_iter.next().unwrap();

        current = if instruction == 'L' {
            nodes[current].left_idx
        } else {
            nodes[current].right_idx
        };
    }

    iterations
}

fn solve_part_two<'a>(input: &'a str) -> usize {
    let (instructions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let mut nodes: Vec<Node> = Vec::new();
    let mut starting_node_indexes: Vec<usize> = Vec::new();
    let mut indices: HashMap<&'a str, usize> = HashMap::new();

    for line in nodes_str.lines() {
        let idx = nodes.len();
        let id = &line[0..3];
        let left_id = &line[7..10];
        let right_id = &line[12..15];

        nodes.push(Node {
            id,
            left_id,
            right_id,
            left_idx: 0,
            right_idx: 0,
        });
        indices.insert(id, idx);

        if &id[2..] == "A" {
            starting_node_indexes.push(idx);
        }
    }

    for node in nodes.iter_mut() {
        node.left_idx = *indices.get(node.left_id).unwrap();
        node.right_idx = *indices.get(node.right_id).unwrap();
    }

    starting_node_indexes
        .par_iter()
        .map(|&idx| {
            let mut instructions_iter = instructions_str.chars().cycle();
            let mut cycle_start = 0;
            let mut cycle_end = 0;

            let mut current = idx;

            loop {
                cycle_end += 1;
                let instruction = instructions_iter.next().unwrap();

                current = if instruction == 'L' {
                    nodes[current].left_idx
                } else {
                    nodes[current].right_idx
                };

                let last_char = *nodes[current].id.as_bytes().last().unwrap() as char;

                if last_char == 'Z' {
                    if cycle_start == 0 {
                        cycle_start = cycle_end;
                    } else {
                        return cycle_end - cycle_start;
                    }
                }
            }
        })
        .reduce_with(|a, b| lcm(a, b))
        .unwrap()
}

// greatest common divisor
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

// least common multiple
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one_a() {
        let res = solve_part_one(&read_test_file_input("08_one_a.txt".to_string()));
        assert_eq!(
            res,
            2,
        );
    }

    #[test]
    fn test_solve_one_b() {
        let res = solve_part_one(&read_test_file_input("08_one_b.txt".to_string()));
        assert_eq!(
            res,
            6,
        );
    }

    #[test]
    fn test_solve_two() {
        let res = solve_part_two(&read_test_file_input("08_two.txt".to_string()));
        assert_eq!(
            res,
            6,
        );
    }
}
