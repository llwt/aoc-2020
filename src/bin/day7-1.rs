use aoc_2020::read_aoc_input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[allow(dead_code)]
#[derive(Debug)]
struct Bag {
  spec: String,
  contained_in: Vec<String>,
  can_contain: Vec<(String, i32)>,
}

impl Bag {
  fn new(spec: String) -> Bag {
    Bag {
      spec,
      contained_in: Vec::new(),
      can_contain: Vec::new(),
    }
  }

  fn parse_spec(&mut self, color: &str) {
    self.contained_in.push(color.to_string());
  }
}

fn main() {
  let mut bags: HashMap<String, Bag> = read_aoc_input("day7-1")
    .lines()
    .map(
      |l| match l.split(" bags contain").collect::<Vec<&str>>().as_slice() {
        [color, spec] => (color.to_string(), Bag::new(spec.to_string())),
        _ => panic!("couldn't parse bag {}"),
      },
    )
    .collect();

  // let mut bag = Bag {
  //   color: "posh teal",
  //   contains: HashMap::new(),
  // };

  // bag.contains.insert("posh teal", &bag);
  let gold_bag = bags.get_mut("shiny gold").unwrap();
  gold_bag.parse_spec("red");
  gold_bag.contained_in.push("foo".to_string());

  println!("answered {:?}", bags.get("shiny gold"));
}
