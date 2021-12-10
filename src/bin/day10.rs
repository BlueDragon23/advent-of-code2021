use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/input10.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    if part == 1 {
        let result: u32 = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| find_corruption(&line))
            .filter_map(|maybe| maybe)
            .map(|c| match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Illegal char"),
            })
            .sum();
        println!("Part 1: {}", result);
    } else {
        let result = reader
            .lines()
            .map(|line| line.unwrap())
            .filter_map(|line| {
                let corrupt = find_corruption(&line);
                if corrupt.is_none() {
                    return Some(line);
                } else {
                    return None;
                }
            })
            .map(|line| fix_line(line))
            .map(|completion| calculate_autocomplete_score(completion))
            .sorted()
            .collect_vec();

        println!("Part 2: {}", result[(result.len() - 1) / 2]);
    }
}

// Get the characters missing from the incomplete line
fn fix_line(line: String) -> String {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        match c {
            '[' | '{' | '(' | '<' => {
                stack.push_back(c);
            }
            ']' | '}' | ')' | '>' => {
                stack.pop_back();
            }
            _ => panic!("Invalid char"),
        };
    }
    stack.into_iter().rev().map(|c| get_opposite(c)).join("")
}

fn calculate_autocomplete_score(completion: String) -> u64 {
    completion.chars().fold(0, |mut acc, c| {
        acc *= 5;
        acc += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Failed score"),
        };
        acc
    })
}

// attempt to parse, if fail return failing char
fn find_corruption(line: &String) -> Option<char> {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        let outcome = match c {
            '[' | '{' | '(' | '<' => {
                stack.push_back(c);
                None
            }
            ']' | '}' | ')' | '>' => {
                let matching = stack.pop_back().unwrap();
                let expected = get_opposite(c);
                if matching == expected {
                    None
                } else {
                    Some(c)
                }
            }
            _ => panic!("Invalid char"),
        };
        if outcome.is_some() {
            return outcome;
        }
    }
    None
}

fn get_opposite(c: char) -> char {
    match c {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        '<' => '>',
        ']' => '[',
        '}' => '{',
        ')' => '(',
        '>' => '<',
        _ => panic!("Invalid char"),
    }
}
