//! https://www.acmicpc.net/problem/1057
//! 토너먼트

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut info = buf.split_ascii_whitespace();
    let mut get_n = || info
        .next()
        .unwrap()
        .parse::<u32>();

    let mut round = 0u8;
    let _ = get_n()?;
    let mut n1 = get_n()?;
    let mut n2 = get_n()?;

    while n1 != n2 {
        n1 -= n1 / 2;
        n2 -= n2 / 2;
        round += 1;
    }

    writeln!(output, "{}", round)?;
    Ok(())
}