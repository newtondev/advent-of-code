// Part 1 was straight forward. Part2 was hard and I got a hint about z3 solver on reddit.
// Had to `sudo apt-get install clang cmake` to get the z3 SMT solver to work.
// See: https://crates.io/crates/z3
// See: https://github.com/Z3Prover/z3
// See: https://avigad.github.io/lamr/using_smt_solvers.html

use aoc::read_file_input;
use itertools::Itertools;
use z3::ast::{Ast, Int};

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Copy, Clone)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn new(px: i64, py: i64, pz: i64, vx: i64, vy: i64, vz: i64) -> Hailstone {
        Hailstone {
            pos: Vec3 {
                x: px,
                y: py,
                z: pz,
            },
            vel: Vec3 {
                x: vx,
                y: vy,
                z: vz,
            },
        }
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let line = line.split('@').collect::<Vec<&str>>();
            let pos: Vec<i64> = line[0]
                .split(',')
                .map(|p| p.trim().parse().unwrap())
                .collect();
            let vel: Vec<i64> = line[1]
                .split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect();
            Hailstone::new(pos[0], pos[1], pos[2], vel[0], vel[1], vel[2])
        })
        .collect()
}

fn solve_part_one(input: &str, range_start: f64, range_end: f64) -> usize {
    parse(&input)
        .into_iter()
        .tuple_combinations()
        .filter(|&(a, b)| {
            let ((x1, y1), (vx1, vy1)) = ((a.pos.x, a.pos.y), (a.vel.x, a.vel.y));
            let ((x2, y2), (vx2, vy2)) = ((b.pos.x, b.pos.y), (b.vel.x, b.vel.y));

            // calculate determinant
            let d = (vx1 as f64).mul_add(vy2 as f64, -(vy1 as f64 * vx2 as f64));
            // check for parallel lines, if close to zero then lines are nearly parallel and do not intersect
            if d.abs() <= f64::EPSILON {
                return false;
            }

            // poisition of the intersection point along the first line segment
            let t = (x2 as f64 - x1 as f64)
                .mul_add(vy2 as f64, -((y2 as f64 - y1 as f64) * vx2 as f64))
                / d;

            // position of the intersection point along the second line segment
            let u = (x2 as f64 - x1 as f64)
                .mul_add(vy1 as f64, -((y2 as f64 - y1 as f64) * vx1 as f64))
                / d;
            if t < 0.0 || u < 0.0 {
                // intersection point lies outside the line segments
                return false;
            }

            let x = (t as f64).mul_add(vx1 as f64, x1 as f64);
            let y = (t as f64).mul_add(vy1 as f64, y1 as f64);

            // Check for intersection points within the range
            (range_start..=range_end).contains(&x) && (range_start..=range_end).contains(&y)
        })
        .count()
}

fn solve_part_two(input: &str) -> i64 {
    let hailstones = parse(&input);

    let ctx = z3::Context::new(&z3::Config::default());
    let solver = z3::Solver::new(&ctx);

    let (x, y, z, vx, vy, vz) = (
        Int::new_const(&ctx, "x"),
        Int::new_const(&ctx, "y"),
        Int::new_const(&ctx, "z"),
        Int::new_const(&ctx, "vx"),
        Int::new_const(&ctx, "vy"),
        Int::new_const(&ctx, "vz"),
    );

    for hailstone in hailstones {
        let (pxn, pyn, pzn) = (
            Int::from_i64(&ctx, hailstone.pos.x),
            Int::from_i64(&ctx, hailstone.pos.y),
            Int::from_i64(&ctx, hailstone.pos.z),
        );

        let (vxn, vyn, vzn) = (
            Int::from_i64(&ctx, hailstone.vel.x),
            Int::from_i64(&ctx, hailstone.vel.y),
            Int::from_i64(&ctx, hailstone.vel.z),
        );

        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&x + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&y + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&z + &vz * &tn)));
    }

    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    let out = model.eval(&(x + y + z), true).unwrap();
    out.as_i64().unwrap()
}

fn main() {
    let input = read_file_input("24.txt".to_string());

    let p1 = solve_part_one(&input, 200000000000000.0, 400000000000000.0);
    let p2 = solve_part_two(&input);

    println!("{}", p1);
    println!("{}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve_part_one(&read_test_file_input("24_one.txt".to_string()), 7.0, 27.0);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_solve_two() {
        let res = solve_part_two(&read_test_file_input("24_one.txt".to_string()));
        assert_eq!(res, 47);
    }

    #[test]
    fn actual_solve_one() {
        let res = solve_part_one(
            &read_file_input("24.txt".to_string()),
            200_000_000_000_000.0,
            400_000_000_000_000.0,
        );
        assert_eq!(res, 12_740);
    }

    #[test]
    fn actual_solve_two() {
        let res = solve_part_two(&read_file_input("24.txt".to_string()));
        assert_eq!(res, 741_991_571_910_536);
    }
}
