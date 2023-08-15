//! https://www.acmicpc.net/problem/10158
//! 개미
//! 

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
        .parse::<u32>();

    let (w, h, p, q) = (get_n()?, get_n()?, get_n()?, get_n()?);
    let t = get_n()?;

    let mut x = t + p;
    let mut y = t + q;

    if x % (2 * w) >= w {
        x = w - (x % w);
    } else {
        x %= w;
    }

    if y % (2 * h) >= h {
        y = h - (y % h);
    } else {
        y %= h;
    }

    writeln!(output, "{} {}", x, y)?;
    Ok(())
}

