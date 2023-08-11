//! https://www.acmicpc.net/problem/1010
//! 다리 놓기

use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;

fn find_cases(n1: u128, n2: u128, output: &mut String) -> Result<(), Box<dyn Error>>{
    let remain = n2.max(n1 - n2);
    let mut numerator = 1u128;
    let mut denominator = 1u128;

    for i in remain + 1..=n1 {
        numerator *= i;
    }

    for i in 2..=n2.min(n1 - n2) {
        denominator *= i;
    }

    writeln!(output, "{}", numerator / denominator)?;
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
        .parse::<u128>();

    let t = get_n()?;
    for _ in 0..t {
        let a = get_n()?;
        let b = get_n()?;

        find_cases(a.max(b), a.min(b), &mut output)?;
    }

    println!("{output}");
    Ok(())
}