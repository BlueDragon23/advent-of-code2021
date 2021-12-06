use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
  let f = File::open("input/input6.txt").unwrap();
  let reader = BufReader::new(f);
  let part = 1;
  let result = reader
      .lines()
      .next()
      .unwrap()
      .unwrap()
      .trim()
      .split(",")
      .map(|n| n.parse::<usize>().unwrap())
      .collect::<Vec<_>>();

  let fish_counts = result
    .into_iter()
    .fold(HashMap::new(), |mut m, v| {
      *m.entry(v).or_insert(0) += 1;
      m
    });
  println!("{:?}", fish_counts);
  
  let result: u64 = (0..256).fold(fish_counts, |state, _| {
    let mut next_state = HashMap::new();
    for age in 0..=8 {
      let count = *state.get(&age).unwrap_or(&0);
      if age == 0 {
        next_state.insert(6, count);
        next_state.insert(8, count);
      } else {
        *next_state.entry(age - 1).or_insert(0) += count;
      }
    }
    next_state
  }).values().sum();

  println!("{} fish", result);
}