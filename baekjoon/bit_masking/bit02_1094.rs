//! https://www.acmicpc.net/problem/1094
//! 막대기

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let n = buf.trim().parse::<u8>()?;

    writeln!(output, "{}", n.count_ones())?;

    Ok(())
}