//! https://www.acmicpc.net/problem/1069
//! 집으로

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

const TEN_B: usize = 10_000_000_000;
const TEN_B_F: f64 = 10_000_000_000.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input.next().unwrap().parse::<usize>();

    let (x, y, d, t) = (get_n()?, get_n()?, get_n()?, get_n()?);
    let distance_big = (((x.pow(2) + y.pow(2)) as f64).sqrt() * TEN_B_F) as usize;
    
    let d_big = d * TEN_B;

    let num_of_jump = distance_big / d_big;
    let rest_big = distance_big % d_big;

    let mut result = distance_big.min(num_of_jump * t * TEN_B + rest_big);
    result = result.min((num_of_jump + 1) * t * TEN_B + (d_big - rest_big));
    
    if num_of_jump > 0  {
        result = result.min((num_of_jump + 1 ) * t * TEN_B);
    } else {
        result = result.min(2 * t * TEN_B);
    }

    writeln!(output, "{:.9}", result as f64 / TEN_B_F)?;
    Ok(())
}
