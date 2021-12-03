use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;


fn main() {
    let f = File::open("input/input3.txt").unwrap();
    let reader = BufReader::new(f);
    let mut o2 =
        reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
    let mut co2 = o2.clone();
    let len = 12;
    let mut mcb: Vec<char> = vec![];
    let mut lcb: Vec<char> = vec![];
    let part = 2;
    for i in 0..len {
        // mcb.push(most_common);
        // lcb.push(if most_common == '1' { '0' } else { '1' });
        if part == 2 {
            // let most_common_o2 = get_most_common_bit(&o2, i);
            // o2 = o2.into_iter().filter(|chars| {
            //     chars[i] == most_common_o2 || most_common_o2 == '2' && chars[i] == '1'
            // }).collect();
            // if o2.len() == 1 {
            //     break;
            // }
            if co2.len() == 1 {
                break;
            }

            let most_common_co2 = get_most_common_bit(&co2, i);
            let least_common_co2 = if most_common_co2 == '1' { '0' } else if most_common_co2 == '0' { '1' } else { '2' };
            co2 = co2.into_iter().filter(|chars| {
                chars[i] == least_common_co2 || least_common_co2 == '2' && chars[i] == '0'
            }).collect();

        }
    }
    // println!("o2: {:?}", o2.first().unwrap().into_iter().join(""));
    // 1719
    println!("co2: {:?}", co2.first().unwrap().into_iter().join(""));

    println!("most common: {:?}", mcb.into_iter().join(""));
    // 1869
    println!("least common: {:?}", lcb.into_iter().join(""));
    // 2226
}

fn get_most_common_bit(numbers: &Vec<Vec<char>>, column: usize) -> char {
    let counts = numbers
    .iter()
    .map(|e| e[column])
    .fold(HashMap::new(), |mut acc, c| {
        acc.insert(c, acc.get(&c).unwrap_or(&0) + 1);
        acc
    });
    if counts.get(&'1') > counts.get(&'0') {
        '1'
    } else if counts.get(&'1') < counts.get(&'0') {
        '0'
    } else {
        '2'
    }
}