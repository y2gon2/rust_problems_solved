//! https://www.acmicpc.net/problem/2156
//! 포도주 시식
//! 
//! 3번째 이후부터 해당 칸까지 누적시킬 수 있는 경우의 수는 아래와 같다. 
//! 1. i-3칸까지의 누적값 +  i-1번째 칸의 양 + i번째 칸의 양
//! 2. i-2칸까지의 누적값 +  i번째 칸의 양
//! 3. i-1칸까지의 누적값까지 (i번째 칸은 제외)
//! 위 세가지 경우 중 가장 큰 값을 i번째 칸의 누적값으로 최종 입력

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
        .parse::<u32>();

    let n = get_n()? as usize;
    let mut acc = vec![0u32; n];
    let mut glasses = vec![0u32; n];

    if n == 1 {
        writeln!(output, "{}", get_n()?)?;
        return Ok(())
    } else if n == 2 {
        writeln!(output, "{}", get_n()?+ get_n()?)?;
        return Ok(())
    }

    for i in 0..3 {
        glasses[i] = get_n()?;
    }

    acc[0] = glasses[0];
    acc[1] = glasses[0] + glasses[1];
    acc[2] = max(max(glasses[1], glasses[0]) + glasses[2], glasses[0] + glasses[1]);
 
    for i in 3..n {
        glasses[i] = get_n()?;
        let temp = max(acc[i - 3] + glasses[i - 1], acc[i - 2]) + glasses[i];

        acc[i] = max(temp, acc[i - 1]);
    }
    
    writeln!(output, "{}", acc[n - 1])?;
    Ok(())
}