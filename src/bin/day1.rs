use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/input1.txt").unwrap();
    let reader = BufReader::new(f);
    let result: i32 = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i32>())
        .tuple_windows()
        .map(|(a, b, c)| a.unwrap() + b.unwrap() + c.unwrap())
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();

    println!("{:?}", result);
}
