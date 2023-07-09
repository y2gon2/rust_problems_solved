//! https://www.acmicpc.net/problem/21758
//! 꿀 따기
//! 

use std::io::{stdin, stdout, Write, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0_usize;
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut buf_iter = buf.split_ascii_whitespace();
    let n = buf_iter.next().unwrap().parse::<usize>()?;

    let mut acc = vec![0_usize; n + 1];
    let mut largest = 0_usize;

    for i in 1..=n {
        let cur = buf_iter.next().unwrap().parse::<usize>()?;
        acc[i] = acc[i - 1] + cur;
        largest = largest.max(cur);
    }
    // println!("{:?}", acc);
    // println!("{} {} {}", (acc[n] - acc[2]) * 2, acc[n - 2] * 2, acc[n - 1] - acc[1] + largest);


    for second in 2..n {
        // 왼쪽끝 첫번째 벌 오른쪽 끝 벌집, 두번째 벌은 그 중간에 위치
        let alone_part_left = acc[second - 1] - acc[1];
        let duplicated_part_left = acc[n] - acc[second];

        result = result.max(alone_part_left + (duplicated_part_left * 2));


        // 오른쪽끝 첫번째 벌 왼쪽쪽 끝 벌집, 두번째 벌은 그 중간에 위치
        let duplicated_part_right = acc[second - 1];
        let alone_part_right = acc[n - 1] - acc[second];

        result = result.max(alone_part_right + (duplicated_part_right * 2));
    }

    // 양끝이 아닌 곳 중 꿀이 많은 곳을 벌집으로 하고 벌들이 양 끝에서 출발하는 경우   
    result = result.max(acc[n - 1] - acc[1] + largest); 
 
    writeln!(output, "{}", result)?;
    Ok(())
}