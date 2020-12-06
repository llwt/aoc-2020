use aoc_2020::read_aoc_input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
  let answered = read_aoc_input("day6-1")
    .split("\n\n")
    .map(|e| e.replace("\n", "").replace(" ", ""))
    .map(|e| HashSet::<char>::from_iter(e.chars()))
    .fold(0, |sum, e| sum + e.len());

  println!("answered {}", answered)
}
