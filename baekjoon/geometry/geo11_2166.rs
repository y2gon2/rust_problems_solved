//! https://www.acmicpc.net/problem/2166
//! 다각형의 면적
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut result = 0.0f64;

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<f64>();

    let n = get_n()? as usize;
    let (x0, y0) = (get_n()?, get_n()?);
    let (mut x1, mut y1) = (get_n()?, get_n()?);


    for _ in 0..n - 2 {
        let (x2, y2) = (get_n()?, get_n()?);
        
        result += (x2 - x0) * (y1 - y0) - (x1 - x0) * (y2 - y0);

        x1 = x2;
        y1 = y2;
    }

    writeln!(output, "{:.1}", result.abs() / 2.0)?;
    Ok(())
}