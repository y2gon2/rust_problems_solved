//! https://www.acmicpc.net/problem/17387
//! 선분 교차 2

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn ccw(x1: i64, x2: i64, x3: i64, y1: i64, y2:i64, y3: i64) -> i8 {
    let result = (x1 * y2 + x2 * y3 + x3 * y1)
        - (x2 * y1 + x3 * y2 + x1 * y3);

    if result == 0 { return 0 } 
    else if  result > 0 { return 1 } 
    else { return -1 }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<i64>();

    let (x1, y1, x2, y2) = (get_n()?, get_n()?, get_n()?, get_n()?);
    let (x3, y3, x4, y4) = (get_n()?, get_n()?, get_n()?, get_n()?);

    let check1 = ccw(x1, x2, x3, y1, y2, y3) * ccw(x1, x2, x4, y1, y2, y4);
    let check2 = ccw(x3, x4, x1, y3, y4, y1) * ccw(x3, x4, x2, y3, y4, y2);

    if check1 < 0 && check2 < 0 {
        writeln!(output, "1")?;
    } else if (check1 == 0 && check2 < 0) || (check1 < 0 && check2 == 0) {
        writeln!(output, "1")?;
    } else if check1 == 0 && check2 == 0 {
        if (x1 >= x3.min(x4) && y1 >= y3.min(y4)) && (x1 <= x3.max(x4) && y1 <= y3.max(y4))
        || (x2 >= x3.min(x4) && y2 >= y3.min(y4)) && (x2 <= x3.max(x4) && y2 <= y3.max(y4))
        || (x3 >= x1.min(x2) && y3 >= y1.min(y2)) && (x3 <= x1.max(x2) && y3 <= y1.max(y2))
        || (x4 >= x1.min(x2) && y4 >= y1.min(y2)) && (x4 <= x1.max(x2) && y4 <= y1.max(y2)) {
            writeln!(output, "1")?;
        } else {
            writeln!(output, "0")?;
        }
    } else {
        writeln!(output, "0")?;
    }

    Ok(())
}
