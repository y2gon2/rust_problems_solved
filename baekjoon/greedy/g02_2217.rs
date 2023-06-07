// https://www.acmicpc.net/problem/2217

use std::io::*;
use std::cmp::max;

fn main() -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();
    
    let mut ropes: Vec<usize> = Vec::new();
    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        ropes.push(
            buffer.trim().parse::<usize>().unwrap()
        );
    }

    ropes.sort();

    let mut max_weight: usize = 0;
    let mut num_ropes: usize = 0;

    for rope in ropes.iter().rev() {
        num_ropes += 1;
        max_weight = max(max_weight, rope * num_ropes);
    }

    println!("{}", max_weight);

    Ok(())
}