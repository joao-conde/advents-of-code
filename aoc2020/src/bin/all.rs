use std::process::Command;

fn main() {
    println!("🎄 Advent of Code 2020 🎄\n");
    println!("Compiling all days in release mode 🛠️\n");
    (1..=25).for_each(|day| {
        Command::new("cargo").args(&["build", "--release", "--bin", &format!("day{:02}", day)]).output().unwrap();
    });
    (1..=25).for_each(|day| {
        let cmd = Command::new("cargo").args(&["run", "--release", "--bin", &format!("day{:02}", day)]).output().unwrap();
        let out = String::from_utf8(cmd.stdout).unwrap();
        println!("> Day {}\n{}", day, out);
    })
}
