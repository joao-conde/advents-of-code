use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "input/day04";

fn main() {
    let input = fs::read_to_string(INPUT_PATH).expect("failure opening input file");
    let passports = input.split("\n\n").collect::<Vec<&str>>();
    println!("Part1: {}", p1(&passports));
    println!("Part2: {}", p2(&passports));
}

fn p1(passports: &Vec<&str>) -> usize {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports.iter().filter(|passport| keys.iter().all(|key| passport.contains(key))).count()
}

fn p2(passports: &Vec<&str>) -> usize {
    let res = [
        "byr:(19[2-9][0-9]|200[0-2])",
        "iyr:20(1[0-9]|20)",
        "eyr:20(2[0-9]|30)",
        "hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)",
        "hcl:#[0-9|a-f]{6}",
        "ecl:(amb|blu|brn|gry|grn|hzl|oth)",
        "pid:[0-9]{9}(\n| |$)",
    ]
    .iter()
    .map(|re_str| Regex::new(re_str).expect("invalid regex expression"));
    passports.iter().filter(|passport| res.clone().all(move |re| re.is_match(passport))).count()
}

#[test]
fn examples() {}

#[test]
fn puzzle() {}
