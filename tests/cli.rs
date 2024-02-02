use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn prints_arch_news() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("pacnews")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Title"));

    Ok(())
}
