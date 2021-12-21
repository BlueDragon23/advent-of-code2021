use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;

const P1_START: u32 = 2;
const P2_START: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    p1_position: u32,
    p1_score: u32,
    p2_position: u32,
    p2_score: u32,
}

fn main() {
    let state = State {
        p1_position: P1_START,
        p2_position: P2_START,
        p1_score: 0,
        p2_score: 0,
    };
    let mut states = HashMap::new();
    states.insert(state, 1);
    let mut finished_states = HashMap::new();
    let rolls = generate_rolls();
    // println!("{:?}", rolls);
    let mut p1_turn = true;
    while states.len() > 0 {
        states = states
            .into_iter()
            .map(|(s, count)| {
                let player_state = match p1_turn {
                    true => (s.p1_position, s.p1_score),
                    false => (s.p2_position, s.p2_score),
                };
                step_2(player_state, &rolls)
                    .into_iter()
                    .map(|((position, score), step_count)| {
                        let new_state = match p1_turn {
                            true => State {
                                p1_position: position,
                                p1_score: score,
                                p2_position: s.p2_position,
                                p2_score: s.p2_score,
                            },
                            false => State {
                                p1_position: s.p1_position,
                                p1_score: s.p1_score,
                                p2_position: position,
                                p2_score: score,
                            },
                        };
                        (new_state, count * step_count)
                    })
                    .collect::<HashMap<_, _>>()
            })
            .fold(HashMap::new(), |mut m: HashMap<State, u128>, sub_state| {
                sub_state.into_iter().for_each(|(s, c)| {
                    *m.entry(s).or_insert(0) += c;
                });
                m
            });
        p1_turn = !p1_turn;
        finished_states = finished_states
            .into_iter()
            .chain(states.clone().into_iter())
            .filter(|(s, _)| s.p1_score >= 21 || s.p2_score >= 21)
            .fold(HashMap::new(), |mut m, (s, c)| {
                *m.entry(s).or_insert(0) += c;
                m
            });
        states = states
            .into_iter()
            .filter(|(s, _)| s.p1_score < 21 && s.p2_score < 21)
            .collect();
    }
    let (p1_wins, p2_wins): (Vec<_>, Vec<_>) = finished_states
        .into_iter()
        .partition(|(s, _)| s.p1_score > s.p2_score);
    let p1_win_count: u128 = p1_wins.into_iter().map(|(_, count)| count).sum();
    let p2_win_count: u128 = p2_wins.into_iter().map(|(_, count)| count).sum();
    println!("Scores were {} and {}", p1_win_count, p2_win_count);
    println!("Part 2: {}", max(p1_win_count, p2_win_count));
}

fn get_game_count(states: &HashMap<State, u128>) -> u128 {
    states.into_iter().map(|(_, c)| c).sum()
}

#[allow(dead_code)]
fn print_game_count(states: &HashMap<State, u128>, finished_states: &HashMap<State, u128>) {
    let ongoing_count = get_game_count(states);
    let finished_count = get_game_count(finished_states);
    let total_count = ongoing_count + finished_count;
    println!(
        "Ongoing games {}, finished games {}, total {}",
        ongoing_count, finished_count, total_count
    );
}

fn step_2(player: (u32, u32), rolls: &HashMap<u32, u128>) -> HashMap<(u32, u32), u128> {
    rolls
        .into_iter()
        .map(|(increment, &count)| {
            let mut position = player.0 + increment;
            while position > 10 {
                position -= 10;
            }
            let score = player.1 + position;
            ((position, score), count)
        })
        .collect::<HashMap<_, _>>()
}

// Find out how many universes there are for each value
fn generate_rolls() -> HashMap<u32, u128> {
    vec![1, 2, 3]
        .into_iter()
        .cartesian_product(vec![1, 2, 3])
        .cartesian_product(vec![1, 2, 3])
        .map(|((a, b), c)| a + b + c)
        .fold(HashMap::new(), |mut m, sum| {
            *m.entry(sum).or_insert(0) += 1;
            m
        })
}
