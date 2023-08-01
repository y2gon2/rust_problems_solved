//! https://www.acmicpc.net/problem/7869
//! 두 원

use std::io::{stdin, stdout, Read,Write};
use std::error::Error;
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn Error>> {
    #[allow(unused_assignments)]
    let mut result = 0.0;
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<f32>();

    let (x1, y1, r1) = (get_n()?, get_n()?, get_n()?);
    let (x2, y2, r2) = (get_n()? , get_n()?, get_n()?);

    let distance = ((x2 - x1).powi(2) + (y2- y1).powi(2)).sqrt();

    if distance >= r1 + r2 {
        result = 0.0;
    } else if distance <=  (r1 - r2).abs() {
        result = PI * (r1.min(r2)).powi(2);
    } else {
        let theta1 = (
            (distance.powi(2) + r1.powi(2) - r2.powi(2)) 
            / (2.0 * distance * r1)
        ).acos();

        let theta2 = (
            (distance.powi(2) + r2.powi(2) - r1.powi(2)) 
            / (2.0 * distance * r2)
        ).acos();
        
        let part1 = (r1.powi(2) * theta1) - (r1.powi(2) * (2.0 * theta1).sin() / 2.0);
        let part2 = (r2.powi(2) * theta2) - (r2.powi(2) * (2.0 * theta2).sin() / 2.0);

        result = part1 + part2;
    }

    writeln!(output, "{:.3}", result)?;

    Ok(())
}