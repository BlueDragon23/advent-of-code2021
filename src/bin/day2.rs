use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input2.txt").unwrap();
    let reader = BufReader::new(f);
    let (x, depth, _) =
        reader
            .lines()
            .map(|line| line.unwrap())
            .fold((0, 0, 0), |(x, depth, aim), line| {
                let mut parts = line.split(" ");
                let direction = parts.next().unwrap();
                let magnitude: i32 = parts.next().unwrap().parse().unwrap();
                match direction {
                    "forward" => (x + magnitude, depth + magnitude * aim, aim),
                    "down" => (x, depth, aim + magnitude),
                    "up" => (x, depth, aim - magnitude),
                    _ => panic!(""),
                }
            });
    println!("{}", x * depth);
}
