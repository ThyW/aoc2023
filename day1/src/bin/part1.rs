fn part(input: &str) -> u32 {
    input
        .lines()
        .map(|mut f| {
            f = f.trim();
            let (mut x, mut y): (u32, u32) = (0, 0);
            for c in f.chars() {
                if c.is_digit(10) && x == 0 {
                    x = c.to_digit(10).unwrap();
                    y = c.to_digit(10).unwrap();
                } else if c.is_digit(10) {
                    y = c.to_digit(10).unwrap();
                }
            }
            return x * 10 + y;
        })
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
        let test_output = 142;
        assert_eq!(part(test_input), test_output);
    }

    #[test]
    fn part1() {
        let test_input = include_str!("./input.txt");
        assert_eq!(part(test_input), 54601);
    }
}
