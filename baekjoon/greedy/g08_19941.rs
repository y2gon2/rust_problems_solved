// https://www.acmicpc.net/problem/19941
// 햄버거 분배

use std::io::{stdin, BufRead, Result};

fn main() -> Result<()> {
    let mut buffer = String::new();

    stdin().lock().read_line(&mut buffer).unwrap();
    let v: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let n = v[0];
    let k = v[1];

    buffer.clear();
    stdin().lock().read_line(&mut buffer).unwrap();
    let mut table: Vec<char> = buffer
        .trim()
        .chars()
        .map(|c| c)
        .collect();

    let mut cnt: usize = 0;
    for i in 0..n {
        if table[i] == 'P' {
            for j in 1..=k {
                if i + j < n && table[i + j] == 'H' {
                    cnt += 1;
                    table[i] = 'X';
                    table[i + j] = 'X';
                    break;
                } 
            }
        } else if table[i] == 'H' {
            for j in 1..=k {
                if i + j < n && table[i + j] == 'P' {
                    cnt += 1;
                    table[i] = 'X';
                    table[i + j] = 'X';
                    break;
                } 
            }
        }        
    }

    println!("{}", cnt);

    Ok(())
}

