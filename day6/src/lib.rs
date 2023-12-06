#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct Race {
    duration: u64,
    distance: u64,
}

impl Race {
    pub fn from_input(input: &str) -> Vec<Self> {
        let (times, dists) = input.split_once('\n').expect("Wrong format");

        times
            .strip_prefix("Time: ")
            .unwrap()
            .trim()
            .split_whitespace()
            .zip(
                dists
                    .strip_prefix("Distance:")
                    .unwrap()
                    .trim()
                    .split_whitespace(),
            )
            .map(|(t, d)| Race {
                duration: t.parse().unwrap(),
                distance: d.parse().unwrap(),
            })
            .collect()
    }

    pub fn from_input2(input: &str) -> Self {
        let (times, dists) = input.split_once('\n').expect("Wrong format");

        let t = times
            .strip_prefix("Time: ")
            .unwrap()
            .trim()
            .split_whitespace()
            .fold(String::new(), |a, e| a + e);
        let d = dists
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split_whitespace()
            .fold(String::new(), |a, e| a + e);

        Self {
            duration: t.parse().unwrap(),
            distance: d.parse().unwrap(),
        }
    }

    pub fn ways_to_win(&self) -> u32 {
        let mut velocity;
        let mut count = 0;
        for i in 0..self.duration {
            velocity = i;
            count += ((self.duration - i) * velocity > self.distance) as u32;
        }
        count
    }
}
