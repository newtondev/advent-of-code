use std::collections::{HashSet, VecDeque};

use aoc::read_file_input;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y,
            }, // Left
            Position {
                x: self.x + 1,
                y: self.y,
            }, // Right
            Position {
                x: self.x,
                y: self.y - 1,
            }, // Up
            Position {
                x: self.x,
                y: self.y + 1,
            }, // Down
        ]
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Quad {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

impl Quad {
    pub fn sum(&self) -> usize {
        self.top + self.right + self.bottom + self.left
    }
}

fn main() {
    let input = read_file_input("21.txt".to_string());

    let p1 = solve_part_one(&input, 64);
    let p2 = solve_part_two(&input);

    println!("{}", p1);
    println!("{}", p2);
}

fn parse(input: &str) -> (HashSet<Position>, Position) {
    let mut garden_plots = HashSet::new();
    let mut start_pos = Position { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    garden_plots.insert(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                }
                'S' => {
                    start_pos = Position {
                        x: x as isize,
                        y: y as isize,
                    };
                    garden_plots.insert(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                }
                '#' => {}
                _ => unimplemented!(),
            };
        }
    }

    (garden_plots, start_pos)
}

fn solve_part_one(input: &str, step_goal: usize) -> usize {
    let (garden_plots, start_pos) = parse(input);

    let mut visited = HashSet::new();
    let mut next: VecDeque<(Position, usize)> = VecDeque::new();
    let mut routes = HashSet::new();

    next.push_back((start_pos, 0));

    let bounds: Position = Position {
        x: garden_plots.iter().map(|p| p.x).max().unwrap(),
        y: garden_plots.iter().map(|p| p.y).max().unwrap(),
    };

    while let Some((pos, steps)) = next.pop_front() {
        if steps == step_goal {
            routes.insert(pos);
            continue;
        }

        if visited.contains(&(pos, steps)) {
            continue;
        }

        visited.insert((pos, steps));
        for neighbour in pos.neighbours() {
            if garden_plots.contains(&Position {
                x: neighbour.x.rem_euclid(bounds.x + 1),
                y: neighbour.y.rem_euclid(bounds.y + 1),
            }) {
                next.push_back((neighbour, steps + 1));
            }
        }
    }

    routes.len()
}

fn solve_part_two(input: &str) -> i64 {
    let mut start_pos = Position::zero();
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == 'S' {
                        start_pos = Position {
                            y: y as isize,
                            x: x as isize,
                        };
                        '.'
                    } else {
                        char
                    }
                })
                .collect()
        })
        .collect();

    let map_size = map.len();
    let grid_size = 26501365 / map_size - 1;

    let points_entirely_in_grid = {
        let even_maps = ((grid_size + 1) / 2 * 2).pow(2);
        let odd_maps = (grid_size / 2 * 2 + 1).pow(2);

        let odd_points = count_pos(&map, start_pos, map_size * 2 + 1);
        let even_points = count_pos(&map, start_pos, map_size * 2);

        (odd_points * odd_maps) + (even_points * even_maps)
    };

    let points_in_grid_corners = {
        Quad {
            top: count_pos(
                &map,
                Position {
                    y: (map_size - 1) as isize,
                    x: start_pos.x,
                },
                map_size - 1,
            ),
            right: count_pos(
                &map,
                Position {
                    y: start_pos.y,
                    x: 0,
                },
                map_size - 1,
            ),
            bottom: count_pos(
                &map,
                Position {
                    y: 0,
                    x: start_pos.x,
                },
                map_size - 1,
            ),
            left: count_pos(
                &map,
                Position {
                    y: start_pos.y,
                    x: (map_size - 1) as isize,
                },
                map_size - 1,
            ),
        }
        .sum()
    };

    let points_in_small = {
        let points = Quad {
            top: count_pos(
                &map,
                Position {
                    y: (map_size - 1) as isize,
                    x: 0,
                },
                map_size / 2 - 1,
            ), // top left
            right: count_pos(&map, Position::zero(), map_size / 2 - 1), // bottom right
            bottom: count_pos(
                &map,
                Position {
                    y: 0,
                    x: (map_size - 1) as isize,
                },
                map_size / 2 - 1,
            ), // bottom left
            left: count_pos(
                &map,
                Position {
                    y: (map_size - 1) as isize,
                    x: (map_size - 1) as isize,
                },
                map_size / 2 - 1,
            ), // top left
        };

        (grid_size + 1) * points.sum()
    };

    let points_in_big = {
        let points = Quad {
            top: count_pos(
                &map,
                Position {
                    y: (map_size - 1) as isize,
                    x: 0,
                },
                map_size * 3 / 2 - 1,
            ), // top right
            right: count_pos(&map, Position::zero(), map_size * 3 / 2 - 1), // bottom right
            bottom: count_pos(
                &map,
                Position {
                    y: 0,
                    x: (map_size - 1) as isize,
                },
                map_size * 3 / 2 - 1,
            ), // bottom left
            left: count_pos(
                &map,
                Position {
                    y: (map_size - 1) as isize,
                    x: (map_size - 1) as isize,
                },
                map_size * 3 / 2 - 1,
            ), // top left
        };

        grid_size * points.sum()
    };

    (points_entirely_in_grid + points_in_grid_corners + points_in_small + points_in_big) as i64
}

fn count_pos(map: &Vec<Vec<char>>, start_pos: Position, steps: usize) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    positions.insert(start_pos);

    for _ in 0..steps {
        let mut new_positions: HashSet<Position> = HashSet::new();
        for position in positions {
            let (y, x) = (position.x, position.y);
            if y > 0 && map[(y as usize) - 1][x as usize] == '.' {
                new_positions.insert(Position { y: y - 1, x });
            }
            if (y as usize) < map.len() - 1 && map[(y as usize) + 1][x as usize] == '.' {
                new_positions.insert(Position { y: y + 1, x });
            }
            if x > 0 && map[y as usize][(x as usize) - 1] == '.' {
                new_positions.insert(Position { y: y, x: x - 1 });
            }
            if (x as usize) < map[y as usize].len() - 1 && map[y as usize][(x as usize) + 1] == '.'
            {
                new_positions.insert(Position { y: y, x: x + 1 });
            }
        }
        positions = new_positions;
    }
    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one_a() {
        let res = solve_part_one(&read_test_file_input("21_one.txt".to_string()), 6);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_solve_one_b() {
        let res = solve_part_one(&read_test_file_input("21_one.txt".to_string()), 64);
        assert_eq!(res, 2665);
    }

    #[test]
    fn test_solve_one_c() {
        let res = solve_part_one(&read_test_file_input("21_one.txt".to_string()), 10);
        assert_eq!(res, 50);
    }

    #[test]
    fn test_solve_one_d() {
        let res = solve_part_one(&read_test_file_input("21_one.txt".to_string()), 50);
        assert_eq!(res, 1594);
    }

    #[test]
    fn test_solve_one_e() {
        let res = solve_part_one(&read_test_file_input("21_one.txt".to_string()), 100);
        assert_eq!(res, 6536);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve_part_one(&read_file_input("21.txt".to_string()), 64);
        assert_eq!(res, 3853);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve_part_two(&read_file_input(
            "21.txt".to_string(), /*/, 639_051_580_070_841 */
        ));
        assert_eq!(res, 639_051_580_070_841);
    }
}
