use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use reformation::Reformation;

#[derive(Reformation, Debug)]
enum Instruction {
    #[reformation(r"forward \({}\)")]
    Forward(i32),
    #[reformation(r"down \({}\)")]
    Down(i32),
    #[reformation(r"up \({}\)")]
    Up(i32),
}

fn main() {
    let f = File::open("input/input2.txt").unwrap();
    let reader = BufReader::new(f);
    let (x, depth, _) = reader
        .lines()
        .map(|line| Instruction::parse(&line.unwrap()).unwrap())
        .fold((0, 0, 0), |(x, depth, aim), line| match line {
            Instruction::Forward(magnitude) => (x + magnitude, depth + magnitude * aim, aim),
            Instruction::Down(magnitude) => (x, depth, aim + magnitude),
            Instruction::Up(magnitude) => (x, depth, aim - magnitude),
        });
    println!("{}", x * depth);
}
