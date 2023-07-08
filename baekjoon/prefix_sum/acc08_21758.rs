//! https://www.acmicpc.net/problem/21758
//! 꿀 따기
//! 

use std::io::{stdin, stdout, Write, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0usize;
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut buf_iter = buf.split_ascii_whitespace();
    let n = buf_iter.next().unwrap().parse::<usize>()?;

    let mut acc = vec![0usize; n + 1];
    let mut largest = 0usize;

    for i in 1..=n {
        let cur = buf_iter.next().unwrap().parse::<usize>()?;
        acc[i] = acc[i - 1] + cur;
        largest = largest.max(cur);
    }
    // println!("{:?}", acc);
    // println!("{} {} {}", (acc[n] - acc[2]) * 2, acc[n - 2] * 2, acc[n - 1] - acc[1] + largest);

    if acc[1] <= acc[n] - acc[n - 1] {
        for second in 2..n {
            let alone_part = acc[second - 1] - acc[1];
            let duplicated_part = acc[n] - acc[second];

            result = result.max(alone_part + (duplicated_part * 2));

            println!("1. second:{}  sum:{} al:{} du:{}",  second, alone_part + (duplicated_part * 2), alone_part, duplicated_part);
        }
    } else {
        for second in 2..n {
            let duplicated_part = acc[second - 1];
            let alone_part = acc[n - 1] - acc[second];

            result = result.max(alone_part + (duplicated_part * 2));
            println!("2. second:{}  sum:{} du:{}",  second, alone_part + (duplicated_part * 2), duplicated_part);
        }
    }

    result = result.max(acc[n - 1] - acc[1] + largest); // 가장 꿀이 많은 곳을 벌집으로 하고 벌들이 양 끝에서 출발하는 경우   
 
    writeln!(output, "{}", result)?;
    Ok(())
}