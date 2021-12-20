use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;
use reformation::Reformation;

const PART: u32 = 2;

#[derive(Reformation, Debug, Clone, Copy)]
enum Line {
    #[reformation(r"{}")]
    Coordinate(Coordinate),
    #[reformation(r"{}")]
    Instruction(Instruction),
}

#[derive(Reformation, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[reformation(r"{x},{y}")]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Reformation, Debug, Clone, Copy)]
enum Instruction {
    #[reformation(r"fold along y={}")]
    Horizontal(u32),
    #[reformation(r"fold along x={}")]
    Vertical(u32),
}

fn main() {
    let f = File::open("input/input13.txt").unwrap();
    let reader = BufReader::new(f);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.trim().is_empty())
        .map(|line| Line::parse(&line).unwrap())
        .collect_vec();
    let coordinates = lines
        .clone()
        .into_iter()
        .filter_map(|line| match line {
            Line::Coordinate(c) => Some(c),
            _ => None,
        })
        .collect_vec();
    let instructions = lines
        .into_iter()
        .filter_map(|line| match line {
            Line::Instruction(i) => Some(i),
            _ => None,
        })
        .collect_vec();

    let final_coords = instructions
        .into_iter()
        .take(if PART == 1 { 1 } else { 20 })
        .fold(coordinates, |coords, instruction| {
            fold_paper(&coords, instruction)
        });
    if PART == 1 {
        println!("{}", final_coords.len());
    } else {
        print_coordinates(&final_coords);
    }
}

fn fold_paper(coordinates: &Vec<Coordinate>, instruction: Instruction) -> Vec<Coordinate> {
    let new_coordinates = match instruction {
        Instruction::Horizontal(line) => {
            let mut above = coordinates
                .into_iter()
                .filter(|&c| c.y < line)
                .map(|&c| c)
                .collect_vec();
            let below = coordinates.into_iter().filter(|c| c.y > line).collect_vec();
            above.extend(below.into_iter().map(|c| Coordinate {
                x: c.x,
                y: line - (c.y - line),
            }));
            above
        }
        Instruction::Vertical(line) => {
            let mut left = coordinates
                .into_iter()
                .filter(|&c| c.x < line)
                .map(|&c| c)
                .collect_vec();
            let right = coordinates.into_iter().filter(|c| c.x > line).collect_vec();
            left.extend(right.into_iter().map(|c| Coordinate {
                x: line - (c.x - line),
                y: c.y,
            }));
            left
        }
    };
    new_coordinates.into_iter().unique().collect_vec()
}

fn print_coordinates(coordinates: &Vec<Coordinate>) {
    let row_count = coordinates.into_iter().map(|c| c.y).max().unwrap();
    let col_count = coordinates.into_iter().map(|c| c.x).max().unwrap();
    let set: HashSet<Coordinate> = HashSet::from_iter(coordinates.iter().map(|&c| c));
    for y in 0..=row_count {
        for x in 0..=col_count {
            let coord = Coordinate { x, y };
            print!("{}", if set.contains(&coord) { "#" } else { "." });
        }
        println!("");
    }
}
