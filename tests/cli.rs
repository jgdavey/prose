use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::PredicateBooleanExt;

fn prose_cmd() -> assert_cmd::Command {
    cargo_bin_cmd!("prose")
}

#[test]
fn test_stdin_default_width() {
    prose_cmd()
        .write_stdin("We the people of the United States, in order to form a more perfect union, establish justice, insure domestic tranquility, provide for the common defense, promote the general welfare, and secure the blessing of liberty to ourselves and our posterity, do ordain and establish the Constitution of the United States of America.")
        .assert()
        .success()
        .stdout(predicates::str::contains("\n"));
}

#[test]
fn test_file_input() {
    prose_cmd()
        .arg("tests/data/inputs/plain.txt")
        .assert()
        .success()
        .stdout(predicates::str::is_empty().not());
}

#[test]
fn test_width_flag() {
    prose_cmd()
        .args(["-w", "30"])
        .write_stdin("We the people of the United States, in order to form a more perfect union.")
        .assert()
        .success()
        .stdout("We the people of the United\nStates, in order to form a\nmore perfect union.\n");
}

#[test]
fn test_markdown_mode() {
    prose_cmd()
        .args(["-m", "-w", "40"])
        .write_stdin("# Heading\n\nThis is a long paragraph that should be reformatted to fit within the target width.")
        .assert()
        .success()
        .stdout(predicates::str::starts_with("# Heading\n"));
}

#[test]
fn test_code_comments_mode() {
    prose_cmd()
        .args(["-c", "-w", "40"])
        .write_stdin(
            "// This is a long comment that should be reformatted to fit within the target width.",
        )
        .assert()
        .success()
        .stdout(predicates::str::starts_with("// "));
}

#[test]
fn test_fit_flag() {
    prose_cmd()
        .args(["-f", "-w", "50"])
        .write_stdin("We the people of the United States, in order to form a more perfect union, establish justice.")
        .assert()
        .success()
        .stdout(predicates::str::contains("\n"));
}

#[test]
fn test_version_flag() {
    prose_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicates::str::starts_with("prose "));
}

#[test]
fn test_help_flag() {
    prose_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage"));
}

#[test]
fn test_nonexistent_file() {
    prose_cmd()
        .arg("nonexistent_file.txt")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error opening"));
}

#[test]
fn test_empty_stdin() {
    prose_cmd().write_stdin("").assert().success().stdout("\n");
}
