use aoc::read_file_input;
use pathfinding::directed::bfs::bfs_reach;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let res = solve(read_file_input("25.txt".to_string()));

    println!("{}", res);
}

fn solve(input: String) -> usize {
    let mut graph = parse(&input);
    
    for _ in 0..3 {
        let connection = find_split_path(&graph);
        graph.get_mut(&connection.0).unwrap().remove(connection.1);
        graph.get_mut(&connection.1).unwrap().remove(connection.0);
    }

    let reached = bfs_reach(*graph.keys().next().unwrap(), |n| graph[n].iter().copied()).count();
    reached * (graph.len() - reached)
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut links: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (component_name, other_components) = line.split_once(": ").unwrap();

        for other_component in other_components.split_whitespace() {
            links
                .entry(component_name)
                .or_default()
                .insert(other_component);
            links
                .entry(other_component)
                .or_default()
                .insert(component_name);
        }
    }

    links
}

fn find_split_path<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();

    for start in graph.keys().copied() {
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some(node) = to_visit.pop_front() {
            for n in graph[&node].iter().copied() {
                if !visited.contains(&n) {
                    to_visit.push_back(n);
                    visited.insert(n);

                    let edge = if n < node { (n, node) } else { (node, n) };
                    *paths.entry(edge).or_default() += 1;
                }
            }
        }
    }

    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("25_one.txt".to_string()));
        assert_eq!(res, 54);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve(read_file_input("25.txt".to_string()));
        assert_eq!(res, 589036);
    }
}
