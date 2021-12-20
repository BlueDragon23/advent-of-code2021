use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/input4.txt").unwrap();
    let reader = BufReader::new(f);
    let mut lines = reader.lines();
    let moves = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|mut chunk| {
            // skip a blank line
            chunk.next();
            chunk
                .map(|line| {
                    line.unwrap()
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let part = 1;

    let mut winning_board = 0;
    let mut last_number = 0;
    let (played_numbers, _) = moves
        .into_iter()
        .fold_while(
            (HashSet::new(), Vec::new()),
            |(mut played_numbers, mut won_boards), x| {
                played_numbers.insert(x);
                let winning_boards = get_winning_boards(&boards, &played_numbers);
                if part == 1 && winning_boards.len() == 1 {
                    winning_board = winning_boards[0];
                    last_number = x;
                    return Done((played_numbers, winning_boards));
                } else if part == 2 && winning_boards.len() == boards.len() {
                    winning_board = winning_boards
                        .clone()
                        .into_iter()
                        .find(|x| !won_boards.contains(x))
                        .unwrap();
                    last_number = x;
                    return Done((played_numbers, winning_boards));
                }
                Continue((played_numbers, winning_boards))
            },
        )
        .into_inner();

    println!(
        "{:?} won, played numbers were {:?}",
        winning_board, played_numbers
    );

    println!(
        "Score is {}",
        calculate_score(&boards[winning_board], &played_numbers, last_number)
    );
}

fn get_winning_boards(boards: &Vec<Vec<u32>>, played_numbers: &HashSet<u32>) -> Vec<usize> {
    let winning_boards = boards
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if is_board_complete(&b, played_numbers) {
                return Some(i);
            }
            None
        })
        .collect::<Vec<_>>();
    winning_boards
}

fn is_board_complete(board: &Vec<u32>, numbers: &HashSet<u32>) -> bool {
    // check rows
    for row in 0..5 {
        let mut complete = true;
        for col in 0..5 {
            if !(numbers.contains(&board[row * 5 + col])) {
                complete = false;
                break;
            }
        }
        if complete {
            return true;
        }
    }

    // check cols
    for col in 0..5 {
        let mut complete = true;
        for row in 0..5 {
            if !(numbers.contains(&board[row * 5 + col])) {
                complete = false;
                break;
            }
        }
        if complete {
            return true;
        }
    }
    false
}

fn calculate_score(board: &Vec<u32>, played_numbers: &HashSet<u32>, last_number: u32) -> u32 {
    board
        .into_iter()
        .filter(|x| !played_numbers.contains(x))
        .sum::<u32>()
        * last_number
}
