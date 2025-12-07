use anyhow::{Context, Result};
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::digit1, sequence::separated_pair,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn parse_number(input: &str) -> IResult<&str, u64> {
    digit1.map_res(|s: &str| s.parse::<u64>()).parse(input)
}

// parses x-y
fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(parse_number, tag("-"), parse_number)
        .map(|(start, end)| start..=end)
        .parse(input)
}

// Parse from string (for tests)
fn parse_data(input: &str) -> Result<Vec<RangeInclusive<u64>>> {
    let mut lines = input.lines();

    // Parse ranges section
    let mut ranges = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break; // Hit the blank line separator
        }
        let (_, range) =
            parse_range(line).map_err(|e| anyhow::anyhow!("Parse range error: {}", e))?;
        ranges.push(range);
    }

    Ok(ranges)
}

// Parse file line by line for memory efficiency
fn parse_file_streaming(path: &str) -> Result<Vec<RangeInclusive<u64>>> {
    let file = File::open(path).context("open file")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse ranges section
    let mut ranges = Vec::new();
    for line in &mut lines {
        let line = line.context("read line")?;
        if line.trim().is_empty() {
            break; // Hit the blank line separator
        }
        let (_, range) =
            parse_range(&line).map_err(|e| anyhow::anyhow!("Parse range error: {}", e))?;
        ranges.push(range);
    }
    Ok(ranges)
}

struct RangeSet {
    ranges: Vec<RangeInclusive<u64>>,
}

impl RangeSet {
    pub fn build(ranges: Vec<RangeInclusive<u64>>) -> Self {
        Self { ranges }
    }

    pub fn num_present(&self) -> usize {
        self.ranges.clone().into_iter().flatten().count()
    }
}

fn part2(input_data: &str) -> Result<usize> {
    let ranges = parse_data(input_data).context("parse input data")?;

    let range_set = RangeSet::build(ranges);

    let res = range_set.num_present();

    Ok(res)
}

fn main() -> Result<()> {
    // get path to input
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or(anyhow::anyhow!("No input file path provided"))?;

    // Parse file line by line for memory efficiency
    let ranges = parse_file_streaming(&fp).context("parse file")?;

    let range_set = RangeSet::build(ranges);
    let num_present = range_set.num_present();

    println!("res: {}", num_present);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_2_example() -> Result<()> {
        let input = "3-5
10-14
16-20
12-18";
        assert_eq!(part2(input).context("part 2")?, 14);
        Ok(())
    }
}
