use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    pub fn from_line(
        input: &str,
        f: impl Fn(HashMap<Card, usize>) -> HandType,
    ) -> (Vec<Card>, HandType) {
        let mut hm: HashMap<Card, usize> = HashMap::new();
        let mut out = vec![];

        for c in input.chars() {
            let card = match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => panic!("badcard"),
            };
            out.push(card);
            let count = hm.get(&card);
            match count {
                Some(cnt) => hm.insert(card, *cnt + 1),
                None => hm.insert(card, 1),
            };
        }

        (out, f(hm))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    pub fn part_1(hm: HashMap<Card, usize>) -> Self {
        let items: Vec<(Card, usize)> = hm.iter().map(|(x, y)| (*x, *y)).collect();

        match items.len() {
            1 => Self::FiveKind,
            2 => {
                let count_fst = items[0].1;
                let count_snd = items[1].1;

                if (count_fst == 1 && count_snd == 4) || (count_fst == 4 && count_snd == 1) {
                    Self::FourKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if items[0].1 == 2 || items[1].1 == 2 || items[2].1 == 2 {
                    Self::TwoPair
                } else {
                    Self::ThreeKind
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("bad cards"),
        }
    }

    pub fn part_2(hm: HashMap<Card, usize>) -> Self {
        let items: Vec<(Card, usize)> = hm.iter().map(|(x, y)| (*x, *y)).collect();
        let n_jokers: usize = items
            .iter()
            .filter(|(c, _)| *c == Card::J)
            .map(|(_, n)| n)
            .sum();

        match items.len() {
            1 => Self::FiveKind,
            2 => {
                let count_fst = items[0].1;
                let count_snd = items[1].1;

                if n_jokers >= 1 {
                    return Self::FiveKind;
                }

                if (count_fst == 1 && count_snd == 4) || (count_fst == 4 && count_snd == 1) {
                    Self::FourKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if items[0].1 == 2 || items[1].1 == 2 || items[2].1 == 2 {
                    if n_jokers == 2 {
                        return Self::FourKind;
                    }
                    if n_jokers == 1 {
                        return Self::FullHouse;
                    }
                    Self::TwoPair
                } else {
                    if n_jokers >= 1 {
                        return Self::FourKind;
                    }
                    Self::ThreeKind
                }
            }
            4 => {
                if n_jokers >= 1 {
                    return Self::ThreeKind;
                }
                Self::OnePair
            }
            5 => {
                if n_jokers >= 1 {
                    return Self::OnePair;
                }
                Self::HighCard
            }
            _ => panic!("bad cards"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: u64,
    pub hand_type: HandType,
}

impl Hand {
    pub fn from_line(input: &str, f: impl Fn(HashMap<Card, usize>) -> HandType) -> Self {
        let (c, b) = input.trim().split_once(' ').unwrap();

        let (cards, hand_type) = Card::from_line(c, f);
        let bid = b.trim().parse().unwrap();

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type < other.hand_type {
            return std::cmp::Ordering::Less;
        }
        if self.hand_type > other.hand_type {
            return std::cmp::Ordering::Greater;
        }
        if self.hand_type == other.hand_type {
            for (&our, &their) in self.cards.iter().zip(other.cards.iter()) {
                if our > their {
                    // println!("more: {:?}, {:?}", our, their);
                    return std::cmp::Ordering::Greater;
                }

                if our < their {
                    // println!("less: {:?} {:?}", our, their);
                    return std::cmp::Ordering::Less;
                }
            }
        }
        std::cmp::Ordering::Equal
    }
}
