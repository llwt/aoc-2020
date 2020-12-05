use aoc_2020::{is_int_between, is_valid_hex_color, read_aoc_input};
use itertools::Itertools;
use phf::phf_map;
use std::collections::HashMap;

type Validator = fn(&str) -> bool;

fn is_valid_height(v: &str) -> bool {
    match v.split_at(v.len() - 2) {
        (height, "cm") => is_int_between(height, 150, 193),
        (height, "in") => is_int_between(height, 59, 76),
        _ => false,
    }
}

const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

// TODO: is it better to just use lazy_static for this?
static VALIDATORS: phf::Map<&'static str, Validator> = phf_map! {
    // TODO: is there a way to curry or return closures from is_number_between?
    "byr" => (|v| is_int_between(v, 1920, 2002)),
    "iyr" => (|v| is_int_between(v, 2010, 2020)),
    "eyr" => (|v| is_int_between(v, 2020, 2030)),
    "hgt" => (is_valid_height),
    "hcl" => (is_valid_hex_color),
    "ecl" => (|v| VALID_EYE_COLORS.contains(&v)),
    "pid" => (|v| v.len() == 9 && is_int_between(v, 0, 999999999))
};

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
            VALIDATORS
                .entries()
                .all(|(key, validator)| match passport.get(*key) {
                    Some(value) => validator(value),
                    None => false,
                })
        })
        .count();

    println!("found {}", valid_passports)
}

#[test]
fn test_is_valid_height() {
    assert_eq!(is_valid_height("abc"), false);
    assert_eq!(is_valid_height("123cm"), false);
    assert_eq!(is_valid_height("170cm"), true);
    assert_eq!(is_valid_height("270cm"), false);
    assert_eq!(is_valid_height("1in"), false);
    assert_eq!(is_valid_height("70in"), true);
    assert_eq!(is_valid_height("100in"), false);
}
