// https://www.acmicpc.net/problem/10816
// 숫자 카드 2 

use std::io::{BufWriter, BufRead, Write, stdin, stdout};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut result = String::new();

    let mut input = stdin().lock().lines().map(|line| line);
    let mut line = || -> Result<Vec<i32>, Box<dyn Error>> {
        match input.next() {
            Some(Ok(line)) => {
                let l: Result<Vec<i32>, _> = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect();
                l.map_err(|e| e.into())
            },
            Some(Err(e)) => Err(e.into()),
            None => Err("No more input".into()),
        }
    };

    let v1 = line()?;
    let n = v1[0] as usize;

    let mut map: HashMap<i32, usize> = HashMap::new();
    let cards = line()?;

    for i in 0..n {
        map.entry(cards[i]).and_modify(|v| *v += 1).or_insert(1);
    }

    let v3 = line()?;
    let _m = v3[0] as usize;

    let checkers = line()?;
    for c in checkers.iter() {
        match map.get(&c) {
            Some(v) => result.push_str(&(v.to_string() +  " ")),
            None => result.push_str("0 "),
        }
    }
    writeln!(output, "{}", result)?;
    Ok(())
}
