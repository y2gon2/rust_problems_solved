//! https://www.acmicpc.net/problem/1025
//! 제곱수 찾기
//!

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::HashSet;

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
    let mut result = -1i32;

    let mut perfect_sq = HashSet::<i32>::new();
    let mut cnt = 1i32;
    
    while cnt / 10 * num as i32 == 0 {
        perfect_sq.insert(cnt.pow(2));
        cnt += 1;
    }

    for i in 0..n {
        for j in 0..m {
            let mut number = Vec::<u8>::new();
            for y_step in 0..n - 1 {
                for x_step in 0..m - 1 {
                    let mut y =i + y_step;
                    let mut x =  j + x_step;
                    while y < n && x < m {
                        println!("(y, x) = ({y}, {x})");
                        number.push(matrix[y][x]);

                        println!("number: {:?}", &number);
                        if !number.is_empty() {
                            let num1_str: String = number.iter().map(|u| u.to_string()).collect();
                            let num1: i32 = num1_str.parse::<i32>()?;
                            if perfect_sq.contains(&num1) {
                                result = result.max(num1);
                            }
                            
                            let num2_str: String = number.iter().rev().map(|u| u.to_string()).collect();
                            let num2: i32 = num2_str.parse::<i32>()?;
                            if perfect_sq.contains(&num2) {
                                result = result.max(num2);
                            }
                            println!("num1: {num1}  num2: {num2}");
                        }
    
                        if y_step == 0 && x_step == 0 { break; } 
                        y += y_step;
                        x += x_step;
                    }
                }
            }
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}



