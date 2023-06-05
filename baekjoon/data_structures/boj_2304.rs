// https://www.acmicpc.net/problem/2304

use std::io::*;

fn main () -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut columns: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let col: Vec<usize> = buffer
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        columns.push(col);
    }

    columns.sort_by(|a, b| a.0.cmp(b.0));

    let mut area: usize = 0;

    columns.

    Ok(())
}