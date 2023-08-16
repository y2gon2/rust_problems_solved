//! https://www.acmicpc.net/problem/1052
//! 물병

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n= || input
        .next()
        .unwrap()
        .parse::<u32>();

    let (mut n, k) = (get_n()?, get_n()?);
    let mut cnt = 0u32;

    if k >= n {
        writeln!(output, "0")?;
    } else {
        while n.count_ones() > k {
            cnt += 1;
            n += 1;
        }
        writeln!(output, "{}", cnt)?;
    }

    Ok(())
}