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
    // let test = vec![
    //   "{([(<{}[<>[]}>{[]{[(<()>",
    //   "[[<[([]))<([[{}[[()]]]",
    //   "[{[{({}]{}}([{[{{{}}([]",
    //   "[<(<(<(<{}))><([]([]()",
    //   "<{([([[(<>()){}]>(<<{{"]
    //   .into_iter()
    //   .map(|line| find_corruption(line.to_string()))
    //   .collect_vec();
    // println!("test: {:?}", test);
    let test = vec![
      "[({(<(())[]>[[{[]{<()<>>",
      "[(()[<>])]({[<{<<[]>>(",
      "(((({<>}<{<{<>}{[]{[]{}",
      "{<[[]]>}<{[{[{[]{()[[[]",
      "<{([{{}}[<[[[<>{}]]]>[]]"]
      .into_iter()
      .map(|line| fix_line(line.to_string()))
      .collect_vec();
    println!("test: {:?}", test);
    println!("scores {:?}", test.into_iter().map(|t| calculate_autocomplete_score(t)).collect_vec());
    if part == 1 {
        let result: u32 = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| find_corruption(&line))
            .filter_map(|maybe| maybe)
            .map(|c| match c {
                ']' => 57,
                '}' => 1197,
                ')' => 3,
                '>' => 25137,
                _ => panic!("Illegal char"),
            })
            .sum();
        println!("{}", result);
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

        println!("{}", result[(result.len() - 1) / 2]);
    }
}

fn fix_line(line: String) -> String {
    let mut stack = VecDeque::new();
    for c in line.chars() {
        // println!("next char: {}", c);
        // println!("current stack: {:?}", stack);
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
    stack
        .into_iter()
        .rev()
        .map(|c| match c {
            '[' => ']',
            '{' => '}',
            '(' => ')',
            '<' => '>',
            _ => panic!("invalid"),
        })
        .join("")
}

fn calculate_autocomplete_score(completion: String) -> u64 {
    completion.chars().fold(0, |mut acc, c| {
        acc *= 5;
        acc += match c {
            ']' => 2,
            '}' => 3,
            ')' => 1,
            '>' => 4,
            _ => panic!("Failed score"),
        };
        acc
    })
}

// attempt to parse, if fail return failing char
fn find_corruption(line: &String) -> Option<char> {
    // println!("========================");
    let mut stack = VecDeque::new();
    for c in line.chars() {
        // println!("next char: {}", c);
        // println!("current stack: {:?}", stack);
        let outcome = match c {
            '[' | '{' | '(' | '<' => {
                stack.push_back(c);
                None
            }
            ']' | '}' | ')' | '>' => {
                let matching = stack.pop_back().unwrap();
                let allowed = match c {
                    ']' => matching == '[',
                    '}' => matching == '{',
                    ')' => matching == '(',
                    '>' => matching == '<',
                    _ => panic!("what"),
                };
                if allowed {
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
