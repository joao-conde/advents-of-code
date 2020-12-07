use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/day04").expect("failure opening input file");
    let passports = input.split("\n\n").collect::<Vec<&str>>();
    println!("Part1: {}", p1(&passports));
    println!("Part2: {}", p2(&passports));
}

fn p1(passports: &[&str]) -> usize {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports.iter().filter(|passport| keys.iter().all(|key| passport.contains(key))).count()
}

fn p2(passports: &[&str]) -> usize {
    let regexes = [
        r"byr:(?:19[2-9][0-9]|200[0-2])",
        r"iyr:20(?:1[0-9]|20)",
        r"eyr:20(?:2[0-9]|30)",
        r"hgt:(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)",
        r"hcl:#[0-9|a-f]{6}",
        r"ecl:(?:amb|blu|brn|gry|grn|hzl|oth)",
        r"pid:[0-9]{9}(?:\n| |$)",
    ]
    .iter()
    .map(|re_str| Regex::new(re_str).expect("invalid regex expression"));
    passports.iter().filter(|passport| regexes.clone().all(move |re| re.is_match(passport))).count()
}
