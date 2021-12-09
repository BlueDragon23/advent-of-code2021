use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let f = File::open("input/input9.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let height_map = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let low_points = find_low_points(&height_map);
    if part == 1 {
      println!("{}", get_risk(&height_map, &low_points));
    } else {
      println!("{}", get_largest_basins(&height_map, &low_points));
    }
}

fn get_largest_basins(height_map: &Vec<Vec<u32>>, low_points: &Vec<Coordinate>) -> usize {
  low_points.into_iter()
    .map(|&point| get_basin_size(height_map, point))
    .sorted()
    .rev()
    .take(3)
    .product()
}

fn get_basin_size(height_map: &Vec<Vec<u32>>, low_point: Coordinate) -> usize {
  let mut seen: HashSet<Coordinate> = HashSet::new();
  let mut to_be_explored: VecDeque<Coordinate> = VecDeque::new();
  let max_x = height_map.len() - 1;
  let max_y = height_map[0].len() - 1;
  to_be_explored.push_back(low_point);
  while to_be_explored.len() > 0 {
    // println!("{:?}", seen);
    let next = to_be_explored.pop_front().unwrap();
    seen.insert(next);
    let adj = get_adjacent_points(next, max_x, max_y);
    adj.into_iter()
      .filter(|c| !seen.contains(c))
      .filter(|c| height_map[c.x][c.y] != 9)
      .for_each(|c| to_be_explored.push_back(c));
  }
  seen.len()
}

fn get_adjacent_points(coordinate: Coordinate, max_x: usize, max_y: usize) -> Vec<Coordinate> {
  let mut adj = vec![];
  if coordinate.x != 0 {
    adj.push(Coordinate { x: coordinate.x - 1, y: coordinate.y });
  }
  if coordinate.x != max_x {
    adj.push(Coordinate { x: coordinate.x + 1, y: coordinate.y });
  }
  if coordinate.y != 0 {
    adj.push(Coordinate { x: coordinate.x, y: coordinate.y - 1 });
  }
  if coordinate.y != max_y {
    adj.push(Coordinate { x: coordinate.x, y: coordinate.y + 1 });
  }
  adj
}

fn find_low_points(height_map: &Vec<Vec<u32>>) -> Vec<Coordinate> {
  let mut low_points = vec![];
  for x in 0..height_map.len() {
    let row = &height_map[x];
    for y in 0..row.len() {
      let height = row[y];
      if is_low_point(height_map, height, x, y, row.len()) {
        low_points.push(Coordinate {x, y});
      }
    }
  }
  low_points
}

fn is_low_point(height_map: &Vec<Vec<u32>>, height: u32, x: usize, y: usize, row_len: usize) -> bool {
  if x == 0 {
    if y == 0 {
      return height < height_map[x][1] && height < height_map[x + 1][0];
    } else if y == row_len - 1 {
      return height < height_map[x][y - 1] && height < height_map[x + 1][y];
    } else {
      return height < height_map[x][y - 1] && height < height_map[x + 1][y] && height < height_map[x][y + 1];
    }
  } else if x == height_map.len() - 1 {
    if y == 0 {
      return height < height_map[x][1] && height < height_map[x - 1][0];
    } else if y == row_len - 1 {
      return height < height_map[x][row_len - 2] && height < height_map[x - 1][row_len - 1];
    } else {
      return height < height_map[x][y - 1] && height < height_map[x - 1][y] && height < height_map[x][y + 1];
    }
  } else if y == 0 {
    return height < height_map[x][y + 1] && height < height_map[x + 1][y] && height < height_map[x - 1][y];
  } else if y == height_map.len() - 1 {
    return height < height_map[x][y - 1] && height < height_map[x + 1][y] && height < height_map[x - 1][y];
  } else {
    return height < height_map[x][y - 1] && 
      height < height_map[x - 1][y] && 
      height < height_map[x][y + 1] &&
      height < height_map[x + 1][y];
  }
}

fn get_risk(height_map: &Vec<Vec<u32>>, low_points: &Vec<Coordinate>) -> u32 {
  low_points.into_iter().map(|coord| 1 + height_map[coord.x][coord.y]).sum()
}