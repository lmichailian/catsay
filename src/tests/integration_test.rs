use assert_cmd::prelude::*;
use std::process::Command; 
use predicates::prelude::*; 

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::current_dir("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
        
    Ok(())
}