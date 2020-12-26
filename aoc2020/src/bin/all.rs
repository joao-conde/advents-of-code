fn main() {
    println!("🎄 Advent of Code 2020 🎄\n");
    println!("Compiling 🛠️\n");
    (1..=25).for_each(|day| {
        let cmd = std::process::Command::new("cargo").args(&["run", "--release", "--bin", &format!("day{:02}", day)]).output().unwrap();
        let out = String::from_utf8(cmd.stdout).unwrap();
        println!("> Day {}\n{}", day, out);
    })
}
