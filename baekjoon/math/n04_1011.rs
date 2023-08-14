//! https://www.acmicpc.net/problem/1011
//! Fly me to the Alpha Centauri
//! 

use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;

fn travel(start: u32, end: u32, output: &mut String) -> Result<(), Box<dyn Error>> {
    let distance = (end - start) as f32;
    let dist_sqrt = distance.sqrt();

    if dist_sqrt == dist_sqrt.floor() {
        writeln!(output, "{}", (dist_sqrt as u32) * 2 - 1)?;
    } else if dist_sqrt.round() == dist_sqrt.floor() {
        writeln!(output, "{}", dist_sqrt as u32 * 2)?;
    } else {
        writeln!(output, "{}", dist_sqrt as u32 * 2 + 1)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<u32>();

    let t = get_n()?;
    for _ in 0..t {
        travel(get_n()?, get_n()?, &mut output)?;
    }

    println!("{output}");
    Ok(())
}