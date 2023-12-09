extern crate day9;
use day9::*;

fn part(input: &str) -> i64 {
    input
        .lines()
        .map(|x| Values::from_line(x).extrapolate_back())
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
    fn it_works() {
        let test_input = include_str!("./test.txt");
        let test_output = 114;
        assert_eq!(part(test_input), test_output);
    }
}
