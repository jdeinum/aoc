struct Machine {
    desired: Vec<bool>,
    current: Vec<bool>,
    buttons: Vec<Vec<bool>>,
}

impl Machine {
    fn solve(mut self) -> usize {
        todo!()
    }

    fn is_finished(&self) -> bool {
        self.current == self.desired
    }

    fn press_button(&mut self) {
        todo!()
    }
}

// I decided to use nom for the parsing. It's much easier than trying to do it by hand
fn parse_desired_state() {
    todo!()
}

fn parse_buttons() {
    todo!()
}

fn parse_joltages() {
    todo!()
}

fn parse_machine() {
    todo!()
}

fn parse_machines(input: &str) -> Vec<Machine> {
    todo!()
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

