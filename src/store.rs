use anyhow::{self, Context};
use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait LineStore {
    fn add_line(&mut self, line: String);
    fn get_all_lines(self) -> Vec<String>;

    fn store_stdin_lines(&mut self) -> io::Result<()> {
        for line in io::stdin().lines() {
            self.add_line(line?);
        }
        Ok(())
    }

    fn store_file_lines<P>(&mut self, file_path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let fh = File::open(file_path)?;
        for line in io::BufReader::new(fh).lines() {
            self.add_line(line?);
        }
        Ok(())
    }

    fn store_all_input(&mut self, file_list: &[String]) -> anyhow::Result<()> {
        if file_list.is_empty() {
            self.store_stdin_lines().context("Reading STDIN")?;
        } else {
            for file_path in file_list {
                self.store_file_lines(file_path)
                    .context(format!("Reading file '{file_path}'"))?;
            }
        }
        Ok(())
    }
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
