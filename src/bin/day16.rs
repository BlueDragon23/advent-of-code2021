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
    let final_state = parse_packet(&input, State { index: 0, version_sum: 0 });
    println!("Part 1: {}", final_state.version_sum);
}

fn parse_packet(line: &Vec<char>, mut state: State) -> State {
    let version = convert_from_binary(read_n_bits(line, state.index, 3));
    state.version_sum += version;
    state.index += 3;
    let type_id = convert_from_binary(read_n_bits(line, state.index, 3));
    state.index += 3;
    if type_id == 4 {
        state = parse_literal(line, state);
    } else {
        state = parse_operator(line, state);
    }
    state
}

fn parse_literal(line: &Vec<char>, mut state: State) -> State {
  let mut chunk = read_n_bits(line, state.index, 5);
  state.index += 5;
  while chunk.chars().next().unwrap() == '1' {
    chunk = read_n_bits(line, state.index, 5);
    state.index += 5;
  }
  state
}

fn parse_operator(line: &Vec<char>, mut state: State) -> State {
  let length_type_id = line[state.index];
  state.index += 1;
  match length_type_id {
    '0' => {
      let bit_length = convert_from_binary(read_n_bits(line, state.index, 15));
      state.index += 15;
      let target_index = state.index + bit_length;
      while state.index != target_index {
        state = parse_packet(line, state);
      }
      state
    },
    '1' => {
      let packet_length = convert_from_binary(read_n_bits(line, state.index, 11));
      state.index += 11;
      for _ in 0..packet_length {
        state = parse_packet(line, state);
      }
      state
    },
    _ => panic!("Oopsy woopsy")
  }
}

fn convert_from_binary(text: String) -> usize {
    usize::from_str_radix(&text, 2).unwrap()
}

fn read_n_bits(line: &Vec<char>, start: usize, length: usize) -> String {
    line[start..start + length].into_iter().join("")
}
