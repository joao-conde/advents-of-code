use std::{ops::RangeInclusive, process::Command};

const DAYS: RangeInclusive<usize> = 1..=24;

fn main() {
    compile_release();
    run_all();
}

fn compile_release() {
    println!("Compiling each day in release mode...");
    for day in DAYS {
        let file = format!("day{:02}", day);
        Command::new("cargo")
            .env("RUSTFLAGS", "-Awarnings")
            .arg("build")
            .arg("--release")
            .arg("--bin")
            .arg(file)
            .spawn()
            .expect("command failed to start")
            .wait()
            .expect("command wasn't running");
    }
}

fn run_all() {
    println!("Running each day...");
    for day in DAYS {
        let file = format!("day{:02}", day);
        println!("--Day {}--", day);
        Command::new(format!("./target/release/{}", file))
            .spawn()
            .expect("command failed to start")
            .wait()
            .expect("command wasn't running");
    }
}
