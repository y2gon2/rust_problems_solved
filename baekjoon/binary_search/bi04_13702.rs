// https://www.acmicpc.net/problem/13702
// 이상한 술집

use std::io::{BufRead, BufWriter, Result, Write, stdin, stdout};

fn main() -> Result<()> {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());
    let nm: Vec<usize> = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let n = nm[0];
    let m = nm[1];

    let mut pot = || input
        .next()
        .unwrap()
        .trim()
        .to_string()
        .parse::<usize>()
        .unwrap();
    
    let mut pots: Vec<usize> = Vec::new();

    for _ in 0..n {
        pots.push(pot());
    } 

    let mut result = 0;

    let mut mn: usize = 1;
    let mut mx: usize = pots[n - 1];

    while mn <= mx {
        let mid = (mn + mx) / 2;
        let mut cnt = 0;
        for i in 0..n {
            cnt += pots[i] / mid;
        } 

        if cnt >= m {
            result = mid;
            mn = mid + 1;
        } else {
            mx = mid - 1;
        }
    }

    writeln!(output, "{}", result)?;

    Ok(())
}