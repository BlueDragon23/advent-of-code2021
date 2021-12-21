use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io::BufRead;

use itertools::Itertools;
use reformation::Reformation;

const PART: u32 = 2;
const P1_START: u32 = 2;
const P2_START: u32 = 1;

#[derive(Clone, Copy)]
struct State {
    p1_position: u32,
    p1_score: u32,
    p2_position: u32,
    p2_score: u32,
    dice_number: u32,
    dice_rolls: u32,
    p1_turn: bool,
}

fn main() {
    let mut state = State {
        p1_position: P1_START,
        p2_position: P2_START,
        p1_score: 0,
        p2_score: 0,
        dice_number: 1,
        dice_rolls: 0,
        p1_turn: true,
    };
    let mut states = vec![state];
    let mut finished_states = Vec::new();
    let end_score = if PART == 1 { 1000 } else { 21 };
    let rolls = generate_rolls();
    println!("{:?}", rolls);
    while state.p1_score < end_score && state.p2_score < end_score {
        if PART == 1 {
            state = step_1(state);
        } else {
            states = states
                .into_iter()
                .map(|s| step_2(s, &rolls))
                .flatten()
                .collect_vec();
            finished_states.extend(states.clone().into_iter().filter(|s| s.p1_score >= 21 || s.p2_score >= 21));
            states = states.into_iter().filter(|s| s.p1_score < 21 && s.p2_score < 21).collect_vec();
            println!("Ongoing games {}, finished games {}", states.len(), finished_states.len());
        }
    }
    println!("{}, {}, {}", state.p1_score, state.p2_score, state.dice_rolls);
    if PART == 1 {
        println!(
            "Part 1: {}",
            min(state.p1_score, state.p2_score) * state.dice_rolls
        );
    }
    let (p1_wins, p2_wins): (Vec<_>, Vec<_>) =
        finished_states.into_iter().partition(|s| s.p1_score > s.p2_score);
    println!("Part 2: {}", max(p1_wins.len(), p2_wins.len()));
}

fn step_1(mut state: State) -> State {
    let rolls = state.dice_number * 3 + 3;
    if state.p1_turn {
        state.p1_position = state.p1_position + rolls;
        while state.p1_position > 10 {
            state.p1_position -= 10;
        }
        state.p1_score += state.p1_position;
    } else {
        state.p2_position = state.p2_position + rolls;
        while state.p2_position > 10 {
            state.p2_position -= 10;
        }
        state.p2_score += state.p2_position;
    }
    state.dice_rolls += 3;
    state.dice_number += 3;
    if state.dice_number > 100 {
        state.dice_number -= 100;
    }
    state.p1_turn = !state.p1_turn;
    state
}

fn step_2(state: State, rolls: &Vec<u32>) -> Vec<State> {
    match state.p1_turn {
        true => {
            rolls.into_iter().map(|r| {
                let mut p1_position = state.p1_position + r;
                while p1_position > 10 {
                    p1_position -= 10;
                }
                State {
                    p1_position: p1_position,
                    p1_score: state.p1_score + p1_position,
                    p2_position: state.p2_position,
                    p2_score: state.p2_score,
                    dice_number: 0,
                    dice_rolls: state.dice_rolls + 3,
                    p1_turn: false
                }
            }).collect_vec()
        },
        false => {
            rolls.into_iter().map(|r| {
                let mut p2_position = state.p2_position + r;
                while p2_position > 10 {
                    p2_position -= 10;
                }
                State {
                    p1_position: state.p1_position,
                    p1_score: state.p1_score,
                    p2_position: p2_position,
                    p2_score: state.p2_score + p2_position,
                    dice_number: 0,
                    dice_rolls: state.dice_rolls + 3,
                    p1_turn: true
                }
            }).collect_vec()
        }
    }
}

fn generate_rolls() -> Vec<u32> {
    vec![1, 2, 3]
        .into_iter()
        .cartesian_product(vec![1, 2, 3])
        .cartesian_product(vec![1, 2, 3])
        .map(|((a, b), c)| a + b + c)
        .collect_vec()
}
