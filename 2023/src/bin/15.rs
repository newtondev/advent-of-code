#![feature(linked_list_cursors)]
use std::{collections::LinkedList, u8};

use aoc::read_file_input;

fn main() {
    let res = solve(read_file_input("15.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u32) {
    let p1 = input
        .split(",")
        .map(|ch| hash(ch.as_bytes()))
        .map(u32::from)
        .sum();

    let p2 = calculate_total_focusing_power(&input);

    (p1, p2)
}

fn hash(ch: &[u8]) -> u8 {
    ch.iter().fold(0, |acc, &ch| {
        acc.overflowing_add(ch).0.overflowing_mul(17).0
    })
}

// Calculate the total focusing power of all the lenses in all the boxes.
fn calculate_total_focusing_power(input: &str) -> u32 {
    let mut boxes = vec![LinkedList::new(); 256];

    // Pack the lenses into the boxes.
    input.split(",").for_each(|ch| match ch.as_bytes() {
        [box_name @ .., b'=', value @ b'0'..=b'9'] => {
            let box_id = hash(box_name) as usize;
            let box_value = value - b'0';

            for (name, value) in boxes[box_id].iter_mut() {
                if name == &box_name {
                    *value = box_value;
                    return;
                }
            }
            boxes[box_id].push_back((box_name, box_value))
        }
        [box_name @ .., b'-'] => {
            let box_id = hash(box_name) as usize;
            let mut cursor = boxes[box_id].cursor_front_mut();
            while let Some((name, _)) = cursor.current() {
                if name == &box_name {
                    cursor.remove_current();
                    break;
                }
                cursor.move_next();
            }
        }
        _ => unreachable!(),
    });

    // Sum up the total focusing power of all the lenses in all the boxes.
    boxes
        .into_iter()
        .zip(1..)
        .map(|(list, box_num)| {
            // Calculate the focusing power of each box of lenses.
            box_num
                * list
                    .into_iter()
                    .zip(1..)
                    .map(|((_, value), slot)| value as u32 * slot)
                    .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("15_one.txt".to_string()));
        assert_eq!(res.0, 1320);
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("15_one.txt".to_string()));
        assert_eq!(res.1, 145);
    }
}
