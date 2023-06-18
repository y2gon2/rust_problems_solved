// https://www.acmicpc.net/problem/2512
// 예산

use std::io::{Result, stdin, stdout, BufRead, BufWriter, Write, Read};

fn main() -> Result<()> {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());

    let mut line = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let n = line()[0];
    let mut req = line();
    let budget = line()[0];
    req.sort();

    for i in 0..n {
        if i > 1 {
            req[i] = req[i] + req[i -1];
        }
    }

    let mut result: usize = 0;

    let mut mn: usize = 0;
    let mut mx: usize = n - 1;
    while mn <= mx {
        let mid = (mn + mx) / 2;
        


    }

    writeln!(output, "{}", limit / n)?;

    Ok(())
}