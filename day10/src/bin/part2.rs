extern crate day10;

use day10::*;

fn part(input: &str) -> u32 {
    PipeGraph::init_from_str(input).tiles_within()
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
        let test_output = 10;
        assert_eq!(part(test_input), test_output);
    }

    #[test]
    fn yes() {
        let test_input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let test_output = 4;
        assert_eq!(part(test_input), test_output);
    }
    #[test]
    fn yes2() {
        let test_input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let test_output = 4;
        assert_eq!(part(test_input), test_output);
    }
}
