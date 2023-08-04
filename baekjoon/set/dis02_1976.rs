//! https://www.acmicpc.net/problem/1976
//! 여행 가자

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut result = false;

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<u16>();

    let n = get_n()? as usize;
    let m = get_n()?;
    let mut map = vec![vec![false; n + 1]; n + 1];
    
    for i in 1..=n {
        for j in 1..=n {
            if get_n()? == 1 {
                map[i][j] = true;
            }
        }
    }
    
    for i in 1..=n {
        println!("{:?}", map[i]);
    }

    let mut from = get_n()?;
    let mut to = get_n()?;

    for i in 2..m {
        if map[from as usize][to as usize] {
            from = to;
            to = get_n()?;
            println!("map[{}][{}]: {}", from, to, map[from as usize][to as usize]);
            result = true;
        } else {
            result =false;
            break;
        }
    }

    match result {
        true => writeln!(output, "YES")?,
        false => writeln!(output, "NO")?,
    }
    
    Ok(())
}