extern crate day8;

use day8::*;

fn part(input: &'static str) -> u64 {
    let graph = Way::from_input(input);

    let mut step: u64 = 0;
    let mut current = "AAA";

    for instructions in std::iter::repeat(graph.instructions) {
        for c in instructions.chars() {
            let (left, right) = graph.nodes.get(current).unwrap();

            match c {
                'L' => current = left,
                'R' => current = right,
                _ => panic!("unexpected dir"),
            }
            step += 1;

            if current == "ZZZ" {
                return step;
            }
        }
    }

    step
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
    fn t1() {
        let test_input = include_str!("./test.txt");
        let test_output = 2;
        assert_eq!(part(test_input), test_output);
    }

    #[test]
    fn t2() {
        let test_input = include_str!("./test2.txt");
        let test_output = 6;
        assert_eq!(part(test_input), test_output);
    }
}
