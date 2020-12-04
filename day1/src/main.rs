use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
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
