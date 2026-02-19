mod ace_sort;

use anyhow::{self, Context};
use clap::Parser;
use rand::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Sort input lines, sorting digit sub-strings numerically
///
/// This is also known as "natural" sorting. Version sorting ("-V" in the UNIX
/// sort command) is similar. `acesort` is named after ACeDB's default method
/// of sorting where, for example, "chr2" sorts before "chr10".
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// List of one or more files to sort, or STDIN if empty.
    file: Vec<String>,

    /// Unique lines only.
    ///
    /// Removes duplicate lines from the sorted output.
    #[arg(short, long)]
    unique: bool,

    /// Sample this number of randomly selected lines from the input.
    ///
    /// Uses reservoir sampling.  Defaults to showing 20 lines if this flag is
    /// set without an argument.
    #[arg(
        short,
        long,
        value_name="INTEGER",
        default_value="0",
        num_args=0..=1,
        default_missing_value="20",
        hide_default_value=true,
    )]
    sample: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Read the lines of all the files or STDIN into a single Vec
    let mut all_lines: Vec<String> = vec![];
    if cli.sample > 0 {
        let smpl = cli.sample;
        let mut rng = rand::rng();
        let mut count = 0;
        let sampler = |x| {
            if count < smpl {
                all_lines.push(x);
            } else {
                let n = rng.random_range(0..count);
                if n < smpl {
                    all_lines[n] = x;
                }
            }
            count += 1;
        };
        read_all_input(sampler, &cli.file)?;
    } else {
        read_all_input(|x| all_lines.push(x), &cli.file)?;
    }

    // Sort lines and print them all to STDOUT
    all_lines.sort_unstable_by(|a, b| ace_sort::ace_cmp(a, b));
    write_vec_to_stdout(&all_lines).context("Writing to STDOUT")?;

    Ok(())
}

fn read_all_input(mut store: impl FnMut(String), file_list: &[String]) -> anyhow::Result<()> {
    if file_list.is_empty() {
        read_stdin_into_vec(&mut store).context("Reading STDIN")?;
    } else {
        for file_path in file_list {
            read_file_lines_into_vec(&mut store, file_path)
                .context(format!("Reading file '{file_path}'"))?;
        }
    }
    Ok(())
}

fn read_stdin_into_vec(mut store: impl FnMut(String)) -> io::Result<()> {
    for line in io::stdin().lines() {
        store(line?);
    }
    Ok(())
}

fn read_file_lines_into_vec<P>(mut store: impl FnMut(String), file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let fh = File::open(file_path)?;
    for line in io::BufReader::new(fh).lines() {
        store(line?);
    }
    Ok(())
}

fn write_vec_to_stdout(lines_vec: &[String]) -> io::Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    for line in lines_vec {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

fn write_vec_unique_to_stdout(lines_vec: &[String]) -> io::Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    let mut itr = lines_vec.iter();
    let Some(mut last) = itr.next() else {
        return Ok(());
    };
    writeln!(out, "{last}")?;
    for line in itr {
        if line != last {
            writeln!(out, "{line}")?;
            last = line;
        }
    }
    Ok(())
}
