fn part1(input: &str) -> usize {
    todo!()
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
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(part1(input), 5);
    }
}

