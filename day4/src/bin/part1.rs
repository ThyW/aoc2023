extern crate day4;

use day4::Card;

fn part(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(Card::from_line)
        .map(|x| x.points / 2)
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part(input);

    println!("result: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let test_input = include_str!("./test.txt");
        let test_output = 13;
        assert_eq!(part(test_input), test_output);
    }
}
