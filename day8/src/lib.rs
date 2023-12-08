use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Way<'a> {
    pub nodes: HashMap<&'a str, (&'a str, &'a str)>,
    pub instructions: &'a str,
}

impl Way<'_> {
    pub fn from_input(input: &'static str) -> Self {
        let mut hm = HashMap::new();

        let (instructions, rest) = input.split_once("\n").unwrap();

        let entries: Vec<(&str, (&str, &str))> = rest
            .trim()
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let (name, lr) = x.split_once(" = ").unwrap();
                let (left, right) = lr.split_once(", ").unwrap();

                (
                    name,
                    (
                        left.strip_prefix('(').unwrap().trim(),
                        right.strip_suffix(')').unwrap().trim(),
                    ),
                )
            })
            .collect();

        for (key, value) in entries {
            hm.insert(key, value);
        }

        Self {
            instructions: instructions.trim(),
            nodes: hm,
        }
    }
}
