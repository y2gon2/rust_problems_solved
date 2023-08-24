//! https://www.acmicpc.net/problem/12865
//! 평범한 배낭

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::max;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, k) = (get_n()?, get_n()?);
    let mut weights = vec![0usize; n + 1];
    let mut values = vec![0usize; n + 1 ];

    for i in 1..=n {
        weights[i] = get_n()?;
        values[i] =  get_n()?;
    }

    let mut backpack = vec![vec![0usize; k + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=k {
            if j as i32 - weights[i] as i32 >= 0 {
                backpack[i][j] = max(
                    backpack[i - 1][j],
                    backpack[i - 1][j - weights[i]] + values[i]
                );
            } else {
                backpack[i][j] = backpack[i - 1][j];
            }
        }
    }

    writeln!(output, "{}", backpack[n][k])?;
    Ok(())
}