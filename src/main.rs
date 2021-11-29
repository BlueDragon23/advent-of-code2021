fn main() {
    let x = Coordinate { row: 1, col: 1 };
    println!("Hello, world! {:?}", x);
}

#[derive(Debug)]
struct Coordinate {
    row: usize,
    col: usize,
}

