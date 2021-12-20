use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

const PART: u32 = 2;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    row: i32,
    col: i32,
}

fn main() {
    let f = File::open("input/input20.txt").unwrap();
    let reader = BufReader::new(f);
    let mut iter = reader.lines();
    let algorithm = iter.next().unwrap().unwrap().chars().collect_vec();
    iter.next();
    let image = iter
        .enumerate()
        .fold(HashSet::new(), |mut set, (row, line)| {
            line.unwrap().chars().enumerate().for_each(|(col, val)| {
                if val == '#' {
                    set.insert(Coordinate {
                        row: row as i32,
                        col: col as i32,
                    });
                }
            });
            set
        });
    let iterations = if PART == 1 { 2 } else { 50 };
    let (final_image, _, _) = (0..iterations).fold(
        (image, HashSet::new(), false),
        |(i, visited, background_on), _| {
            let (next_image, next_visited) = process(&i, &algorithm, &visited, background_on);
            (next_image, next_visited, !background_on)
        },
    );
    println!("Result: {}", final_image.len());
}

// Return the new image, and the visited coordinates
fn process(
    image: &HashSet<Coordinate>,
    algorithm: &Vec<char>,
    visited: &HashSet<Coordinate>,
    background_on: bool,
) -> (HashSet<Coordinate>, HashSet<Coordinate>) {
    let queue = image
        .into_iter()
        // also include previously visited pixels, since they may be adjacent to background lit pixels
        .chain(visited.into_iter())
        .map(|pixel| get_surrounding(pixel))
        .flatten()
        .unique()
        .collect::<VecDeque<_>>();
    let next_visited = queue.clone().into_iter().collect::<HashSet<_>>();
    (
        queue.into_iter().fold(HashSet::new(), |mut set, pixel| {
            if apply_algorithm(image, &pixel, algorithm, visited, background_on) {
                set.insert(pixel);
            }
            set
        }),
        next_visited,
    )
}

fn get_surrounding(pixel: &Coordinate) -> Vec<Coordinate> {
    (-1..=1)
        .map(|row| {
            (-1..=1)
                .map(|col| Coordinate {
                    row: pixel.row + row,
                    col: pixel.col + col,
                })
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn apply_algorithm(
    image: &HashSet<Coordinate>,
    pixel: &Coordinate,
    algorithm: &Vec<char>,
    visited: &HashSet<Coordinate>,
    background_on: bool,
) -> bool {
    let adj = get_surrounding(pixel);
    let bin_str = &adj
        .into_iter()
        .map(|c| {
            if image.contains(&c) || !visited.contains(&c) && background_on {
                '1'
            } else {
                '0'
            }
        })
        .join("");
    // println!("Coordinate {:?} makes string {}", pixel, bin_str);
    let index = usize::from_str_radix(&bin_str, 2).unwrap();
    algorithm[index] == '#'
}

#[allow(dead_code)]
fn print_image(image: &HashSet<Coordinate>) {
    let min_row = image.into_iter().map(|c| c.row).min().unwrap();
    let max_row = image.into_iter().map(|c| c.row).max().unwrap();
    let min_col = image.into_iter().map(|c| c.col).min().unwrap();
    let max_col = image.into_iter().map(|c| c.col).max().unwrap();

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if image.contains(&Coordinate { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
