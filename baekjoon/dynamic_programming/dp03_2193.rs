//! https://www.acmicpc.net/problem/2193
//! 이친수

use std::io::{stdin, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let n = buf.trim().parse::<usize>()?;
    let mut memoization = vec![0usize; n + 2];
    memoization[0] = 1;
    memoization[1] = 1;

    match n {
        0 => panic!("invalid input"),
        1 | 2 => (),
        _ => {
            let mut p = 2usize;

            while p < n  {
                memoization[p] = memoization[p - 1] + memoization[p - 2];
                p += 1;
            }
        }
    }
    println!("{}", memoization[n - 1]);    
    Ok(())
}