use std::collections::{BinaryHeap, HashMap};

use crate::util::{parse_column, AdvError, Result};

pub fn run_level_01b(input: &str) -> Result<()> {
    let mut a = parse_column(&input, 0)?;
    let b = parse_column(&input, 1)?
        .iter()
        .fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    let mut score: u32 = 0;

    for x in a.iter_mut() {
        if let Some(&y) = b.get(x) {
            score += *x * y;
        }
    }

    println!("{:?}", score);

    Ok(())
}

pub fn run_level_01a(input: &str) -> Result<()> {
    let mut a: BinaryHeap<u32> = BinaryHeap::from(parse_column(&input, 0)?);
    let mut b: BinaryHeap<u32> = BinaryHeap::from(parse_column(&input, 1)?);

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

    Ok(())
}
