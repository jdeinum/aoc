// The initial thought for part 2, is that we can't actually materialize the grid because ;a

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read input")?;

    let res = part1(&s).context("part 1")?;

    println!("res: {res}");

    Ok(())
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    input.trim().split("\n").for_each(|x| {
        let (num1, num2) = x.split_once(",").unwrap();
        let num1 = num1.parse::<usize>().unwrap();
        let num2 = num2.parse::<usize>().unwrap();
        res.push((num1, num2));
    });
    res
}

fn part1(input: &str) -> Result<usize> {
    // parse the input
    let points = parse_input(input);

    println!("points: {points:?}");

    let res = points
        .iter()
        .tuple_combinations()
        .map(|(x, y)| (x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1))
        .max()
        .unwrap();

    Ok(res)
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_part1_example() -> Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part1(input).context("part 1")?, 50);
        Ok(())
    }
}
