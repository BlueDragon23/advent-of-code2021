use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

type Graph = HashMap<String, Vec<String>>;

const PART: u32 = 2;

fn main() {
    let f = File::open("input/input12.txt").unwrap();
    let reader = BufReader::new(f);
    let graph = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(HashMap::new(), |g, line| add_edge(line, g));
    println!("{:?}", graph);
    println!("{}", find_paths(&graph, &"start".to_string(), HashSet::new(), false, 0, Vec::new()));
}

fn add_edge(line: String, mut graph: Graph) -> Graph {
  let (n1, n2) = line.split_once("-").unwrap();
  graph.entry(n1.to_string()).or_insert(Vec::new()).push(n2.to_string());
  graph.entry(n2.to_string()).or_insert(Vec::new()).push(n1.to_string());
  graph
}

fn find_paths(graph: &Graph, start_node: &String, mut seen: HashSet<String>, mut seen_twice: bool,
   depth: u32, mut visited: Vec<String>) -> u64 {
  // println!("depth: {}: {}", depth, start_node);
  visited.push(start_node.to_string());
  if is_lowercase(start_node) {
    if seen.contains(start_node) {
      if !seen_twice {
        seen_twice = true;
      }
    } else {
      seen.insert(start_node.to_string());
    }
  }
  let adjacent = graph.get(start_node).unwrap();
  let to_visit = adjacent.into_iter().filter(|&n| {
    if n == "start" {
      return false;
    }
    if (PART == 1 || seen_twice) && seen.contains(n) {
      return false;
    }
    true
  }).collect_vec();
  let mut acc = 0;
  for n in to_visit {
    if n == "end" {
      // println!("Found path {:?}", visited.clone().into_iter().chain(vec!["end".to_string()].into_iter()).join(","));
      acc += 1;
      continue;
    }
    // println!("Recursing");
    acc += find_paths(graph, n, seen.clone(), seen_twice.clone(), depth + 1, visited.clone());
  }
  // println!("Returning {}", acc);
  acc
}

fn is_lowercase(node: &String) -> bool {
  node.chars().all(|c| c.is_lowercase())
}