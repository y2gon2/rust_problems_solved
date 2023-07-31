//! https://www.acmicpc.net/problem/12781
//! PIZZA ALVOLOC

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

    let mut x = [0i32; 4];
    let mut y = [0i32; 4];

    for i in 0..4 {
        (x[i], y[i]) = (get_n()?, get_n()?);
    }

    let tri1 = (x[2] - x[0]) * (y[1] - y[0]) - (x[1] - x[0]) * (y[2] - y[0]);
    let tri2 = (x[3] - x[0]) * (y[1] - y[0]) - (x[1] - x[0]) * (y[3] - y[0]);

    // println!("tri1: {tri1} tri2: {tri2}");

    if tri1 == 0 || tri2 == 0 {
        writeln!(output, "0")?;
    } else if (tri1 > 0 && tri2 > 0) || (tri1 < 0 && tri2 < 0) {
        writeln!(output, "0")?;
    } else if (tri1 > 0 && tri2 < 0) || (tri1 < 0 && tri2 > 0) {
        writeln!(output, "1")?;
    }

    Ok(())
}