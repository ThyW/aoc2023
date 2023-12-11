const TIMES: i32 = 99;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Galaxy {
    n: u32,
    pos: (i64, i64),
}

impl Galaxy {
    pub fn new(n: u32, x: i64, y: i64) -> Self {
        Self { n, pos: (x, y) }
    }

    pub fn diff(&self, other: Galaxy) -> u64 {
        let x1 = self.pos.0;
        let x2 = other.pos.0;
        let y1 = self.pos.1;
        let y2 = other.pos.1;
        ((x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2))) as u64
    }
}

#[derive(Clone, Debug)]
pub struct Universe {
    galaxies: Vec<Galaxy>,
}

impl Universe {
    pub fn from_input(input: &str) -> Self {
        let mut universe: Vec<Vec<char>> = Vec::new();

        let mut x_multiplier = 0;
        let mut x_multipliers = vec![];

        for line in input.lines() {
            let l: Vec<char> = line.chars().collect();
            if line.chars().all(|x| x == '.') {
                x_multiplier += 1;
            }
            x_multipliers.push(x_multiplier);
            universe.push(l);
        }

        let mut y_multiplier = 0;
        let mut y_multipliers = vec![];

        for i in 0..universe[0].len() {
            if universe.iter().all(|x| x[i] == '.') {
                y_multiplier += 1;
            }
            y_multipliers.push(y_multiplier);
        }

        let mut galaxies = Vec::new();
        let mut galaxy_counter = 0;
        for (x, line) in universe.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                if *c == '#' {
                    galaxy_counter += 1;
                    galaxies.push(Galaxy::new(
                        galaxy_counter,
                        (x as i32 + (TIMES * x_multipliers[x])) as i64,
                        (y as i32 + (TIMES * y_multipliers[y])) as i64,
                    ));
                }
            }
        }

        Self { galaxies }
    }

    pub fn galaxy_pairs(&self) -> Vec<(Galaxy, Galaxy)> {
        let mut pairs = Vec::new();
        for (i, fst) in self.galaxies.iter().enumerate() {
            for snd in &self.galaxies[i + 1..] {
                pairs.push((*fst, *snd));
            }
        }
        pairs
    }

    pub fn count_shortest_paths(&self) -> u64 {
        let res = self
            .galaxy_pairs()
            .iter()
            .map(|(a, b)| a.diff(*b))
            .sum::<u64>();

        return res;
    }
}
