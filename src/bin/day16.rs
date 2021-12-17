use core::panic;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

const PART: u32 = 1;

struct State {
  index: usize,
  version_sum: usize
}

fn main() {
    let f = File::open("input/input16.txt").unwrap();
    let reader = BufReader::new(f);
    let line = reader.lines().next().unwrap().unwrap();
    // let line = "A0016C880162017C3686B18A3D4780";
    let input = line
        .chars()
        .map(|c| {
            format!("{:04b}", usize::from_str_radix(&c.to_string(), 16).unwrap())
                .chars()
                .collect_vec()
        })
        .flatten()
        .collect_vec();
    let (final_state, result) = parse_packet(&input, State { index: 0, version_sum: 0 });
    println!("Part 1: {}", final_state.version_sum);
    println!("Part 2: {}", result);
}

fn parse_packet(line: &Vec<char>, mut state: State) -> (State, usize) {
    let version = convert_from_binary(read_n_bits(line, state.index, 3));
    state.version_sum += version;
    state.index += 3;
    let type_id = convert_from_binary(read_n_bits(line, state.index, 3));
    state.index += 3;
    match type_id {
      4 => parse_literal(line, state),
      op => parse_operator(line, state, op)
    }
}

fn parse_literal(line: &Vec<char>, mut state: State) -> (State, usize) {
  let mut chunk = read_n_bits(line, state.index, 5);
  let mut chunks = vec![chunk[1..5].to_string()];
  state.index += 5;
  while chunk.chars().next().unwrap() == '1' {
    chunk = read_n_bits(line, state.index, 5);
    chunks.push(chunk[1..5].to_string());
    state.index += 5;
  }
  let literal = convert_from_binary(chunks.join(""));
  (state, literal)
}

fn parse_operator(line: &Vec<char>, mut state: State, operator: usize) -> (State, usize) {
  let length_type_id = line[state.index];
  state.index += 1;
  let (new_state, subpackets) = match length_type_id {
    '0' => {
      let bit_length = convert_from_binary(read_n_bits(line, state.index, 15));
      state.index += 15;
      let target_index = state.index + bit_length;
      let mut results = vec![];
      while state.index != target_index {
        let output = parse_packet(line, state);
        state = output.0;
        results.push(output.1);
      }
      (state, results)
    },
    '1' => {
      let packet_length = convert_from_binary(read_n_bits(line, state.index, 11));
      state.index += 11;
      let mut results = vec![];
      for _ in 0..packet_length {
        let output = parse_packet(line, state);
        state = output.0;
        results.push(output.1);
      }
      (state, results)
    },
    _ => panic!("Oopsy woopsy")
  };
  let result = match operator {
    0 => subpackets.into_iter().sum(),
    1 => subpackets.into_iter().product(),
    2 => subpackets.into_iter().min().unwrap(),
    3 => subpackets.into_iter().max().unwrap(),
    5 => if subpackets[0] > subpackets[1] { 1 } else { 0 },
    6 => if subpackets[0] < subpackets[1] { 1 } else { 0 },
    7 => if subpackets[0] == subpackets[1] { 1 } else { 0 },
    _ => panic!("Naughty operator")
  };
  (new_state, result)
}

fn convert_from_binary(text: String) -> usize {
    usize::from_str_radix(&text, 2).unwrap()
}

fn read_n_bits(line: &Vec<char>, start: usize, length: usize) -> String {
    line[start..start + length].into_iter().join("")
}
