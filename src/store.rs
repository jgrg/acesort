use anyhow::{self, Context};
use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait LineStore {
    fn add_line(&mut self, line: String);

    /// Stores lines from `STDIN`
    fn store_stdin_lines(&mut self) -> io::Result<()> {
        for line in io::stdin().lines() {
            self.add_line(line?);
        }
        Ok(())
    }

    /// Stores lines from the file at `file_path`
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

    /// Stores lines from all the files in `file_list` or from `STDIN`
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

impl LineStore for Vec<String> {
    fn add_line(&mut self, line: String) {
        self.push(line);
    }
}


/// Contains internal state required for reservoir sampling of the input lines
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

    pub fn get_all_lines(self) -> Vec<String> {
        self.lines
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
}
