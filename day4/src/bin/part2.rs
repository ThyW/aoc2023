extern crate day4;

use day4::Card;
use std::collections::HashMap;

fn part(input: &str) -> u32 {
    let cards: Vec<_> = input.trim().lines().map(Card::from_line).collect();

    let mut hm = HashMap::new();

    for card in cards.iter() {
        hm.insert(card.id, vec![card.clone()]);
    }

    for ii in 1..cards[cards.len() - 1].id + 1 {
        let i = ii as u32;
        let mut to_insert: Vec<(u32, Card)> = Vec::new();
        for card in hm.get_mut(&i).unwrap().iter() {
            let n_cards = card.num_winning();

            for new_id in i + 1..i + n_cards + 1 {
                to_insert.push((new_id, cards[new_id as usize - 1].clone()))
            }
        }

        for (cardi, card) in to_insert {
            hm.get_mut(&cardi).unwrap().push(card);
        }
    }

    hm.values().map(|x| x.len()).sum::<usize>() as u32
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
        let test_output = 30;
        assert_eq!(part(test_input), test_output);
    }
}
