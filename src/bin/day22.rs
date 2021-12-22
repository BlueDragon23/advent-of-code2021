use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::hash::Hash;

use itertools::Itertools;
use reformation::Reformation;

const PART: u32 = 2;

#[derive(Reformation, Debug, Clone, Copy)]
enum Instruction {
    #[reformation("on {}")]
    On(Cuboid),
    #[reformation("off {}")]
    Off(Cuboid),
}

#[derive(Reformation, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reformation("x={x},y={y},z={z}")]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn contains(&self, other: Cuboid) -> bool {
        self.x.contains(other.x) && self.y.contains(other.y) && self.z.contains(other.z)
    }

    fn overlaps(&self, other: Cuboid) -> bool {
        self.x.overlaps(other.x) && self.y.overlaps(other.y) && self.z.overlaps(other.z)
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

#[derive(Reformation, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[reformation("{min}..{max}")]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, other: Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    fn length(&self) -> i32 {
        self.max - self.min
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.min, self.max)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
    let final_state: HashSet<Cuboid> = instructions
        .into_iter()
        .fold(HashSet::new(), |set, instruction| {
            process_instruction(instruction, set)
        });
    println!("Result: {}", calculate_cubes(final_state));
}

fn filter_instruction(instruction: &Instruction) -> bool {
    let cuboid = match instruction {
        Instruction::Off(c) => c,
        Instruction::On(c) => c,
    };
    PART == 2
        || cuboid.x.min >= -50
            && cuboid.x.max <= 50
            && cuboid.y.min >= -50
            && cuboid.y.max <= 50
            && cuboid.z.min >= -50
            && cuboid.z.max <= 50
}

fn calculate_cubes(active: HashSet<Cuboid>) -> u128 {
    active.into_iter().fold(0, |acc, c| {
        acc + ((c.x.length() + 1) as u128)
            * ((c.y.length() + 1) as u128)
            * ((c.z.length() + 1) as u128)
    })
}

fn process_instruction(instruction: Instruction, active: HashSet<Cuboid>) -> HashSet<Cuboid> {
    let mut new_set = active.clone();
    // Need to find overlapping segments
    match instruction {
        Instruction::Off(c) => handle_off(c, &mut new_set),
        Instruction::On(c) => {
            // remove the overlapping sections
            handle_off(c, &mut new_set);
            new_set.insert(c);
        }
    }
    // println!("==================");
    // print_cubes(&new_set);
    new_set
}

fn print_cubes(cubes: &HashSet<Cuboid>) {
    println!(
        "{}",
        cubes
            .into_iter()
            .sorted_by(|c1, c2| {
                let x_cmp = c1.x.min.cmp(&c2.x.min);
                if x_cmp != Ordering::Equal {
                    return x_cmp;
                }
                c1.y.min.cmp(&c2.y.min)
            })
            .join("\n")
    );
}

fn handle_off(c: Cuboid, active: &mut HashSet<Cuboid>) {
    // Disable any overlapping segments
    let overlapping = find_overlapping(c, &active);
    overlapping.into_iter().for_each(|overlap| {
        // Remove the existing cube
        active.remove(&overlap);
        // Add the subsections that aren't inside the turned off bit
        active.extend(
            subdivide(c, overlap)
                .into_iter()
                .filter(|&child| !c.contains(child)),
        )
    })
}

// Divide two overlapping cubes into 27 subcubes
fn subdivide(c1: Cuboid, c2: Cuboid) -> HashSet<Cuboid> {
    let x_ranges = get_ranges(c1.x, c2.x);
    let y_ranges = get_ranges(c1.y, c2.y);
    let z_ranges = get_ranges(c1.z, c2.z);
    x_ranges
        .into_iter()
        .cartesian_product(y_ranges)
        .cartesian_product(z_ranges)
        .map(|((x, y), z)| Cuboid { x, y, z })
        // .filter(|c| c.x.length() > 0 && c.y.length() > 0 && c.z.length() > 0)
        .filter(|&c| c1.contains(c) || c2.contains(c))
        .collect()
}

fn get_ranges(range1: Range, range2: Range) -> Vec<Range> {
    vec![
        Range {
            min: min(range1.min, range2.min),
            max: max(range1.min, range2.min) - 1,
        },
        Range {
            min: max(range1.min, range2.min),
            max: min(range1.max, range2.max),
        },
        Range {
            min: min(range1.max, range2.max) + 1,
            max: max(range1.max, range2.max),
        },
    ]
}

fn find_overlapping(cuboid: Cuboid, active: &HashSet<Cuboid>) -> HashSet<Cuboid> {
    active
        .into_iter()
        .filter(|&&c| cuboid.overlaps(c))
        .map(|&c| c)
        .collect()
}
