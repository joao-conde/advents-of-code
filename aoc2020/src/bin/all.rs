use std::{ops::RangeInclusive, process::Command};

const DAYS: RangeInclusive<usize> = 1..=25;

fn main() {
    println!("| Advent of Code 2020 |\n");
    println!("Compiling...\n");
    DAYS.for_each(|day| {
        let cmd = Command::new("cargo").args(&["run", "--release", "--bin", &format!("day{:02}", day)]).output().unwrap();
        let out = String::from_utf8(cmd.stdout).unwrap();
        println!("> Day {}\n{}", day, out);
    })
}
