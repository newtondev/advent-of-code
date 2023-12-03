use aoc::read_file_input;

fn main() {
    let res = solve(read_file_input("02.txt".to_string()));

    println!("{}", res.0);
    println!("{}", res.1);
}

fn solve(input: String) -> (u32, u32) {
    let mut score1 = 0;
    let mut score2 = 0;

    for line in input.lines() {
        let cols: Vec<&str> = line.split(' ').collect();

        if cols[1] == "X" { // one: I should choose Rock | two: means I need to lose
            score1 += 1; // score for Rock is 1 point
            if cols[0] == "A" { // If opponent chooses Rock
                score1 += 3;
                score2 += 3;
            } else if cols[0] == "C" { // If opponent chooses Scissors
                score1 += 6;
                score2 += 2;
            } else { // If opponent chooses Paper
                score2 += 1;
            }
        } else if cols[1] == "Y" { // one: I should choose Paper | two: means I need to end the round in a draw
            score1 += 2; // score for Paper is 2 points
            if cols[0] == "B" { // If opponent chooses Paper
                score1 += 3;
                score2 += 2;
            } else if cols[0] == "A" { // If opponent chooses Rock
                score1 += 6;
                score2 += 1;
            } else { // If opponent chooses Scissors
                score2 += 3;
            }
            score2 += 3;
        } else { // Z -> one: I should choose Scissors | two: means I need to win
            score1 += 3; // score for Scissors is 3 points
            if cols[0] == "C" { // If opponent chooses Scissors
                score1 += 3;
                score2 += 1;
            } else if cols[0] == "B" { // If opponent chooses Paper
                score1 += 6;
                score2 += 3;
            } else { // If opponent chooses Rock
                score2 += 2;
            }
            score2 += 6;
        }
    }

    (score1, score2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::read_test_file_input;

    #[test]
    fn test_solve_one() {
        let res = solve(read_test_file_input("02_one.txt".to_string()));
        assert_eq!(
            res.0,
            15
        );
    }

    #[test]
    fn test_solve_two() {
        let res = solve(read_test_file_input("02_one.txt".to_string()));
        assert_eq!(
            res.1,
            12
        );
    }
}
