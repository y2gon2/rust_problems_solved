//! https://www.acmicpc.net/problem/1806
//! 부분합
//! 
//! 풀이과정
//! 누적합으로 vec value 를 만들어 놓고 two pointer 를 각각 전진해 가면서 최소의 합으로
//! 목적값 이상이 나온는 구간을 찾는다.

use std::io::{stdin, stdout, read_to_string, Write, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut result = usize::MAX;
    
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut line = buf.lines();

    let mut info = line.next().unwrap().split_ascii_whitespace();
    let (n, m) = (
        info.next().unwrap().parse::<usize>()?, 
        info.next().unwrap().parse::<usize>()?,
    );

    let mut nums = line
        .next()
        .unwrap()
        .split_ascii_whitespace();

    let mut acc = vec![0usize; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + nums.next().unwrap().parse::<usize>()?;
    }

    // println!("{:?}", acc);

    let mut left = 0usize;
    let mut right = 1usize;
    while left < right &&  right <= n {
        // println!("left: {}  right: {}", left, right);

        if acc[right] - acc[left] >= m {
            result = result.min(right - left);
            left += 1;
        } else {
            if right - left == n {
                result = 0;
                break;
            }
            right += 1;
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}