use thiserror::Error;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Level {
    #[clap(name = "01a")]
    Level01a,

    #[clap(name = "01b")]
    Level01b,
}

pub fn parse_column(input: &str, col: usize) -> Result<Vec<u32>> {
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

pub fn read_input_string(input_path: &str) -> Result<String> {
    std::fs::read_to_string(input_path).map_err(|e| e.into())
}

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
