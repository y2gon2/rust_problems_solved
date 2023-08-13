//! https://www.acmicpc.net/problem/1015
//! 수열 정렬

use std::io::{stdin, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let n = get_n()?;
    let mut list = vec![Vec::<usize>::new(); 1001];
    let mut mn = usize::MAX;

    for i in 0..n {
        let x = get_n()?;
        list[x].push(i);
        mn = mn.min(x);
    }

    let mut result = vec![0usize; n];
    let mut cnt  = 0usize;
    for i in 0..n {
        loop {
            if list[cnt].is_empty() {
                cnt += 1;
            } else {
                result[list[cnt].remove(0)] = i;
                break;
            }
        }
    }
    
    let output: String = result
        .iter()
        .map(|u| format!("{} ", u))
        .collect();

    println!("{output}");
    Ok(())
}