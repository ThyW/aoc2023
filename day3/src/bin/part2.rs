use std::collections::HashMap;

#[derive(Clone, Debug)]
struct PartialNum {
    y_start: i32,
    y_end: i32,
    x: i32,
    n: u32,
}

impl PartialNum {
    fn new(y_start: i32, y_end: i32, x: i32, num: &[char]) -> Self {
        Self {
            y_start,
            y_end,
            x,
            n: num
                .iter()
                .cloned()
                .collect::<String>()
                .parse::<u32>()
                .expect("bad num"),
        }
    }

    fn neighbour_of(&self, x: i32, y: i32) -> bool {
        (x == self.x || x == self.x - 1 || x == self.x + 1)
            && (y >= self.y_start - 1 && y <= self.y_end + 1)
    }
}

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
    let mut partial_nums: HashMap<i32, Vec<PartialNum>> = HashMap::new();
    let mut gears = vec![];
    for (ii, line) in lines.iter().enumerate() {
        let i = ii as i32;
        partial_nums.insert(i, vec![]);
        let mut num_buff = vec![];
        for (ci, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if num_buff.is_empty() {
                    n_start = ci as i32;
                }
                num_buff.push(c);
            } else {
                if c == '*' {
                    gears.push((i, ci as i32));
                }
                if !num_buff.is_empty() {
                    if !not_part_num(
                        lines.as_ref(),
                        linesize as i32,
                        i as i32,
                        n_start as i32,
                        ci as i32,
                    ) {
                        partial_nums.get_mut(&i).unwrap().push(PartialNum::new(
                            n_start,
                            ci as i32 - 1,
                            i,
                            &num_buff,
                        ));
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
                partial_nums.get_mut(&i).unwrap().push(PartialNum::new(
                    n_start,
                    line.len() as i32 - 1,
                    i,
                    &num_buff,
                ));
            }
        }
        num_buff.clear();
    }

    dbg!(&gears);
    for (i, ci) in gears {
        let mut part_nums = vec![];
        for ii in i - 1..=i + 1 {
            let ii: i32 = ii;
            if ii >= 0 && ii < lines.len() as i32 {
                part_nums.extend(partial_nums.get(&ii).unwrap());
            }
        }

        let filtered: Vec<&&PartialNum> =
            part_nums.iter().filter(|x| x.neighbour_of(i, ci)).collect();
        if filtered.len() == 2 {
            dbg!(&filtered);
            sum += filtered[0].n * filtered[1].n;
        }
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
    fn part2() {
        let test_input = include_str!("./test.txt");
        let test_output = 467835;
        assert_eq!(part(test_input), test_output);
    }
}
