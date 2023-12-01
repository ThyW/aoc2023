fn part(input: &str) -> u32 {
    let mut sum = 0;
    for mut line in input.lines() {
        line = line.trim();
        let (mut x, mut y) = (0, 0);
        let mut buff = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                x = if x == 0 { c.to_digit(10).unwrap() } else { x };
                y = c.to_digit(10).unwrap();
            } else {
                buff.push(c);
                let mut num: u32 = 0;
                for (i, each) in [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ]
                .iter()
                .enumerate()
                {
                    if buff.ends_with(each) {
                        num = i as u32 + 1;
                        break;
                    }
                }

                if num > 0 {
                    x = if x == 0 { num } else { x };
                    y = num;
                }
            }
        }
        println!("{line} {x}{y}");

        sum += x * 10 + y;
    }
    sum
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
    fn it_works2() {
        let test_input = include_str!("./test2.txt");
        let test_output = 281;
        assert_eq!(part(test_input), test_output);
    }

    #[test]
    fn part2() {
        let test_input = include_str!("./input.txt");
        assert_eq!(part(test_input), 54078);
    }
}
