use aoc_2020::read_aoc_input;

struct BordingPass {
  id: u32,
  seat_number: u32,
}

fn to_binary_str(input: &str) -> String {
  input
    .replace("F", "0")
    .replace("L", "0")
    .replace("B", "1")
    .replace("R", "1")
}

fn bin_2_dec(input: &str) -> u32 {
  u32::from_str_radix(&input, 2).unwrap()
}

impl BordingPass {
  fn new(seat: &str) -> BordingPass {
    let as_bin = to_binary_str(seat);
    let (row_bin, col_bin) = as_bin.split_at(7);

    BordingPass {
      id: bin_2_dec(row_bin) * 8 + bin_2_dec(col_bin),
      seat_number: bin_2_dec(&as_bin),
    }
  }
}

fn main() {
  let mut boarding_passes: Vec<BordingPass> = read_aoc_input("day5-1")
    .split("\n")
    .map(|seat| BordingPass::new(seat))
    .collect();

  let max = boarding_passes.iter().map(|p| p.id).max().unwrap();

  println!("max id is: {}", max);

  // part 2

  boarding_passes.sort_by(|a, b| a.seat_number.cmp(&b.seat_number));

  let my_seat = boarding_passes
    .iter()
    .enumerate()
    .find(|(idx, pass)| match idx {
      0 => false, // Skip seat 1 since we know it's filled
      _ => pass.seat_number - boarding_passes[idx - 1].seat_number > 1,
    })
    .map(|(_, next_seat_pass)| {
      // 0 indicates pad with zeros
      // 10 is the target width
      // b indicates to format as binary
      BordingPass::new(&format!("{:010b}", next_seat_pass.seat_number - 1))
    })
    .unwrap();

  println!("my sead ID is {}", my_seat.id);
}

#[test]
fn test_new_boarding_pass() {
  let pass1 = BordingPass::new("BFFFBBFRRR");
  assert_eq!(pass1.id, 567);

  let pass2 = BordingPass::new("FFFBBBFRRR");
  assert_eq!(pass2.id, 119);

  let pass3 = BordingPass::new("BBFFBBFRLL");
  assert_eq!(pass3.id, 820);
}
