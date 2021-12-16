use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use advent_of_code2021::get_adjacent_points;
use advent_of_code2021::Coordinate;
use itertools::Itertools;

const PART: u32 = 2;

fn main() {
    let f = File::open("input/input15.txt").unwrap();
    let reader = BufReader::new(f);
    let cave = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let result = find_path(&cave);
    println!("Result: {}", result);
}

fn find_path(cave: &Vec<Vec<u32>>) -> u32 {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let row_count = if PART == 1 {
        cave.len()
    } else {
        cave.len() * 5
    };
    let col_count = if PART == 1 {
        cave[0].len()
    } else {
        cave[0].len() * 5
    };
    let mut current = Coordinate { row: 0, col: 0 };
    let mut current_distance = 0;
    distances.insert(current, 0);
    visited.insert(current);
    loop {
        get_adjacent_points(current, row_count, col_count)
            .into_iter()
            .for_each(|c| {
                let new_distance = current_distance + get_risk(cave, c);
                let stored_distance = distances.entry(c).or_insert(1_000_000);
                if new_distance < *stored_distance {
                    *stored_distance = new_distance;
                }
            });
        current = find_next_node(&distances, &visited);
        current_distance = *distances.get(&current).unwrap();
        visited.insert(current);
        if visited.len() % 10000 == 0 {
            println!("Visited {}/{} nodes", visited.len(), row_count * col_count);
        }
        if current.row == row_count - 1 && current.col == col_count - 1 {
            // found the target
            break current_distance;
        }
    }
}

fn find_next_node(
    distances: &HashMap<Coordinate, u32>,
    visited: &HashSet<Coordinate>,
) -> Coordinate {
    distances
        .into_iter()
        .filter(|(c, _)| !visited.contains(c))
        .min_by_key(|(_, &d)| d)
        .map(|(&c, _)| c)
        .unwrap()
}

fn get_risk(cave: &Vec<Vec<u32>>, coordinate: Coordinate) -> u32 {
    let row_count = cave.len();
    let col_count = cave[0].len();
    let base_coordinate_row = coordinate.row % row_count;
    let base_coordinate_col = coordinate.col % col_count;
    let initial = cave[base_coordinate_row][base_coordinate_col];
    let diff = (coordinate.row / row_count) + (coordinate.col / col_count);
    let mut result = initial + (diff as u32);
    if result >= 10 {
        result -= 9;
    }
    // println!("row:{},col:{},base_row:{},base_col:{},base_val:{},diff:{},final:{}",
    // coordinate.row, coordinate.col, base_coordinate_row, base_coordinate_col, initial, diff, result);
    result
}
