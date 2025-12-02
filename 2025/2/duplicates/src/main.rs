// Something I unsure about is whether the numbers can be represented as numbers, or if I need to
// make them strings, depends entirely if they are smaller than u64::max

use anyhow::Result;

fn main() -> Result<()> {
    todo!()
}

fn part1(ranges: &str) -> Result<Vec<String>> {
    todo!()
}

fn parse_id_ranges(ranges: &str) -> Result<Vec<IdRange>> {
    let raw_ranges: Result<Vec<IdRange>> =
        ranges.split(",").map(|x| IdRange::try_from(x)).collect();
    raw_ranges
}

struct IdRange {
    starting_number: String,
    ending_number: String,
}

impl TryFrom<&str> for IdRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (starting_number, ending_number) = value
            .split_once("-")
            .ok_or_else(|| anyhow::Error::msg("Range must contain -"))?;

        Ok(Self {
            starting_number: starting_number.to_string(),
            ending_number: ending_number.to_string(),
        })
    }
}

impl IdRange {
    pub fn find_invalid(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
