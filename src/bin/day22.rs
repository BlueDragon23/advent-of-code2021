use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::hash::Hash;
use std::io::BufRead;
use std::iter::Inspect;

use itertools::Itertools;
use reformation::Reformation;

const PART: u32 = 1;

#[derive(Reformation, Debug, Clone, Copy)]
enum Instruction {
    #[reformation("on {}")]
    On(Cuboid),
    #[reformation("off {}")]
    Off(Cuboid),
}

#[derive(Reformation, Debug, Clone, Copy)]
#[reformation("x={x_min}..{x_max},y={y_min}..{y_max},z={z_min}..{z_max}")]
struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let instructions = fs::read_to_string("input/input22.txt")
        .unwrap()
        .lines()
        .map(|line| Instruction::parse(line).unwrap())
        .filter(|instruction| filter_instruction(instruction))
        .collect_vec();
    // I bet I need to actually track ranges rather than cubes for part 2
    let final_state: HashSet<Cube> = instructions
        .into_iter()
        .fold(HashSet::new(), |set, instruction| {
            process_instruction(&instruction, set)
        });
    println!("Result: {}", final_state.len());
}

fn filter_instruction(instruction: &Instruction) -> bool {
    let Cuboid = match instruction {
        Instruction::Off(c) => c,
        Instruction::On(c) => c,
    };
    PART == 2
        || Cuboid.x_min >= -50
            && Cuboid.x_max <= 50
            && Cuboid.y_min >= -50
            && Cuboid.y_max <= 50
            && Cuboid.z_min >= -50
            && Cuboid.z_max <= 50
}

fn process_instruction(instruction: &Instruction, mut cubes: HashSet<Cube>) -> HashSet<Cube> {
    match instruction {
        Instruction::On(c) => {
            for x in c.x_min..=c.x_max {
                for y in c.y_min..=c.y_max {
                    for z in c.z_min..=c.z_max {
                        cubes.insert(Cube { x, y, z });
                    }
                }
            }
        }
        Instruction::Off(c) => {
            for x in c.x_min..=c.x_max {
                for y in c.y_min..=c.y_max {
                    for z in c.z_min..=c.z_max {
                        cubes.remove(&Cube { x, y, z });
                    }
                }
            }
        }
    }
    cubes
}
