mod ace_sort;

use anyhow::{self, Context};
use clap::Parser;
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
    if cli.file.is_empty() {
        read_stdin_into_vec(&mut all_lines).context("Error reading STDIN")?;
    } else {
        for file_path in cli.file {
            read_file_lines_into_vec(&mut all_lines, &file_path)
                .context(format!("Error reading file '{file_path}'"))?;
        }
    }

    // Sort lines and print them all to STDOUT
    all_lines.sort_unstable_by(|a, b| ace_sort::ace_cmp(a, b));
    write_vec_to_stdout(&all_lines).context("Error writing to STDOUT")?;

    Ok(())
}

fn read_stdin_into_vec(lines_vec: &mut Vec<String>) -> io::Result<()> {
    for line in io::stdin().lines() {
        lines_vec.push(line?);
    }
    Ok(())
}

fn read_file_lines_into_vec<P>(lines_vec: &mut Vec<String>, file_path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let fh = File::open(file_path)?;
    for line in io::BufReader::new(fh).lines() {
        lines_vec.push(line?);
    }
    Ok(())
}

fn write_vec_to_stdout(lines_vec: &Vec<String>) -> io::Result<()> {
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
