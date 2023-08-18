//! https://www.acmicpc.net/problem/11053
//! 가장 긴 증가하는 부분 수열

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut result = 1u16;

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<u16>();

    let n = get_n()? as usize;
    let mut list = vec![0u16; n];
    let mut acc = vec![0u16; n];
    acc[n - 1] = 1;
    
    for l in list.iter_mut() {
        *l = get_n()?;
    }
    
    for i in (0..n - 1).rev() {
        let mut post = i + 1;
        while post < n {
            if list[post] > list[i] {
                acc[i] = acc[i].max(acc[post]);
            } 
            post += 1;
        }
        acc[i] += 1;
        result = result.max(acc[i]);
    }
    writeln!(output, "{}", result)?;
    Ok(())
}