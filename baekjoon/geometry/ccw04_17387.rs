//! https://www.acmicpc.net/problem/17387
//! 선분 교차 2

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
        .parse::<i64>();

    let (x0, y0, x1, y1) = (get_n()?, get_n()?, get_n()?, get_n()?);
    let (x2, y2, x3, y3) = (get_n()?, get_n()?, get_n()?, get_n()?);

    if (x0 == x2 && y0 == y2) || (x0 == x3 && y0 == y3) || (x1 == x2 && y1 == y2) || (x1 == x3 && y1 == y3) {
        writeln!(output, "1")?;
        return Ok(())
    }  

    let ccw1 = (x2 - x0) * (y1 - y0) - (x1 - x0) * (y2 - y0);
    let ccw2 = (x3 - x0) * (y1 - y0) - (x1 - x0) * (y3 - y0);
    let ccw3 = (x0 - x3) * (y2 - y3) - (x2 - x3) * (y0 - y3);
    let ccw4 = (x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3);

    if ccw1 * ccw2 < 0 && ccw3 * ccw4 < 0 {
        writeln!(output, "1")?;
    } else if (ccw1 * ccw2 == 0 && ccw3 * ccw4 < 0) || (ccw1 * ccw2 < 0 && ccw3 * ccw4 == 0) {
        writeln!(output, "1")?;
    } else {
        writeln!(output, "0")?;
    }

    Ok(())
}