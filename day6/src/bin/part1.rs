extern crate day6;

use day6::*;

fn part(input: &str) -> u32 {
    let races = Race::from_input(input);

    races.iter().map(|race| race.ways_to_win()).product()
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
        let test_output = 288;
        assert_eq!(part(test_input), test_output);
    }
}
