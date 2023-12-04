#[derive(Clone, Debug)]
pub struct Card {
    pub id: u32,
    pub points: u32,
    pub winning: Vec<u32>,
    pub has: Vec<u32>,
}

impl Card {
    pub fn from_line(mut input: &str) -> Self {
        input = input.trim();
        let (card, rest) = input.split_once(": ").unwrap();
        let id_str = card.strip_prefix("Card ").unwrap();
        let id = id_str.trim().parse().unwrap();

        let (winning_nums, has_nums) = rest.split_once(" | ").unwrap();

        let winning: Vec<u32> = winning_nums
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let has: Vec<u32> = has_nums
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse().unwrap())
            .collect();

        Self {
            id,
            points: 2u32.pow(winning.iter().filter(|x| has.contains(x)).count() as u32),
            winning,
            has,
        }
    }

    pub fn num_winning(&self) -> u32 {
        self.winning.iter().filter(|x| self.has.contains(x)).count() as u32
    }
}
