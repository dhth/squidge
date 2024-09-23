use clap::Parser;
use regex::Regex;
use squidge::{shorten_line, Config};
use std::io::{self, BufRead};
use std::process;

const DEFAULT_DELIMITER: &str = "/";
const DEFAULT_IGNORE_FIRST_N: usize = 0;
const DEFAULT_IGNORE_LAST_N: usize = 1;

/// sqdj shortens delimited data
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    /// Delimiter
    #[arg(short = 'd', long = "delimiter", value_name = "STRING")]
    #[clap(default_value = DEFAULT_DELIMITER)]
    delimiter: String,
    /// Regex for ignoring elements (ie, they won't be shortened)
    #[arg(short = 'r', long = "ignore-regex", value_name = "STRING")]
    ignore_regex: Option<String>,
    /// Ignore first n elements
    #[arg(short = 'f', long = "ignore-first-n", value_name = "NUMBER")]
    #[clap(default_value_t = DEFAULT_IGNORE_FIRST_N)]
    ignore_first_n: usize,
    /// Ignore last n elements
    #[arg(short = 'l', long = "ignore-last-n", value_name = "NUMBER")]
    #[clap(default_value_t = DEFAULT_IGNORE_LAST_N)]
    ignore_last_n: usize,
}

fn main() {
    let args = Args::parse();

    let re = args.ignore_regex.map(|r| {
        Regex::new(&r).unwrap_or_else(|err| {
            eprintln!("Error: couldn't compile regex: {}", err);
            process::exit(1);
        })
    });

    let cfg = Config {
        delimiter: &args.delimiter,
        ignore_first_n: args.ignore_first_n,
        ignore_last_n: args.ignore_last_n,
        ignore_regex: re,
    };

    let mut lines = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_else(|err| {
            eprintln!("Error: couldn't read line from stdin: {}", err);
            process::exit(1);
        });
        lines.push(line);
    }

    let shortened_lines: Vec<String> = lines
        .iter()
        .map(|l| {
            let s = shorten_line(&cfg, l);
            s.join(&args.delimiter)
        })
        .collect();

    println!("{}", shortened_lines.join("\n"));
}
