extern crate day5;

use day5::*;

use std::thread::*;

fn part(input: &str) -> u128 {
    let almanac = Almanac::from_lines(input);

    let mut pairs = Vec::new();
    let mut x = 0;
    let mut y;

    let mut handles = vec![];

    for (i, each) in almanac.seeds.iter().enumerate() {
        if i % 2 == 0 {
            x = *each;
        } else {
            y = *each;
            pairs.push((x, y));
        }
    }

    for (i, pair) in pairs.iter().enumerate() {
        let pair = pair.clone();
        let almanac = almanac.clone();
        handles.push(spawn(move || {
            let mut min = u128::MAX;
            let (s, e) = pair;
            for seed in s..s + e {
                let r = almanac.resolve_seed(seed);
                if min > r {
                    min = r;
                }
            }
            println!("processed {i} pair ({s}, {e}): {min}");
            min
        }))
    }

    let mut min = u128::MAX;
    for h in handles {
        let res = h.join().unwrap();
        if min > res {
            min = res;
        }
    }

    min
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
        let test_output = 46;
        assert_eq!(part(test_input), test_output);
    }
}
