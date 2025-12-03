use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

fn part2(joltages: &str) -> Result<u64> {
    let res: Result<Vec<u64>> = joltages
        .trim()
        .split("\n")
        .map(|x| get_largest_joltage(x))
        .collect();
    Ok(res.context("get joltages")?.iter().sum())
}

// The largest number should come first, but it cannot be the last number
// If there are multiple max values, we pick the first to make the window for the second value to
// be the largest
//
// The second number is the largest of (index_of_largest..]
fn get_largest_joltage(bank: &str) -> Result<u64> {
    // find the index of the maximum element
    // we reverse the iterator so that if there are multiple, we pick the earliest one
    // then we convert back to our standard index format
    //
    //
    // 1235489
    // 9845321
    // finds 8 at index 1, need to convert to index 5 (string length 6)
    let index_of_max = {
        let i = bank
            .chars()
            .rev()
            .skip(1)
            .position_max()
            .ok_or_else(|| anyhow!("No max element in iterator: {bank}"))?;
        println!("index in rev: {i}");
        bank.len() - i - 2 // 1 because we skip, 1 because start counting from 0
    };

    println!("found index of max at {index_of_max}");

    // now that we have the index of the max, we scan the rest of the elements for the next largest
    let index_of_second_max = bank
        .chars()
        .skip(index_of_max + 1)
        .position_max()
        .ok_or_else(|| anyhow!("couldn't find max element in subset: {bank}"))?;

    println!("found index of second max at {index_of_second_max}");

    let put_together = format!(
        "{}{}",
        bank.chars().nth(index_of_max).unwrap(),
        bank.chars()
            .nth(index_of_second_max + index_of_max + 1)
            .unwrap()
    );

    println!("put together: {put_together}");

    put_together
        .parse::<u64>()
        .context("convert string to number")
}

fn main() -> Result<()> {
    let fp = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow::anyhow!("No file path provided"))?;

    let s = std::fs::read_to_string(&fp).context("read file to string")?;

    let res = part2(&s).context("run part 2")?;

    println!("res: {res}");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Context;

    #[test]
    fn test_single_bank_1_part_2() -> Result<()> {
        let input = "987654321111111";
        assert_eq!(get_largest_joltage(input)?, 98);
        Ok(())
    }
    #[test]
    fn test_single_bank_2_part_2() -> Result<()> {
        let input = "811111111111119";
        assert_eq!(get_largest_joltage(input)?, 89);
        Ok(())
    }
    #[test]
    fn test_single_bank_3_part_2() -> Result<()> {
        let input = "234234234234278";
        assert_eq!(get_largest_joltage(input)?, 78);
        Ok(())
    }
    #[test]
    fn test_single_bank_4_part_2() -> Result<()> {
        let input = "818181911112111";
        assert_eq!(get_largest_joltage(input)?, 92);
        Ok(())
    }
    #[test]
    fn test_part_2() -> Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part2(input).context("part1")?, 357);
        Ok(())
    }
}
