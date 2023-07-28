//! https://www.acmicpc.net/problem/11758
//! CCW

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

#[derive(Eq, PartialEq)]
enum Quadrant4 {
    One,
    Two,
    Three,
    Four,
}

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

    let (hor1, hor2) = (x2 - x1, x3 - x2);
    let (ver1, ver2) = (y2 - y1, y3 - y2);

    if hor1 == 0 && hor2 == 0 {
        result = 0;
    } else if (hor1 == 0 && hor2 > 0) || (hor1 > 0 && hor2 == 0) {
        result = 1;
    } else if  (hor1 == 0 && hor2 < 0) || (hor1 < 0 && hor2 == 0) {
        result = -1;
    } else {
        let (gra1, gra2) = (ver1 as f32 / hor1 as f32, ver2 as f32 / hor2 as f32);

        
    }

    writeln!(output, "{}", result)?;
    Ok(())
}
