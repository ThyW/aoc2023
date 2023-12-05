use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Almanac {
    pub seeds: Vec<u128>,
    mappings: HashMap<usize, Vec<RangeMap>>,
}

#[derive(Debug, Clone, Default)]
pub struct RangeMap {
    first_start: u128,
    second_start: u128,
    num_elems: u128,
}

impl RangeMap {
    pub fn from_line(line: &str) -> Self {
        let nums: Vec<u128> = line
            .split(' ')
            .map(|x| x.trim().parse::<u128>().expect("Unable to parse number"))
            .collect();
        Self {
            first_start: nums[0],
            second_start: nums[1],
            num_elems: nums[2],
        }
    }

    pub fn get_mapping_for(&self, num: u128) -> Option<u128> {
        if num < self.second_start {
            return None;
        }
        if num > self.second_start + self.num_elems {
            return None;
        }
        let diff = num - self.second_start;
        Some(self.first_start + diff)
    }
}

impl Almanac {
    pub fn from_lines(input: &str) -> Self {
        let mut almanac = Almanac::default();
        let mut to_change = Vec::new();
        let mut current_index = 0;
        for mut line in input.lines() {
            line = line.trim();
            if line.is_empty() || line.contains("seed-to-soil") {
                continue;
            }
            match line {
                line if line.starts_with("seeds: ") => almanac.seeds.extend(
                    line.strip_prefix("seeds: ")
                        .unwrap()
                        .split(' ')
                        .map(|x| x.trim().parse::<u128>().unwrap()),
                ),
                line if !line.as_bytes()[0].is_ascii_digit() => {
                    current_index += 1;
                    almanac.mappings.insert(current_index, to_change.clone());
                    to_change = Vec::new();
                }
                _ => to_change.push(RangeMap::from_line(line)),
            }
        }
        almanac
            .mappings
            .insert(current_index + 1, to_change.clone());

        almanac
    }

    pub fn resolve_seed(&self, seed: u128) -> u128 {
        let mut seeds = vec![seed];
        for lvl in 1..8 {
            let mut next_level = vec![];
            for seed in seeds {
                next_level.extend(self.resolve_seed_level(lvl, seed));
            }
            seeds = next_level.clone();
        }

        seeds.iter().fold(u128::MAX, |a, &e| a.min(e))
    }

    fn resolve_seed_level(&self, level: usize, seed: u128) -> Vec<u128> {
        let out = self.mappings[&(level)]
            .iter()
            .map(|rm| rm.get_mapping_for(seed))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<u128>>();
        if out.is_empty() {
            return vec![seed];
        }
        return out;
    }
}
