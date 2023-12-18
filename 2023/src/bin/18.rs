use aoc::read_file_input;

fn main() {
    let res = solve(read_file_input("18.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u64, u64) {
    let p1 = solve_part_one(&input);
    let p2 = solve_part_two(&input);

    (p1, p2)
}

fn solve_part_one(input: &str) -> u64 {
    let (_, visited, exterior) = input.lines().fold(
        ([0, 0], vec![[0, 0]], 0),
        |([x, y], mut visited, count), line| {
            let line = line.as_bytes();
            let direction = line[0];
            let mut steps = (line[2] - b'0') as i64;
            match line[3] {
                d @ b'0'..=b'9' => steps = steps * 10 + (d - b'0') as i64,
                b' ' => (),
                _ => (),
            };

            let [x, y] = match direction {
                b'R' => [x + steps, y], // Right
                b'D' => [x, y + steps], // Down
                b'L' => [x - steps, y], // Left
                b'U' => [x, y - steps], // Up
                _ => panic!("Unknown direction"),
            };
            visited.push([x, y]);

            ([x, y], visited, count + steps)
        },
    );

    calculate_area(&visited, exterior as _)
}

fn solve_part_two(input: &str) -> u64 {
    let (_, visited, exterior) = input.lines().fold(
        ([0, 0], vec![[0, 0]], 0),
        |([x, y], mut visited, count), line| {
            let line = line.as_bytes();
            let direction = line[line.len() - 2];
            let steps_hex = &line[line.len() - 7..line.len() - 2];
            let mut steps_hex_padded = [b'0'; 6];
            steps_hex_padded[1..].copy_from_slice(steps_hex);

            let mut steps_hex_decode = [0; 4];
            faster_hex::hex_decode(&steps_hex_padded, &mut steps_hex_decode[1..]).unwrap();
            let steps = u32::from_be_bytes(steps_hex_decode) as i64;

            let [x, y] = match direction {
                b'0' => [x + steps, y],
                b'1' => [x, y + steps],
                b'2' => [x - steps, y],
                b'3' => [x, y - steps],
                _ => panic!("Unknown direction"),
            };
            visited.push([x, y]);

            ([x, y], visited, count + steps)
        },
    );

    calculate_area(&visited, exterior as _)
}

fn calculate_area(visited: &[[i64; 2]], exterior: u64) -> u64 {
    let area = visited
        .iter()
        .zip(visited.iter().cycle().skip(1))
        .map(|(&[x1, y1], &[x2, y2])| (y1 + y2) * (x2 - x1))
        .sum::<i64>()
        .abs()
        / 2;

    area as u64 + 1 + exterior / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("18_one.txt".to_string()));
        assert_eq!(res.0, 62);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("18_one.txt".to_string()));
        assert_eq!(res.1, 952_408_144_115);
    }
}
