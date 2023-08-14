//! https://www.acmicpc.net/problem/1024
//! 수열의 합

use std::io::{stdin, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output= String::from("-1");
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let info: Vec<i64> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    let n = info[0];
    let length = info[1];

    for l in length..=100 {
        if (n - (l * (l - 1) / 2)) % l == 0 {
            let a = (n - (l * (l - 1) / 2)) / l;
            // println!("l: {l}  a: {a}");
            if a >= 0 {
                output = (a..a + l)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");    
            }
            break;
        }
    }

    println!("{output}");
    Ok(())
}