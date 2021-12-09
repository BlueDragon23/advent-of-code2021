use advent_of_code2021::{get_adjacent_points, Coordinate};

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/input9.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let height_map = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let low_points = find_low_points(&height_map);
    if part == 1 {
        println!("{}", get_risk(&height_map, &low_points));
    } else {
        println!("{}", get_largest_basins(&height_map, &low_points));
    }
}

fn get_largest_basins(height_map: &Vec<Vec<u32>>, low_points: &Vec<Coordinate>) -> usize {
    low_points
        .into_iter()
        .map(|&point| get_basin_size(height_map, point))
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn get_basin_size(height_map: &Vec<Vec<u32>>, low_point: Coordinate) -> usize {
    let mut seen: HashSet<Coordinate> = HashSet::new();
    let mut to_be_explored: VecDeque<Coordinate> = VecDeque::new();
    let row_count = height_map.len();
    let col_count = height_map[0].len();
    to_be_explored.push_back(low_point);
    while to_be_explored.len() > 0 {
        let next = to_be_explored.pop_front().unwrap();
        seen.insert(next);
        let adj = get_adjacent_points(next, row_count, col_count);
        adj.into_iter()
            .filter(|c| !seen.contains(c))
            .filter(|c| height_map[c.row][c.col] != 9)
            .for_each(|c| to_be_explored.push_back(c));
    }
    seen.len()
}

fn find_low_points(height_map: &Vec<Vec<u32>>) -> Vec<Coordinate> {
    let mut low_points = vec![];
    for row_num in 0..height_map.len() {
        let row = &height_map[row_num];
        for col_num in 0..row.len() {
            let coord = Coordinate {
                row: row_num,
                col: col_num,
            };
            if is_low_point(height_map, coord, row.len()) {
                low_points.push(coord);
            }
        }
    }
    low_points
}

fn is_low_point(height_map: &Vec<Vec<u32>>, coord: Coordinate, row_len: usize) -> bool {
    let adj = get_adjacent_points(coord, height_map.len(), row_len);
    adj.into_iter()
        .all(|c| height_map[coord.row][coord.col] < height_map[c.row][c.col])
}

fn get_risk(height_map: &Vec<Vec<u32>>, low_points: &Vec<Coordinate>) -> u32 {
    low_points
        .into_iter()
        .map(|coord| 1 + height_map[coord.row][coord.col])
        .sum()
}
