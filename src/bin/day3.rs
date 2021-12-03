use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;


fn main() {
    let f = File::open("input/input3.txt").unwrap();
    let reader = BufReader::new(f);
    let result =
        reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
    let len = 12;
    let mut mcb = vec![];
    let mut lcb = vec![];
    for i in 0..len {
        let counts = result
            .iter()
            .map(|e| e[i])
            .fold(HashMap::new(), |mut acc, c| {
                acc.insert(c, acc.get(&c).unwrap_or(&0) + 1);
                acc
            });
        if counts.get(&'1') > counts.get(&'0') {
            mcb.push('1');
            lcb.push('0');
        } else {
            mcb.push('0');
            lcb.push('1');
        }
    }

    println!("most common: {:?}", mcb.into_iter().join(""));
    // 1869
    println!("least common: {:?}", lcb.into_iter().join(""));
    // 2226
}