use anyhow::Context;
use clap::Parser;
use regex::Regex;
use squidge::{shorten_line, Config};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    /// Input file
    #[arg(short = 'p', long = "input-path", value_name = "STRING")]
    input_file_path: Option<String>,
    /// Ignore first n elements
    #[arg(short = 'f', long = "ignore-first-n", value_name = "NUMBER")]
    #[clap(default_value_t = DEFAULT_IGNORE_FIRST_N)]
    ignore_first_n: usize,
    /// Ignore last n elements
    #[arg(short = 'l', long = "ignore-last-n", value_name = "NUMBER")]
    #[clap(default_value_t = DEFAULT_IGNORE_LAST_N)]
    ignore_last_n: usize,
    /// Output delimiter
    #[arg(short = 'o', long = "output-delimiter", value_name = "STRING")]
    #[clap(default_value = DEFAULT_DELIMITER)]
    output_delimiter: String,
    /// Read input from stdin
    #[arg(short = 's', long = "use-stdin", value_name = "BOOLEAN")]
    use_stdin: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let re = args
        .ignore_regex
        .map(|r| Regex::new(&r).context("couldn't compile regex"))
        .transpose()?;

    let cfg = Config {
        delimiter: &args.delimiter,
        ignore_first_n: args.ignore_first_n,
        ignore_last_n: args.ignore_last_n,
        ignore_regex: re,
    };

    let lines = match (args.use_stdin, args.input_file_path) {
        (false, None) => {
            return Err(anyhow::anyhow!(
                "a source needs to be provided (either a file or stdin)"
            ));
        }
        (true, Some(_)) => {
            return Err(anyhow::anyhow!(
                "only one source (either a file or stdin) can be used at a time"
            ));
        }
        (true, None) => {
            let stdin = io::stdin();
            let mut lines = vec![];
            for line in stdin.lock().lines() {
                let line = line.context("couldn't read line from stdin")?;
                lines.push(line);
            }
            lines
        }
        (false, Some(path)) => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            reader
                .lines()
                .map(|line| line.context("couldn't read line from file"))
                .collect::<Result<_, _>>()?
        }
    };

    if lines.is_empty() {
        return Err(anyhow::anyhow!("nothing to shorten"));
    }

    let shortened_lines = get_shortened_lines(&cfg, &lines, &args.output_delimiter);

    println!("{}", shortened_lines.join("\n"));

    Ok(())
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
