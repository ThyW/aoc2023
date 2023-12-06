extern crate day6;

use day6::*;

fn part(input: &str) -> u32 {
    Race::from_input2(input).ways_to_win()
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
        let test_output = 71503;
        assert_eq!(part(test_input), test_output);
    }
}
