use rand::prelude::*;

pub trait LineStore {
    fn add_line(&mut self, line: String);
    fn get_all_lines(self) -> Vec<String>;
}

pub struct Simple(Vec<String>);

impl Simple {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl LineStore for Simple {
    fn add_line(&mut self, line: String) {
        self.0.push(line);
    }

    fn get_all_lines(self) -> Vec<String> {
        self.0
    }
}

pub struct Reservoir {
    lines: Vec<String>,
    resr_size: usize,
    count: usize,
    rng: ThreadRng,
}
impl Reservoir {
    pub fn new(resr_size: usize) -> Self {
        Self {
            lines: vec![],
            resr_size,
            count: 0,
            rng: rand::rng(),
        }
    }
}

impl LineStore for Reservoir {
    fn add_line(&mut self, line: String) {
        self.count += 1;
        if self.count <= self.resr_size {
            self.lines.push(line);
        } else {
            let i = self.rng.random_range(0..self.count);
            if i < self.resr_size {
                self.lines[i] = line;
            }
        }
    }

    fn get_all_lines(self) -> Vec<String> {
        self.lines
    }
}
