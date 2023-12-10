use std::collections::{HashMap, HashSet};

pub struct PipeGraph {
    steps: HashMap<char, ((i32, i32), (i32, i32))>,
    lines: Vec<Vec<char>>,
    initial: (i32, i32),
    initial_pipe: char,
}

impl PipeGraph {
    pub fn init_from_str(input: &str) -> Self {
        let mut steps = HashMap::new();

        steps.insert('|', ((1, 0), (-1, 0)));
        steps.insert('-', ((0, -1), (0, 1)));
        steps.insert('L', ((-1, 0), (0, 1)));
        steps.insert('J', ((-1, 0), (0, -1)));
        steps.insert('F', ((0, 1), (1, 0)));
        steps.insert('7', ((0, -1), (1, 0)));

        let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

        let mut pos = (-1, -1);
        for (i, line) in lines.iter().enumerate() {
            for (j, &char) in line.iter().enumerate() {
                if char == 'S' {
                    pos = (i as i32, j as i32);
                    break;
                }
            }
        }

        assert!(pos.0 >= 0 && pos.1 >= 0);

        Self {
            steps,
            lines,
            initial: pos,
            initial_pipe: '.',
        }
    }

    pub fn walk_cycle(&mut self) -> (HashSet<(i32, i32)>, u32) {
        let (mut d1, mut d2) = (0u32, 0u32);
        let (mut p1, mut p2) = self.start_positions();
        let d = [
            (self.initial.0 - p1.0, self.initial.1 - p1.1),
            (self.initial.0 - p2.0, self.initial.1 - p2.1),
        ];

        self.initial_pipe = "-|JLF7"
            .chars()
            .filter(|x| {
                [self.steps[x].0, self.steps[x].1]
                    .iter()
                    .all(|diff| d.contains(diff))
            })
            .collect::<Vec<char>>()[0];

        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        visited.insert(self.initial);

        while !visited.contains(&p1) || !visited.contains(&p2) {
            if let Some(c) = self.get_on_pos(p1, (0, 0)) {
                visited.insert(p1);
                let diffs = self.get_diffs(c);
                let np1 = diffs.iter().find_map(|&d| {
                    let pos = self.get_pos(p1, d);
                    if visited.contains(&pos) {
                        return None;
                    } else {
                        return Some(pos);
                    }
                });

                if np1.is_none() {
                    return (visited, d1 + 1);
                }
                p1 = np1.unwrap();

                d1 += 1;
            }
            if let Some(c) = self.get_on_pos(p2, (0, 0)) {
                visited.insert(p2);
                let diffs = self.get_diffs(c);
                let np2 = diffs.iter().find_map(|&d| {
                    let pos = self.get_pos(p2, d);
                    if visited.contains(&pos) {
                        return None;
                    } else {
                        return Some(pos);
                    }
                });

                if np2.is_none() {
                    return (visited, d2 + 1);
                }
                p2 = np2.unwrap();
                d2 += 1;
            }
        }

        dbg!(&p1, &p2);

        (visited, d1.max(d2))
    }

    pub fn get_largest_dist(&mut self) -> u32 {
        self.walk_cycle().1
    }

    pub fn tiles_within(&mut self) -> u32 {
        let visited = self.walk_cycle().0;
        let mut count = 0;
        // assert!(visited.contains(&(0, 0)));
        for (i, line) in self.lines.clone().into_iter().enumerate() {
            let mut inside = false;
            for j in 1..line.len() {
                let pos = (i as i32, j as i32);
                if visited.contains(&pos) {
                    let on_pos = self.get_on_pos(pos, (0, 0)).unwrap();
                    match on_pos {
                        '|' | '7' | 'F' => inside = !inside,
                        _ => (),
                    };
                } else if inside {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_on_pos(&self, initial: (i32, i32), diff: (i32, i32)) -> Option<char> {
        let (x, y) = self.get_pos(initial, diff);

        if x < 0 || x >= self.lines.len() as i32 {
            return None;
        } else if y < 0 || y >= self.lines[x as usize].len() as i32 {
            return None;
        } else {
            Some(self.lines[x as usize][y as usize])
        }
    }

    fn get_pos(&self, initial: (i32, i32), diff: (i32, i32)) -> (i32, i32) {
        (initial.0 + diff.0, initial.1 + diff.1)
    }

    fn get_diffs(&self, c: char) -> [(i32, i32); 2] {
        let (a, b) = self.steps[&c];
        return [a, b];
    }

    fn start_positions(&self) -> ((i32, i32), (i32, i32)) {
        let mut out = vec![];
        for (i, diff) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().enumerate() {
            let pos = self.get_pos(self.initial, *diff);
            // dbg!(pos);
            if let Some(c) = self.get_on_pos(pos, (0, 0)) {
                if i % 2 != 0 && (matches!(c, '|' | '7' | 'F') || matches!(c, '|' | 'J' | 'L')) {
                    out.push(pos);
                } else if i % 2 == 0
                    && (matches!(c, '-' | 'J' | '7') || matches!(c, '-' | 'L' | 'F'))
                {
                    out.push(pos);
                }
            }
        }

        (out[0], out[1])
    }
}
