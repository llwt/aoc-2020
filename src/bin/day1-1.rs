use aoc_2020::read_aoc_input;
use std::collections::HashSet;

fn main() {
    let input = read_aoc_input("day1-1");
    let numbers: HashSet<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let valid_number = numbers
        .iter()
        .find(|n| numbers.contains(&(2020 - *n)))
        .expect("no valid number found");
    let number_2 = 2020 - *valid_number;
    let product = valid_number * number_2;

    println!(
        "{0} + {1} == 2020, {0} * {1} == {2}",
        valid_number, number_2, product
    );
}
