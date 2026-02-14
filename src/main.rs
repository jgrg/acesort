use anyhow::{self, Context};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut args_itr = env::args();
    let _cmd = args_itr.next();
    let files: Vec<String> = args_itr.collect();

    let mut all_lines: Vec<String> = vec![];
    if files.is_empty() {
        read_stdin_into_vec(&mut all_lines).context("Error reading STDIN")?;
    } else {
        for file_path in files {
            read_file_lines_into_vec(&mut all_lines, &file_path)
                .context(format!("Error reading file '{file_path}'"))?;
        }
    }

    all_lines.sort();
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
