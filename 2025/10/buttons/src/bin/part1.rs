use std::collections::{HashSet, VecDeque};

use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, digit1, multispace0, newline},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, preceded},
};

struct Machine {
    desired: Vec<bool>,
    current: Vec<bool>,
    buttons: Vec<Vec<bool>>,
}

impl Machine {
    fn solve(self) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((self.current.clone(), 0));
        visited.insert(self.current.clone());

        while let Some((state, steps)) = queue.pop_front() {
            if state == self.desired {
                return steps;
            }

            for button in &self.buttons {
                let mut next_state = state.clone();

                for i in 0..next_state.len() {
                    next_state[i] ^= button[i];
                }

                // if we haven't seen the new state yet, add it
                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, steps + 1));
                }
            }
        }

        0
    }
}

// I decided to use nom for the parsing. It's much easier than trying to do it by hand
fn parse_desired_state(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(
        char('['),
        many1(alt((char('.').map(|_| false), char('#').map(|_| true)))),
        char(']'),
    )
    .parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list0(char(','), digit1.map(|s: &str| s.parse().unwrap())),
        char(')'),
    )
    .parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    many1(preceded(multispace0, parse_button)).parse(input)
}

fn parse_joltages(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('{'),
        separated_list0(char(','), digit1.map(|s: &str| s.parse().unwrap())),
        char('}'),
    )
    .parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, desired) = parse_desired_state(input)?;
    let (input, _) = multispace0(input)?;
    let (input, button_indices) = parse_buttons(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _joltages) = parse_joltages(input)?;

    let n = desired.len();
    let current = vec![false; n];

    // Convert button indices to Vec<bool> representation
    let buttons: Vec<Vec<bool>> = button_indices
        .into_iter()
        .map(|indices| {
            let mut button = vec![false; n];
            for idx in indices {
                button[idx] = true;
            }
            button
        })
        .collect();

    Ok((
        input,
        Machine {
            desired,
            current,
            buttons,
        },
    ))
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let result: IResult<&str, Vec<Machine>> = separated_list1(newline, parse_machine).parse(input);
    result.unwrap().1
}

fn part1(input: &str) -> usize {
    let machines = parse_machines(input);
    machines.into_iter().map(|x| x.solve()).sum()
}

fn main() {
    let fp = std::env::args().skip(1).next().unwrap();

    let s = std::fs::read_to_string(&fp).unwrap();

    let res = part1(&s);

    println!("res: {res}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part1(input), 7);
    }
}
