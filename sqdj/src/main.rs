use clap::Parser;
use regex::Regex;
use squidge::{shorten_line, Config};
use std::io::{self, BufRead};
use std::process;

const DEFAULT_DELIMITER: &str = "/";
const DEFAULT_IGNORE_FIRST_N: usize = 0;
const DEFAULT_IGNORE_LAST_N: usize = 1;

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
    /// Output delimiter
    #[arg(long = "output-delimiter", value_name = "STRING")]
    #[clap(default_value = DEFAULT_DELIMITER)]
    output_delimiter: String,
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

    let shortened_lines = get_shortened_lines(&cfg, &lines, &args.output_delimiter);

    println!("{}", shortened_lines.join("\n"));
}

fn get_shortened_lines(cfg: &Config, lines: &[String], output_delimiter: &str) -> Vec<String> {
    lines
        .iter()
        .map(|l| {
            let s = shorten_line(cfg, l);
            s.join(output_delimiter)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_shortened_lines_works() {
        // GIVEN
        let data = include_str!("assets/example-paths.txt");
        let lines: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let re = Regex::new("billing|utils").unwrap();
        let cfg = Config {
            ignore_first_n: 2,
            ignore_last_n: 2,
            ignore_regex: Some(re),
            ..Config::default()
        };

        // WHEN
        let shortened_lines = get_shortened_lines(&cfg, &lines, " .. ");

        // THEN
        let expected = vec![
            "src .. main .. s .. a .. billing .. ApplicationComponents.scala",
            " .. src .. m .. s .. a .. billing .. Components.scala",
            "src .. main .. s .. a .. billing .. api .. ErrorHandler.scala",
            "src .. main .. s .. a .. billing .. a .. utils .. Authenticator.scala",
        ];
        assert_eq!(expected, shortened_lines);
    }
}
