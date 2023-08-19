//! https://www.acmicpc.net/problem/1149
//! RGB거리

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Reuslt<(), Box<dyn Error>> {
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let n = get_n()?;
    let mut r = get_n()?;
    let mut g = get_n()?;
    let mut b = get_n()?;

    let mut prev1: (char, usize);
    let mut prev2: (char, usize);

    if r <= g && r <= b {
        prev1 = ('r', r);
        if g <= b {
            prev2 = ('g', g);
        } else {
            prev2 = ('b', b);
        }
    } else if g <= r && g <= b {
        prev1 = ('g', g);
        if b <= r {
            prev2 = ('b', b);
        } else {
            prev2 = ('r', r);
        }
    } else {
        prev1 = ('b', b);
        if r <= 
    }

    writeln!(output, "{}", )?;
    Ok(())
}