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
  
  let result = (0..80).fold(result, |state, _| {
    let mut new_fish = 0;
    let mut next_state = state.into_iter().map(|f| {
      if f == 0 {
        new_fish += 1;
        6
      } else {
        f - 1
      }
    }).collect::<Vec<_>>();
    for _ in 0..new_fish {
      next_state.push(8);
    }
    next_state
  }).len();

  println!("{} fish", result);
}