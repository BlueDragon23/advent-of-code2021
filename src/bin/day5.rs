use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::cmp;
use std::ops::RangeInclusive;

use reformation::Reformation;

#[derive(Reformation, Eq, PartialEq, Debug)]
#[reformation(r"{start} -> {end}")]
struct Line {
    start: Coordinate,
    end: Coordinate
}

#[derive(Reformation, Eq, PartialEq, Hash, Debug)]
#[reformation(r"{x},{y}")]
struct Coordinate {
    x: usize,
    y: usize
}

fn main() {
    let f = File::open("input/input5.txt").unwrap();
    let reader = BufReader::new(f);
    let vents = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Line::parse(&line).unwrap())
        .filter(|line| {
            line.start.x == line.end.x || line.start.y == line.end.y
        })
        .collect::<Vec<_>>();
    let max_x = vents.iter().map(|l| {
        cmp::max(l.start.x, l.end.x)
    }).max().unwrap();
    let max_y = vents.iter().map(|l| {
        cmp::max(l.start.y, l.end.y)
    }).max().unwrap();

    let counts = vents
        .iter()
        .fold(HashMap::new(), |mut m, l| {
            if l.start.x == l.end.x {
                // iterate column
                let x = l.start.x;
                for y in get_range(l.start.y, l.end.y) {
                    let coord = Coordinate { x, y };
                    *m.entry(coord).or_insert(0) += 1;
                }
            } else if l.start.y == l.end.y {
                // iterate row
                let y = l.start.y;
                for x in get_range(l.start.x, l.end.x) {
                    let coord = Coordinate { x, y };
                    *m.entry(coord).or_insert(0) += 1;
                }
            }
            m
        });
    let total = counts.values().filter(|&&v| v > 1).count();
    println!("{}", total);
}

fn get_range(start: usize, end: usize) -> RangeInclusive<usize> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}