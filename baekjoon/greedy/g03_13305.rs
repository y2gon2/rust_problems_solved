// https://www.acmicpc.net/problem/13305

use std::io::*;
use std::cmp::min;

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let _n = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let distances: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let prices: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut min_price: usize = 1_000_000_001;
    let mut result: usize = 0;

    for i in 0..prices.len() - 1 {
        min_price = min(min_price, prices[i]);
        result += min_price * distances[i];
    }

    println!("{}", result);

    Ok(())
}