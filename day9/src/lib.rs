#[derive(Clone, Debug)]
pub struct Values {
    values: Vec<i64>,
}

impl Values {
    pub fn from_line(input: &str) -> Self {
        Self {
            values: input
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }

    fn differences(&self) -> Vec<Vec<i64>> {
        let mut other_values: Vec<Vec<i64>> = vec![self.values.clone()];

        let mut previous = self.values.clone();
        let mut next_values: Vec<i64>;

        while !previous.iter().all(|x| *x == 0) {
            next_values = previous.windows(2).map(|a| a[1] - a[0]).collect();
            other_values.push(next_values.clone());
            previous = next_values.clone();
            next_values.clear();
        }
        other_values
    }

    pub fn extrapolate_back(&self) -> i64 {
        let mut other_values = self.differences();
        other_values.last_mut().as_mut().unwrap().push(0);
        for i in (1..other_values.len()).rev() {
            let a = other_values[i].last().copied().unwrap();
            let b = other_values[i - 1].last().copied().unwrap();

            other_values[i - 1].push(a + b);
        }

        other_values[0].last().copied().unwrap()
    }

    pub fn extrapolate_front(&self) -> i64 {
        let mut other_values = self.differences();

        other_values.last_mut().as_mut().unwrap().insert(0, 0);
        for i in (1..other_values.len()).rev() {
            let a = other_values[i].first().copied().unwrap();
            let b = other_values[i - 1].first().copied().unwrap();

            other_values[i - 1].insert(0, b - a);
        }

        dbg!(&other_values);

        other_values[0].first().copied().unwrap()
    }
}
