mod ace_sort;
mod store;

use crate::store::LineStore;
use anyhow::{self, Context};
use clap::Parser;
use std::io::{self, Write};

/// Sort input lines, sorting digit sub-strings numerically.
///
/// This is also known as "natural" sorting.  Version sorting (the `-V` option
/// in the UNIX sort command) is similar.  `acesort` is named after ACeDB's
/// default method of sorting where, for example, "chr2" sorts
/// before "chr10".
#[derive(Parser)]
#[command(version, max_term_width = 80)]
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

    let mut all_lines: Vec<String> = if cli.sample > 0 {
        // Read a random sample of lines from the files or STDIN
        let mut store = store::Reservoir::new(cli.sample);
        store.store_all_input(&cli.file)?;
        store.get_all_lines()
    } else {
        // Read the lines of all the files or STDIN
        let mut store = vec![];
        store.store_all_input(&cli.file)?;
        store
    };

    // Sort lines and print them all to STDOUT
    all_lines.sort_unstable_by(|a, b| ace_sort::ace_cmp(a, b));
    if cli.unique {
        write_vec_unique_to_stdout(&all_lines).context("Writing to STDOUT")?;
    } else {
        write_vec_to_stdout(&all_lines).context("Writing to STDOUT")?;
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
