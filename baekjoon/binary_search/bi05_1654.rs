// https://www.acmicpc.net/problem/1654
// 랜선 자르기

use std::io::{stdin, stdout, Read, Write, BufWriter, Result};

fn main() -> Result<()> {
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf).unwrap();
    let mut output = BufWriter::new(stdout().lock());

    let mut input: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    
    let n = input[0];
    let k = input[1];
    let cables = input.split_off(2);

    let mut mn: usize = 1;
    let mut mx: usize = 2_147_483_647;

    while mn <= mx {
        let mid = (mn + mx) / 2;
        let mut cnt = 0;
        for i in 0..n {
            cnt += cables[i] / mid;
        }

        // println!("mn:{}\t mid:{}\t mx:{}\t cnt:{}",  mn, mid, mx, cnt);
        if cnt >= k {
            mn = mid + 1;
        } else {
            mx = mid - 1;
        }
    }

    writeln!(output, "{}", mx)?;
    Ok(())
}