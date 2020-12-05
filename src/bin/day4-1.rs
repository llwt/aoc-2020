use aoc_2020::read_aoc_input;
use itertools::Itertools;
use std::collections::HashMap;

const REQUIRED_FIELDS: &[&str] = &[
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
           // "cid", // (Country ID)
];

fn main() {
    let input = read_aoc_input("day4-1");

    let valid_passports = input
        .split("\n\n")
        .map(|p| {
            p.replace("\n", " ")
                .split_whitespace()
                .flat_map(|v| v.split(":"))
                .map(String::from)
                .tuples::<(_, _)>()
                .collect::<HashMap<_, _>>()
        })
        .filter(|passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|required_field| passport.contains_key(*required_field))
        })
        .count();

    println!("found {}", valid_passports)
}
