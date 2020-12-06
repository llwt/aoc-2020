use aoc_2020::read_aoc_input;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
  let all_answers = HashSet::<char>::from_iter("abcdefghijklmnopqrstuvwxyz".chars());

  let answered = read_aoc_input("day6-1")
    .split("\n\n")
    .map(|e| {
      e.lines()
        .map(|l| HashSet::<char>::from_iter(l.chars()))
        .fold(all_answers.iter().cloned().collect(), |a, l| {
          l.intersection(&a).cloned().collect()
        })
    })
    .fold(0, |sum, e| sum + e.len());

  println!("answered {}", answered)
}
