//! https://www.acmicpc.net/problem/1027
//! 고층 건물

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::max;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf)?;
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let n = get_n()? as usize;
    let mut skyline: Vec<usize> = vec![0usize; n];

    for i in 0..n {
        skyline[i] = get_n()?;
    }

    let mut is_visible = [[false; 50]; 50];

    for i in 0..n {
        let mut prev_grad = i64::MIN;
        let mut right = i + 1;

        while  right < n {
            let grad = (skyline[right] as i64- skyline[i] as i64) * 1_000_000 / (right - i) as i64;

            if grad > prev_grad {
                is_visible[i][right] = true;
                is_visible[right][i] = true;
                prev_grad = grad
            }
            right += 1;
        }
    }

    let mut result = 0u8;
    for i in 0..n {
        let mut temp = 0u8;
        for j in 0..n {
            if is_visible[i][j] {
                temp += 1;
            }
        }
        result = max(result, temp);
    }

    writeln!(output, "{}", result)?;
    Ok(())
}