//! https://www.acmicpc.net/problem/1912
//! 연속합

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<i32>();

    let n = get_n()? as usize;
    let mut list = vec![0usize; n];
    let mut result = get_n()?;
    let mut acc = result;

    list[0] = result;

    for i in 1..n {
        acc += get_n()?;

        if acc > 0 {
            acc _
        } 
    }
    
    Ok(())
}