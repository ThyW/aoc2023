extern crate day7;

use day7::*;

fn part(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::from_line(line, HandType::part_2))
        .collect();

    hands.sort_by(|a, b| a.cmp(b));
    // sort_by_key(&mut hands, |x| x.hand_type);
    // sort_by_key(&mut hands, |x| x.cards.clone());

    // println!("{:#?}", &hands);

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
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
        let test_output = 5905;
        assert_eq!(part(test_input), test_output);
    }
}
