use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/input2.txt").unwrap();
    let reader = BufReader::new(f);
    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .fold((0, 0, 0), |acc, line| {
            let mut parts = line.split(" ");
            let direction = parts.next().unwrap();
            let length: i32 = parts.next().unwrap().parse().unwrap();
            match direction {
                "forward" => (acc.0 + length, acc.1 + length * acc.2, acc.2),
                "down" => (acc.0, acc.1, acc.2 + length),
                "up" => (acc.0, acc.1, acc.2 - length),
                _ => panic!(""),
            }
        });
    println!("{}", result.0 * result.1);
}

// let result = reader
// .lines()
// .map(|line| line.unwrap())
// .fold((0, 0), |acc, line| {
//     let mut parts = line.split(" ");
//     let direction = parts.next().unwrap();
//     let length: i32 = parts.next().unwrap().parse().unwrap();
//     match direction {
//         "forward" => (acc.0 + length, acc.1),
//         "down" => (acc.0, acc.1 + length),
//         "up" => (acc.0, acc.1 - length),
//         _ => panic!("")
//     }
// });
