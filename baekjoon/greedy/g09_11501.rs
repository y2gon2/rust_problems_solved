// https://www.acmicpc.net/problem/11501
// 주식

use std::io::{stdin, BufRead, Result};

fn main() -> Result<()> {
    let mut buf = String::new();
    stdin().lock().read_line(&mut buf).unwrap();
    let n = buf.trim().to_string().parse::<usize>().unwrap();

    let mut days: Vec<usize> = Vec::new();
    let mut stocks: Vec<Vec<usize>> = Vec::new();

    for _ in 0..n {
        buf.clear();
        stdin().lock().read_line(&mut buf).unwrap();
        days.push(buf.trim().to_string().parse::<usize>().unwrap());

        buf.clear();
        stdin().lock().read_line(&mut buf).unwrap();
        let temp_s = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
        stocks.push(temp_s);
    }
 
    for i in 0..n {
        let mut height = 0;
        let mut result = 0;

        for j in (0..days[i]).rev() {
            if height > stocks[i][j] {
                result += height - stocks[i][j];
            } else if height < stocks[i][j] {
                height = stocks[i][j];
            }
        }

        println!("{}", result);
    }



    Ok(())
}