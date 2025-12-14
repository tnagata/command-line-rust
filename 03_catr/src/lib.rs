use anyhow::{Result, Context};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
pub struct Config {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}



// --------------------------------------------------
pub fn run(config: Config) -> Result<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: open: {e}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result.with_context(|| format!("reading {filename}"))?;
                    if config.number_lines {
                        println!("{:6}\t{line}", line_num + 1);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/*
reading コンテキストを読み込みエラーに付与（line_result.with_context(...)）。
ファイルを開けない場合のメッセージを構造化（{filename}: open: {e}）。
*/