use std::collections::BinaryHeap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdvError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Invalid input")]
    InvalidInput,

    #[error("Unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, AdvError>;

pub enum Level {
    Level01a,
    Level01b,
}

fn main() {
    match run(Level::Level01a, "res/01a.txt") {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_input_string(input_path: &str) -> Result<String> {
    std::fs::read_to_string(input_path).map_err(|e| e.into())
}

fn parse_column(input: &str, col: usize) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(col)
                .ok_or(AdvError::InvalidInput)
                .and_then(|s| s.parse::<u32>().map_err(|_| AdvError::InvalidInput))
        })
        .collect()
}

fn run(level: Level, input_path: &str) -> Result<()> {
    let str = read_input_string(input_path)?;
    let mut a: BinaryHeap<u32> = BinaryHeap::from(parse_column(&str, 0)?);
    let mut b: BinaryHeap<u32> = BinaryHeap::from(parse_column(&str, 1)?);

    let mut sum: u32 = 0;

    while !a.is_empty() && !b.is_empty() {
        sum += match (a.pop(), b.pop()) {
            (Some(a), Some(b)) => {
                if a > b {
                    a - b
                } else {
                    b - a
                }
            }
            _ => return Err(AdvError::Unknown),
        }
    }

    println!("{:?}", sum);
    // convert to heap

    Ok(())
}
