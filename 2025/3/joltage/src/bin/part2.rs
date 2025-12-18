use anyhow::{Context, Result, anyhow};

fn part2(joltages: &str) -> Result<u64> {
    let res: Result<Vec<u64>> = joltages
        .trim()
        .split("\n")
        .map(|x| get_largest_joltage(x))
        .collect();
    Ok(res.context("get joltages")?.iter().sum())
}

// Select 12 digits from the bank to form the maximum number
// Greedy approach: for each position, pick the maximum digit from the valid range
fn get_largest_joltage(bank: &str) -> Result<u64> {
    let chars: Vec<char> = bank.chars().collect();
    let n = chars.len();
    let mut result = String::new();
    let mut input_idx = 0;

    for result_pos in 0..12 {
        let remaining = 12 - result_pos - 1;  // digits still needed after this one
        let search_end = n - remaining;       // latest position we can pick from

        // Find max digit in range [input_idx, search_end)
        // When there are ties, prefer the first occurrence (earlier index)
        let (max_idx, max_digit) = chars[input_idx..search_end]
            .iter()
            .enumerate()
            .max_by(|(i1, c1), (i2, c2)| {
                match c1.cmp(c2) {
                    std::cmp::Ordering::Equal => i2.cmp(i1), // prefer earlier (reverse idx comparison)
                    other => other,
                }
            })
            .ok_or_else(|| anyhow!("No digit found in range"))?;

        result.push(*max_digit);
        input_idx += max_idx + 1;  // move past the digit we just picked
    }

    result.parse::<u64>().context("parse result")
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
        assert_eq!(get_largest_joltage(input)?, 987654321111);
        Ok(())
    }
    #[test]
    fn test_single_bank_2_part_2() -> Result<()> {
        let input = "811111111111119";
        assert_eq!(get_largest_joltage(input)?, 811111111119);
        Ok(())
    }
    #[test]
    fn test_single_bank_3_part_2() -> Result<()> {
        let input = "234234234234278";
        assert_eq!(get_largest_joltage(input)?, 434234234278);
        Ok(())
    }
    #[test]
    fn test_single_bank_4_part_2() -> Result<()> {
        let input = "818181911112111";
        assert_eq!(get_largest_joltage(input)?, 888911112111);
        Ok(())
    }
    #[test]
    fn test_part_2() -> Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part2(input).context("part2")?, 3121910778619);
        Ok(())
    }
}
