//! https://www.acmicpc.net/problem/11057
//! 오르막 수

use std::io::{stdin, stdout, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    let n = buf.trim().parse::<usize>()?;
    let mut acc = vec![[1u16; 10]; n + 1];
    
    for i in 1..=n {
        for j in 0..10 {
            let mut temp = 0u16;
            for k in 0..=j {
                temp += acc[i - 1][k];
            }
            acc[i][j] = temp % 10_007;
        }
    }

    writeln!(output, "{}", acc[n][9])?;
    Ok(())
}