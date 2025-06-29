use assert_cmd::Command;

// SUCCESSES
#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("sqdj (short for squidge) shortens delimited data"));
}

#[test]
fn works_for_input_file() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"s/m/s/a/b/ApplicationComponents.scala
s/m/s/a/b/Components.scala
s/m/s/a/b/Server.scala
"#;
    assert_eq!(stdout, expected);
}

#[test]
fn works_for_non_default_delimiter() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-2.txt");
    cmd.arg("-d=::");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"s/m/s/a/b/ApplicationComponents.scala
s/m/s/a/b/Components.scala
s/m/s/a/b/Server.scala
"#;
    assert_eq!(stdout, expected);
}

#[test]
fn handles_ignore_regex_correctly() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-r=(?:scala|billing)");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"s/m/scala/a/billing/ApplicationComponents.scala
s/m/scala/a/billing/Components.scala
s/m/scala/a/billing/Server.scala
"#;
    assert_eq!(stdout, expected);
}

#[test]
fn handles_ignore_first_n_correctly() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-f=3");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"src/main/scala/a/b/ApplicationComponents.scala
src/main/scala/a/b/Components.scala
src/main/scala/a/b/Server.scala
"#;
    assert_eq!(stdout, expected);
}

#[test]
fn handles_ignore_last_n_correctly() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-l=2");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"s/m/s/a/billing/ApplicationComponents.scala
s/m/s/a/billing/Components.scala
s/m/s/a/billing/Server.scala
"#;
    assert_eq!(stdout, expected);
}

#[test]
fn uses_output_delimiter_correctly() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-o=::");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"s::m::s::a::b::ApplicationComponents.scala
s::m::s::a::b::Components.scala
s::m::s::a::b::Server.scala
"#;
    assert_eq!(stdout, expected);
}

// FAILURES
#[test]
fn fails_if_no_source_is_provided() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_more_than_one_source_is_provided() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-s");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_input_file_is_non_existent() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("No such file or directory"));
}

#[test]
fn fails_if_input_file_is_empty() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/empty.txt");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("nothing to shorten"));
}

#[test]
fn fails_if_ignore_regex_is_incorrect() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-r=(?:scala|billing");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("couldn't compile regex"));
}

#[test]
fn fails_if_ignore_first_n_is_not_a_number() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-f=blah");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_ignore_last_n_is_not_a_number() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-l=blah");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{stdout}");
    }
    assert!(!output.status.success());
}
