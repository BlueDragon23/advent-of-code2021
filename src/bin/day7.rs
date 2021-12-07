#![feature(int_abs_diff)]
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input7.txt").unwrap();
    let reader = BufReader::new(f);
    let crab_positions = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let max_position = crab_positions.clone().into_iter().max().unwrap();

    let part = 2;
    let fuel_costs = (0..max_position)
        .map(|pos| {
            if part == 1 {
                get_cost_part_1(&crab_positions, pos)
            } else {
                get_cost_part_2(&crab_positions, pos)
            }
        })
        .collect::<Vec<_>>();

    let minimum_fuel: u32 = fuel_costs.into_iter().min().unwrap();

    println!("{}", minimum_fuel);
}

fn get_cost_part_1(crab_positions: &Vec<u32>, pos: u32) -> u32 {
    crab_positions.into_iter().map(|x| x.abs_diff(pos)).sum()
}

fn get_cost_part_2(crab_positions: &Vec<u32>, pos: u32) -> u32 {
    crab_positions
        .into_iter()
        .map(|x| {
            let distance = x.abs_diff(pos);
            return distance * (distance + 1) / 2;
        })
        .sum()
}
