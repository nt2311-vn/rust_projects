use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{process::Command, result::Result};

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}
