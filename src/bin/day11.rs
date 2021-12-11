use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;
use advent_of_code2021::{Coordinate, get_adjacent_points_diagonal, print_matrix};

const SIZE: usize = 10;

fn main() {
    let f = File::open("input/input11.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let mut state = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let mut flashes = 0;
    let step_count = if part == 1 { 100 } else { 1000 };
    for step_number in 0..step_count {
      let (next_state, step_flash) = step(&state);
      state = next_state;
      flashes += step_flash;
      if part == 2 && step_flash == 100 {
        println!("All flashed after step {}", step_number + 1);
        break;
      }
    }
    println!("Total flashes {}", flashes);
}

fn step(state: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, u64) {
  let mut new_state = state.clone();
  // Increase all
  for row in 0..SIZE {
    for col in 0..SIZE {
      new_state[row][col] += 1
    }
  }
  // Flash
  let mut to_be_flashed = VecDeque::new();
  for row in 0..SIZE {
    for col in 0..SIZE {
      if new_state[row][col] > 9 {
        to_be_flashed.push_back(Coordinate { row, col });
      }
    }
  }
  // println!("to be flashed immediately {:?}", to_be_flashed);
  
  let mut flashed = HashSet::new();
  while to_be_flashed.len() > 0 {
    let coord = to_be_flashed.pop_front().unwrap();
    // println!("flashing {:?}", coord);
    let adj = get_adjacent_points_diagonal(coord, SIZE, SIZE);
    adj.into_iter().for_each(|c| {
      new_state[c.row][c.col] += 1;
      if new_state[c.row][c.col] > 9 && !flashed.contains(&c) && !to_be_flashed.contains(&c) {
        to_be_flashed.push_back(c);
        // println!("adding {:?} to be flashed", c);
      }
    });
    flashed.insert(coord);
  }
  let size = flashed.len();
  flashed.into_iter().for_each(|c| {
    new_state[c.row][c.col] = 0;
  });
  // print_matrix(&new_state);
  (new_state, size as u64)
}
