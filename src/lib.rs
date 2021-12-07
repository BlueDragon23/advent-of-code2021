use reformation::Reformation;

#[derive(Debug)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

// Example union input
#[derive(Reformation, Eq, PartialEq, Debug)]
enum Ant {
    #[reformation(r"Queen\({}\)")]
    Queen(String),
    #[reformation(r"Worker\({}\)")]
    Worker(i32),
    #[reformation(r"Warrior")]
    Warrior,
}

// Example struct input
#[derive(Reformation, Debug)]
#[reformation(r"{year}-{month}-{day} {hour}:{minute}")]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

// Create a method for parsing a line of ints

// Create a method for parsing lines of a file to ints

// Create a method for parsing lines of a file to a particular struct using reformation
