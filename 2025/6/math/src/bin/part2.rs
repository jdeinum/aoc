// After trying this, I realized that the alignment in this case is actually important. This means
// we need to change how we are parsing the values, here are the rules I think I'll start with:
//
// Each line contains a 0-X spaces, followed by 1-Y digits, followed by 1 space as a
// separator, followed by 0 or more spaces and 1 or more digits N times. Obviously we'll use nom
// for this, since it can be broken down into multiple steps easily. One note of interest is that
// other than the seperator spaces, we want to capture all of the spaces so we can convert them to
// zeroes.
//
// Ended up ditching nom, a character grid is easier

use anyhow::{Context, Result};

#[derive(Debug)]
enum Operation {
    Sum,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    operation: Operation,
    values: Vec<String>, // string to preserve leading 0s
}

impl Problem {
    fn solve(self) -> u64 {
        match self.operation {
            Operation::Sum => self
                .values
                .iter()
                .map(|x| x.parse::<u64>().expect("parse u64"))
                .sum(),
            Operation::Multiply => self
                .values
                .iter()
                .map(|x| x.parse::<u64>().expect("parse u64"))
                .fold(1, |acc, e| acc * e),
        }
    }
}

pub fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part2(&s).context("part 1")?;

    println!("res: {res}");

    Ok(())
}

pub fn part2(input: &str) -> Result<u64> {
    let problems = parse_input(input).context("parse input")?;

    let res = problems.into_iter().map(|x| x.solve()).sum();

    Ok(res)
}

fn extract_problems(char_matrix: Vec<Vec<char>>) -> Result<Vec<Problem>> {
    let num_rows = char_matrix.len();
    let num_cols = char_matrix[0].len();

    let operator_row = &char_matrix[num_rows - 1];
    let value_rows = &char_matrix[0..num_rows - 1];

    let mut problems = Vec::new();
    let mut current_values = Vec::new();
    let mut current_op = None;

    for col_idx in (0..num_cols).rev() {
        let col_chars: String = value_rows
            .iter()
            .filter_map(|row| {
                let ch = row[col_idx];
                if ch != ' ' { Some(ch) } else { None }
            })
            .collect();

        let op_char = operator_row[col_idx];

        if col_chars.is_empty() && op_char == ' ' {
            if !current_values.is_empty() && current_op.is_some() {
                problems.push(Problem {
                    operation: current_op.take().unwrap(),
                    values: current_values.clone(),
                });
                current_values.clear();
            }
        } else {
            if !col_chars.is_empty() {
                current_values.push(col_chars);
            }

            // Update operator
            match op_char {
                '+' => current_op = Some(Operation::Sum),
                '*' => current_op = Some(Operation::Multiply),
                _ => {}
            }
        }
    }

    // cant't forget last problem
    if !current_values.is_empty() && current_op.is_some() {
        problems.push(Problem {
            operation: current_op.unwrap(),
            values: current_values,
        });
    }

    Ok(problems)
}

fn parse_input(input: &str) -> Result<Vec<Problem>> {
    // Split into lines
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Err(anyhow::anyhow!("No lines in input"));
    }

    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let char_grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    extract_problems(char_grid)
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_part2_example() -> Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part2(input).context("part 1")?, 3263827);
        Ok(())
    }
}
