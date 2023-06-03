// https://www.acmicpc.net/problem/2605

use std::io::{self, *};

fn main() -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let move_rules: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    
    let mut result: Vec<usize> = (1..n + 1).collect();
    
    for i in 1..n {
        let switch: usize = move_rules[i];

        for j in switch..0 {
            let temp = result[j];
            result[j] = result[j - 1];
            result[j - 1] = temp;
        }
    }

    for x in 0..n {
        println!()
    }

    
    Ok(())
}