#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fs;

pub fn read_aoc_input(name: &str) -> String {
  let input_path = format!("inputs/{}", name);

  return fs::read_to_string(&input_path).expect(&format!("Could not find input {}", &input_path));
}

pub fn is_int_between(v: &str, min: i32, max: i32) -> bool {
  v.parse::<i32>().map_or(false, |n| n >= min && n <= max)
}

#[test]
fn test_is_int_between() {
  assert_eq!(is_int_between("42", 41, 43), true);
  assert_eq!(is_int_between("41", 41, 43), true);
  assert_eq!(is_int_between("43", 41, 43), true);
  assert_eq!(is_int_between("44", 41, 43), false);
  assert_eq!(is_int_between("40", 41, 43), false);
  assert_eq!(is_int_between("abc", 41, 43), false);
}

pub fn is_valid_hex_color(v: &str) -> bool {
  lazy_static! {
    static ref IS_VALID_COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  }

  IS_VALID_COLOR.is_match(v)
}

#[test]
fn test_is_valid_hex_color() {
  assert_eq!(is_valid_hex_color("#aaaaaa"), true);
  assert_eq!(is_valid_hex_color("#123abc"), true);
  assert_eq!(is_valid_hex_color("#aaaaag"), false);
  assert_eq!(is_valid_hex_color("#aaaa"), false);
  assert_eq!(is_valid_hex_color("aaaaaa"), false);
}

