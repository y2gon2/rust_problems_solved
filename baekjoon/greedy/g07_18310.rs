// https://www.acmicpc.net/problem/18310
// 안테나

use std::io::{stdin, BufRead, Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().lock().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    stdin().lock().read_line(&mut buffer).unwrap();
    let mut houses: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    houses.sort();

    println!("{}", houses[(n - 1) / 2]);

    Ok(())
}