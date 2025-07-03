mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

// SUCCESSES
#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--help"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    sqdj (short for squidge) shortens delimited data

    Usage: sqdj [OPTIONS]

    Options:
      -d, --delimiter <STRING>         Delimiter [default: /]
      -r, --ignore-regex <STRING>      Regex for ignoring elements (ie, they won't be shortened)
      -p, --input-path <STRING>        Input file
      -f, --ignore-first-n <NUMBER>    Ignore first n elements [default: 0]
      -l, --ignore-last-n <NUMBER>     Ignore last n elements [default: 1]
      -o, --output-delimiter <STRING>  Output delimiter [default: /]
      -s, --use-stdin                  Read input from stdin
      -h, --help                       Print help

    ----- stderr -----
    ");
}

#[test]
fn works_for_input_file() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--input-path", "tests/data/input-1.txt"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    s/m/s/a/b/ApplicationComponents.scala
    s/m/s/a/b/Components.scala
    s/m/s/a/b/Server.scala

    ----- stderr -----
    ");
}

#[test]
fn works_for_non_default_delimiter() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-2.txt",
        "--delimiter",
        "::",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    s/m/s/a/b/ApplicationComponents.scala
    s/m/s/a/b/Components.scala
    s/m/s/a/b/Server.scala

    ----- stderr -----
    ");
}

#[test]
fn handles_ignore_regex_correctly() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--ignore-regex",
        "(?:scala|billing)",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    s/m/scala/a/billing/ApplicationComponents.scala
    s/m/scala/a/billing/Components.scala
    s/m/scala/a/billing/Server.scala

    ----- stderr -----
    ");
}

#[test]
fn handles_ignore_first_n_correctly() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--ignore-first-n",
        "3",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    src/main/scala/a/b/ApplicationComponents.scala
    src/main/scala/a/b/Components.scala
    src/main/scala/a/b/Server.scala

    ----- stderr -----
    ");
}

#[test]
fn handles_ignore_last_n_correctly() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--ignore-last-n",
        "2",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    s/m/s/a/billing/ApplicationComponents.scala
    s/m/s/a/billing/Components.scala
    s/m/s/a/billing/Server.scala

    ----- stderr -----
    ");
}

#[test]
fn uses_output_delimiter_correctly() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--output-delimiter",
        "::",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    s::m::s::a::b::ApplicationComponents.scala
    s::m::s::a::b::Components.scala
    s::m::s::a::b::Server.scala

    ----- stderr -----
    ");
}

// FAILURES
#[test]
fn fails_if_no_source_is_provided() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.base_cmd();

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: a source needs to be provided (either a file or stdin)
    ");
}

#[test]
fn fails_if_more_than_one_source_is_provided() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--input-path", "tests/data/input-1.txt", "--use-stdin"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: only one source (either a file or stdin) can be used at a time
    ");
}

#[test]
fn fails_if_input_file_is_non_existent() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--input-path", "tests/data/nonexistent.txt"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: No such file or directory (os error 2)
    ");
}

#[test]
fn fails_if_input_file_is_empty() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--input-path", "tests/data/empty.txt"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: nothing to shorten
    ");
}

#[test]
fn fails_if_ignore_regex_is_incorrect() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--ignore-regex",
        "(?:scala|billing",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't compile regex

    Caused by:
        regex parse error:
            (?:scala|billing
            ^
        error: unclosed group
    ");
}

#[test]
fn fails_if_ignore_first_n_is_not_a_number() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--ignore-first-n",
        "blah",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--ignore-first-n <NUMBER>': invalid digit found in string

    For more information, try '--help'.
    ");
}

#[test]
fn fails_if_ignore_last_n_is_not_a_number() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd([
        "--input-path",
        "tests/data/input-1.txt",
        "--ignore-last-n",
        "blah",
    ]);

    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--ignore-last-n <NUMBER>': invalid digit found in string

    For more information, try '--help'.
    ");
}
