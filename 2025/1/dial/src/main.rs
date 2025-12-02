use anyhow::{Context, Result, anyhow};
use std::io::Read;

fn main() -> Result<()> {
    // expect path
    let path = std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow!("No path provided"))?;

    // open file
    let mut f = std::fs::File::open(&path).context("open path to file")?;

    // read all of the lines
    let mut buf: String = String::new();
    f.read_to_string(&mut buf)
        .context("read bytes into string")?;

    let zeros = part2(&buf).context("run program")?;

    println!("hit zero {zeros} times");

    Ok(())
}

fn part1(rotations_str: &str) -> Result<usize> {
    let rotations = parse_rotations(rotations_str).context("parse rotations")?;
    count_times_ended_on_zero(&rotations)
}

fn part2(rotations_str: &str) -> Result<usize> {
    let rotations = parse_rotations(rotations_str).context("parse rotations")?;
    count_times_hit_zero(&rotations)
}

fn parse_rotations(rotations_str: &str) -> Result<Vec<i64>> {
    let rotations: Result<Vec<i64>> = rotations_str
        .split("\n")
        .filter(|x| x.len() > 1) // if the length is 1 or less, we either miss rotation or num
        .map(|x| {
            println!("val: {x}");
            let multipler: i64 = match &x[0..1] {
                "L" => Ok(-1),
                "R" => Ok(1),
                y => Err(anyhow::anyhow!("Invalid direction {y}")),
            }
            .context("get multipler")?;

            // parse roations
            let rotation: i64 = x[1..].parse().context("parse rotation")?;

            Ok(rotation * multipler)
        })
        .collect();

    rotations
}

fn count_times_ended_on_zero(rotations: &[i64]) -> Result<usize> {
    let mut current_count: usize = 0;
    let mut current_val: i64 = 50;

    rotations.iter().for_each(|x| {
        current_val += x;
        if current_val % 100 == 0 {
            current_count += 1
        }
    });

    Ok(current_count)
}

// Count how many times the dial points at 0 during rotations
fn count_times_hit_zero(rotations: &[i64]) -> Result<usize> {
    let mut current_count: usize = 0;
    let mut current_pos: i64 = 50;

    for movement in rotations.iter() {
        // Count complete revolutions (each passes through 0 once)
        let complete_revolutions = movement.abs() / 100;
        current_count += complete_revolutions as usize;

        // Check if we cross 0 in the remaining partial revolution
        let remaining = movement.abs() % 100;

        if *movement > 0 {
            // Moving clockwise
            // We cross 0 if current + remaining >= 100
            if current_pos + remaining >= 100 {
                current_count += 1;
            }
        } else if *movement < 0 {
            // moving counter clockwise
            // We cross 0 if current position > 0 and we move back far enough
            if current_pos > 0 && current_pos <= remaining {
                current_count += 1;
            }
        }

        // update position
        current_pos = (current_pos + movement).rem_euclid(100);
        println!(
            "movement: {} | count so far: {} | new pos: {}",
            movement, current_count, current_pos
        );
    }

    Ok(current_count)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_example_part1() -> Result<()> {
        let rotations_str: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(rotations_str).context("run")?, 3);

        Ok(())
    }

    #[test]
    fn test_basic_example_part2() -> Result<()> {
        let rotations_str: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part2(rotations_str).context("run")?, 6);

        Ok(())
    }

    #[test]
    fn test_large_rotation() -> Result<()> {
        // From the problem: "if the dial were pointing at 50, a single rotation
        // like R1000 would cause the dial to point at 0 ten times"
        let rotations_str: &str = "R1000";

        assert_eq!(part2(rotations_str).context("run")?, 10);

        Ok(())
    }

    #[test]
    fn test_large_rotation_negative() -> Result<()> {
        // From the problem: "if the dial were pointing at 50, a single rotation
        // like R1000 would cause the dial to point at 0 ten times"
        let rotations_str: &str = "L1000";

        assert_eq!(part2(rotations_str).context("run")?, 10);

        Ok(())
    }
}
