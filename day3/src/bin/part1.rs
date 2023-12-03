fn not_part_num(lines: &[&str], linesize: i32, i: i32, n_start: i32, ci: i32) -> bool {
    (n_start - 1..=ci)
        .map(|y| (i - 1, y))
        .chain(std::iter::once((i, n_start - 1)).chain(std::iter::once((i, ci))))
        .chain((n_start - 1..=ci).map(|y| (i + 1, y)))
        .all(|(x, y)| {
            if (x > 0 && x < lines.len() as i32) && (y > 0 && y < linesize) {
                return lines[x as usize].as_bytes()[y as usize] == b'.';
            }
            return true;
        })
}

fn part(input: &str) -> u32 {
    let mut sum = 0;

    let (fst, _) = input.split_once('\n').expect("unable to split");
    let linesize = fst.len();

    let mut n_start = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (ii, line) in lines.iter().enumerate() {
        let i = ii as i32;
        let mut num_buff = vec![];
        for (ci, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if num_buff.is_empty() {
                    n_start = ci as i32;
                }
                num_buff.push(c);
            } else {
                if !num_buff.is_empty() {
                    if !not_part_num(
                        lines.as_ref(),
                        linesize as i32,
                        i as i32,
                        n_start as i32,
                        ci as i32,
                    ) {
                        let n = num_buff
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .parse::<u32>()
                            .expect("bad num");
                        sum += n;
                        dbg!(&n);
                    }
                }
                num_buff.clear();
            }
        }
        if !num_buff.is_empty() {
            if !not_part_num(
                lines.as_ref(),
                linesize as i32,
                i as i32,
                n_start as i32,
                line.len() as i32,
            ) {
                let n = num_buff
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("bad num");
                sum += n;
                dbg!(&n);
            }
        }
        num_buff.clear();
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
    fn part1() {
        let test_input = include_str!("./test.txt");
        let test_output = 4361;
        assert_eq!(part(test_input), test_output);
    }
}
