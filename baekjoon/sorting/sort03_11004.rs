//! https://www.acmicpc.net/problem/11004
//! K번째 수

use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::num::ParseIntError;

fn get_info() -> Result<Vec<i32>, ParseIntError> {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    return buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| e);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let info = get_info()?;
    let mut arr = get_info()?;

    arr.sort();

    writeln!(output, "{}", arr[info[1] as usize - 1])?;
    Ok(())
}