//! https://www.acmicpc.net/problem/1064
//! 평행사변형
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut result = -1.0f64;

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<i32>();

    let mut points = [(0i32, 0i32); 3];
    for i in 0..3 {
        points[i] = (get_n()?, get_n()?);
    } 

    if (points[2].0 - points[0].0) * (points[1].1 - points[0].1) - (points[1].0 - points[0].0) * (points[2].1 - points[0].1) == 0 {

    } else {
        let mut l = vec![0.0f64; 3];
        l[0] = (((points[1].0 - points[0].0).pow(2) + (points[1].1 - points[0].1).pow(2)) as f64).sqrt();
        l[1] = (((points[2].0 - points[1].0).pow(2) + (points[2].1 - points[1].1).pow(2)) as f64).sqrt();
        l[2] = (((points[0].0 - points[2].0).pow(2) + (points[0].1 - points[2].1).pow(2)) as f64).sqrt();
    
        l.sort_by(|a, b| b.partial_cmp(a).unwrap());
        result = (l[0] - l[2]) * 2.0;    
    }
    writeln!(output, "{:.10}", result)?;
    Ok(())
}