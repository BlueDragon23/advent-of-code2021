use reformation::Reformation;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
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

pub fn get_adjacent_points(
    coordinate: Coordinate,
    row_count: usize,
    col_count: usize,
) -> Vec<Coordinate> {
    let mut adj = vec![];
    if coordinate.row != 0 {
        adj.push(Coordinate {
            row: coordinate.row - 1,
            col: coordinate.col,
        });
    }
    if coordinate.row != row_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row + 1,
            col: coordinate.col,
        });
    }
    if coordinate.col != 0 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col - 1,
        });
    }
    if coordinate.col != col_count - 1 {
        adj.push(Coordinate {
            row: coordinate.row,
            col: coordinate.col + 1,
        });
    }
    adj
}
