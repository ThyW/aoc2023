extern crate day5;

use day5::*;

fn part(input: &str) -> u128 {
    let almanac = Almanac::from_lines(input);

    let mut results = vec![];
    for seed in almanac.seeds.iter() {
        results.push(almanac.resolve_seed(*seed));
    }
    results.iter().fold(u128::MAX, |a, &elem| a.min(elem))
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
        let test_output = 35;
        assert_eq!(part(test_input), test_output);
    }
}
