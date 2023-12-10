extern crate day10;

use day10::*;

fn part(input: &str) -> u32 {
    PipeGraph::init_from_str(input).get_largest_dist()
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
        let test_output = 8;
        assert_eq!(part(test_input), test_output);
    }
}
