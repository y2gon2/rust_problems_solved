//! https://www.acmicpc.net/problem/1149
//! RGB거리

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::min;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let n = get_n()?;
    let mut prev = (get_n()?, get_n()?, get_n()?);
    let mut cur = (0usize, 0usize, 0usize);

    for _ in 1..n {
        cur = (
            get_n()? + min(prev.1, prev.2),
            get_n()? + min(prev.2, prev.0),
            get_n()? + min(prev.0, prev.1),
        );
        prev = cur;
    }

    writeln!(output, "{}", min(min(cur.0, cur.1), cur.2))?;
    Ok(())
}