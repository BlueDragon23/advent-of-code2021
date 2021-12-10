use std::io::Lines;
use std::fs::File;
use std::io::BufReader;

use itertools::Itertools;
use reformation::Reformation;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

// Example union input
#[derive(Reformation, Eq, PartialEq, Debug)]
enum Ant {
    #[reformation(r"Queen\({}\)")]
    Queen(String),
    #[reformation(r"Worker\({}\)")]
    Worker(i32),
    #[reformation(r"Warrior")]
    Warrior,
}

// Example struct input
#[derive(Reformation, Debug)]
#[reformation(r"{year}-{month}-{day} {hour}:{minute}")]
#[allow(dead_code)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

// Create a method for parsing a line of ints
pub fn parse_line_to_num(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec()
}

// Create a method for parsing lines of a file to ints
pub fn parse_lines_to_nums(lines: Lines<BufReader<File>>) -> Vec<i32> {
    lines.map(|line| line.unwrap().parse::<i32>().unwrap()).collect_vec()
}

// fn parse_dates(reader: BufReader<File>) -> Vec<Date> {
//     parse_lines_to_struct::<Date>(reader)
// }

// Create a method for parsing lines of a file to a particular struct using reformation
// pub fn parse_lines_to_struct<'a, T: Reformation<'a>>(reader: BufReader<File>) -> Vec<T> {
//     reader
//         .lines()
//         .map(|line| T::parse(&line.unwrap()).unwrap())
//         .collect_vec()
// }

pub fn get_adjacent_points(
    coordinate: Coordinate,
    row_count: usize,
    col_count: usize,
) -> Vec<Coordinate> {
    let mut adj = vec![];
    if coordinate.row != 0 {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col,
        });
    }
    if coordinate.row != row_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col,
        });
    }
    if coordinate.col != 0 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col - 1,
        });
    }
    if coordinate.col != col_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col + 1,
        });
    }
    adj
}
