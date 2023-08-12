//! https://www.acmicpc.net/problem/1025
//! 제곱수 찾기
//!

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let (n, m) = (
        input.next().unwrap().parse::<usize>()?,
        input.next().unwrap().parse::<usize>()?,
    );

    let mut matrix = vec![
        vec![0u8; m]; n];
    for i in 0..n {
        let v = input
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        matrix[i] = v;
    }

    let num = n.max(m);
    let result = -1i32;

    let mut perfect_sq: Vec<i32> = vec![0];
    let mut cnt = 1i32;
    
    while cnt / 10 * num as i32 == 0 {
        perfect_sq.push(cnt.pow(2));
        cnt += 1;
    }

    let ver_step: Vec<usize> = (0..n - 1).collect();
    let hor_step: Vec<usize> = (0..m - 1).collect();

    for i in 0..n {
        for j in 0..m {
            let mut number = Vec::<u8>::new();
            for y_step in 0..n - 1 {
                if i + y_step >= n { break; }
                for x_step in 0..m - 1 {
                    if j + x_step >= m { break; }

                    
                }
            }
        }
    }


    writeln!(output, "{}", result)?;
    Ok(())
}



