const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

fn part(input: &str) -> u32 {
    let mut sum = 0;
    for mut line in input.lines() {
        line = line.trim();
        line = line
            .strip_prefix("Game ")
            .expect("does not start with Game");

        let gameid_str: String = line.chars().take_while(|x| *x != ':').collect();
        let gameid: u32 = gameid_str.parse().expect("bad gameid");
        let mut rgb = [0u32; 3];
        for subset in line
            .strip_prefix(&gameid_str)
            .unwrap()
            .strip_prefix(": ")
            .unwrap()
            .split(';')
        {
            for mut part in subset.split(",") {
                for (i, each) in [" red", " green", " blue"].iter().enumerate() {
                    if part.ends_with(each) {
                        part = part.strip_suffix(each).unwrap().trim();
                        let n: u32 = part.parse().unwrap();
                        if n > rgb[i] {
                            rgb[i] = n;
                        }
                    }
                }
            }
        }
        if rgb[0] <= R && rgb[1] <= G && rgb[2] <= B {
            sum += gameid;
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
    fn part1() {
        let test_input = include_str!("./test.txt");
        let test_output = 8;
        assert_eq!(part(test_input), test_output);
    }
}
