use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn prints_hello() {
    // Using .unwrap() here can be dangerous if the
    // input values aren't what you expect.
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn dies_if_no_args() -> Result<()> {
    // We'll replace .unwrap() with the `?` operator,
    // to either unpack an Ok() value or propagate the Err()
    // value to the return type.
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    let expected = fs::read_to_string("tests/expected/hello1.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}
