extern crate day8;

use day8::*;

fn part(input: &'static str) -> u64 {
    let graph = Way::from_input(input);

    let mut step: u64 = 0;
    let mut positions: Vec<&&str> = graph.nodes.keys().filter(|x| x.ends_with('A')).collect();
    let mut nsteps = Vec::new();

    for i in 0..positions.len() {
        for c in graph.instructions.chars().cycle() {
            let current = positions[i];
            let (left, right) = graph.nodes.get(current).unwrap();

            match c {
                'L' => positions[i] = left,
                'R' => positions[i] = right,
                _ => panic!("unexpected dir"),
            }

            step += 1;

            if positions[i].ends_with('Z') {
                nsteps.push(step);
                step = 0;
                break;
            }
        }
    }

    lcm(&nsteps)
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(b, a % b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
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
        let test_input = include_str!("./test3.txt");
        let test_output = 6;
        assert_eq!(part(test_input), test_output);
    }
}
