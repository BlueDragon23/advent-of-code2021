use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;
use reformation::Reformation;

const PART: u32 = 2;

#[derive(Reformation)]
#[reformation(r"{left_one}{left_two} -> {right}")]
struct Rule {
  left_one: char,
  left_two: char,
  right: char
}

fn main() {
    let f = File::open("input/input14.txt").unwrap();
    let reader = BufReader::new(f);
    let mut iterator = reader
        .lines()
        .map(|line| line.unwrap());
    let state = iterator.next().unwrap();
    iterator.next();
    let rules_struct = iterator
      .map(|line| Rule::parse(&line).unwrap())
      .collect_vec();
    let rules = rules_struct
      .into_iter()
      .fold(HashMap::new(), |mut m, rule| {
        m.insert((rule.left_one, rule.left_two), rule.right);
        m
      });
    let steps = if PART == 1 { 10 } else { 40 };
    let final_state = (0..steps).fold(state, |s, _| step(s, &rules));
    // println!("{}", final_state);
    let char_counts = final_state
      .chars()
      .fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0_u128) += 1;
        m
      });
    let max = char_counts.clone().into_iter().max_by_key(|e| e.1).unwrap().1;
    let min = char_counts.into_iter().min_by_key(|e| e.1).unwrap().1;
    println!("{} - {}", max, min);
    println!("result: {}", max - min);
}

fn step(state: String, rules: &HashMap<(char, char), char>) -> String {
  state.chars().tuple_windows().map(|(a, b)| {
    let insert = rules[&(a, b)];
    (a, insert, b)
  })
  .fold(("".to_string(), 0), |(mut acc, count), (a, insert, b)| {
    if count == 0 {
      acc.extend([a, insert, b]);
    } else {
      acc.extend([insert, b]);
    }
    (acc, count + 1)
  }).0
}

fn step_better(state: &HashMap<(char, char), u128>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), u128> {
  state.into_iter().map(|(pair, count)| {
    let insert = rules[pair];
    ((pair.0, insert), (insert, pair.1), count)
  })
  .fold(HashMap::new(), |mut m, (pair1, pair2, count)| {
    *m.entry(pair1).or_insert(0) += count;
    *m.entry(pair2).or_insert(0) += count;
    m
  })
}

// start with  nn, nc, cb
// creates nc, cn, nb, nc, ch, hb