//! https://www.acmicpc.net/problem/11758
//! CCW

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut result = 0i8;

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<i32>();

    let (x1, y1) = (get_num()?, get_num()?);
    let (x2, y2) = (get_num()?, get_num()?);
    let (x3, y3) = (get_num()?, get_num()?);

    let double_s = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);

    if double_s == 0{
        result = 0;
    } else if  double_s > 0  {
        result = 1;
    } else {
        result = -1;
    }

    writeln!(output, "{}", result)?;
    Ok(())
}
