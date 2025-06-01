use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success();
}
