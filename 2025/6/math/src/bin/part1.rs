// Initial ideas:
//
// Using iterators with skip and step can leave us with only the values of interest, and we should
// be able to reverse that iterator of strings, and match on the operation, and use filter with +
// and a base of 0 or multiplied with a base of 1. To do things this way, we need to find out the
// total number of problems (i.e columns).

use anyhow::{Context, Result};

enum Operation {
    Sum,
    Multiply,
}

struct Problem {
    operation: Operation,
    values: Vec<u64>,
}

impl Problem {
    fn solve(self) -> u64 {
        match self.operation {
            Operation::Sum => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().fold(1, |acc, &e| acc * e),
        }
    }
}

pub fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part1(&s).context("part 1")?;

    println!("res: {res}");

    Ok(())
}

pub fn part1(input: &str) -> Result<u64> {
    let problems = parse_input(input).context("parse input")?;

    let res = problems.into_iter().map(|x| x.solve()).sum();

    Ok(res)
}

fn parse_input(input: &str) -> Result<Vec<Problem>> {
    // first we calculate the number of columns by checking the first row and how many numbers it
    let num_columns = input
        .trim()
        .split("\n")
        .next()
        .ok_or(anyhow::anyhow!("No rows"))?
        .split_whitespace()
        .count();

    // now we create the single iterator of values
    let data: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    // now we can create our problems
    let mut res: Vec<Problem> = Vec::new();
    (0..num_columns).into_iter().for_each(|x| {
        let all_strs: Vec<&str> = data
            .iter()
            .skip(x)
            .step_by(num_columns)
            .map(|x| x.as_str())
            .rev()
            .collect();

        let operation = match all_strs.iter().next() {
            Some(&"+") => Operation::Sum,
            Some(&"*") => Operation::Multiply,
            x => {
                panic!("Got unexpected operation {x:?}");
            }
        };

        let values: Vec<u64> = all_strs
            .iter()
            .skip(1)
            .map(|x| x.parse::<u64>().expect("parse str into u64"))
            .collect();

        res.push(Problem { operation, values });
    });

    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_part1_exmaple() -> Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part1(input).context("part 1")?, 4277556);
        Ok(())
    }
}
