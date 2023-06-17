// https://www.acmicpc.net/problem/3495
// 아스키 도형

use std::io::{BufRead, BufWriter, Result, stdin, stdout, Write};

fn main() -> Result<()> {
    let mut output = BufWriter::new(stdout().lock());
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut result: usize = 0;
    let mut inside = false;

    let v: Vec<usize> = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut line = || input.next().unwrap().trim().to_string();

    for _ in 0..v[0] {
        let s = line();
        
        for c in s.chars() {
            if c == '/' || c == '\\' {
                if inside {
                    result += 1;
                    inside = false;
                } else {
                    inside = true;
                }
            } 
            if c == '.' {
                if inside {
                    result += 1;
                }
            }
        }
    }
    writeln!(output, "{}", result)?;
    Ok(())
}
