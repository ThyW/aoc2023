extern crate day11;

use day11::*;

fn part(input: &str) -> u64 {
    Universe::from_input(input).count_shortest_paths()
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
        let test_output = 8410;
        assert_eq!(part(test_input), test_output);
    }
}
