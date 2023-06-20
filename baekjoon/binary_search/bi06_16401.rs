// https://www.acmicpc.net/problem/16401
// 과자 나눠주기

use std::io::{stdin, stdout, Write, BufRead, BufWriter, Result};


fn main() -> Result<()> {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());

    let mut line = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mn = line();
    let m = mn[0];
    let n = mn[1];

    let snacks = line();
    let mut sum: usize = 0;
    for s in snacks.iter() {
        sum += *s;
    }

    if sum < m {
        writeln!(output, "0")?;
    } else {
        let mut mn: usize = 1;
        let mut mx: usize = 1_000_000_000;
    
        while mn <= mx {
            let mid = (mn + mx) / 2;
            let mut cnt: usize = 0;
            for i in 0..n {
                cnt += snacks[i] / mid;
            }
    
            if cnt >= m {
                mn = mid + 1;
            } else {
                mx = mid - 1;
            }
        }

        writeln!(output, "{}", mx)?;
    }
    Ok(())
}
 