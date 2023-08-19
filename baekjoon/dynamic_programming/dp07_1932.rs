//! https://www.acmicpc.net/problem/1932
//! 정수 삼각형

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<u32>();

    let n = get_n()? as usize;

    if n == 1 {
        writeln!(output, "{}", get_n()?)?;
        return Ok(())
    }

    let mut tri = vec![Vec::<u32>::new(); n - 1];

    for i in 0..n - 1 {
        let mut v = vec![0u32; i + 1];
        for j in 0..=i {
            v[j] = get_n()?;
        }
        tri[i] = v;
    } 

    let mut prev = get_n()?;
    for i in 0..n - 1 {
        let cur = get_n()?;
        tri[n - 2][i] += prev.max(cur);
        prev = cur;
    }

    for i in (0..n - 2).rev() {
        for j in 0..=i {
            tri[i][j] += tri[i + 1][j].max(tri[i + 1][j + 1]);
        }
    }

    writeln!(output, "{}", tri[0][0])?;
    Ok(())
}