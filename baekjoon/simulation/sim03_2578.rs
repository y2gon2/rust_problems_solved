//! https://www.acmicpc.net/problem/2578
//! 빙고
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn get_info() -> Result<([[u8; 5]; 5], [u8; 25]), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<u8>();

    let mut board = [[0u8; 5]; 5];
    let mut call = [0u8; 25];

    for i in 0..5 {
        for j in 0..5 {
            board[i][j] = get_num()?;
        }
    }

    for i in 0..25 {
        call[i] = get_num()?;
    }

    Ok((board, call))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let (board, call) = get_info()?;
    let mut row = [0u8; 5];
    let mut col = [0u8; 5];
    let mut cross_left = 0u8;
    let mut cross_right = 0u8;
    let mut cnt = 0u8;


    for (idx, c) in call.iter().enumerate() {
        for i in 0..5 {
            for j in 0..5 {
                if board[i][j] == *c {
                    row[i] += 1;
                    if row[i] == 5 {
                        cnt += 1;
                    }

                    col[j] += 1;
                    if col[j] == 5 {
                        cnt += 1;
                    }
                    
                    if i == j { 
                        cross_left += 1; 
                        if cross_left == 5 {cnt += 1;}
                    }
                    if i == (4 - j) { 
                        cross_right += 1; 
                        if cross_right == 5 {cnt += 1;}
                    }

                    // println!("------------------------------");
                    // println!("cnt: {} y, x: ({}, {}), c: {}", cnt, i , j, c);
                    // println!("row: {:?}", row);
                    // println!("col: {:?}", col);
                    // println!("corss: {} {}", cross_left, cross_right);
 
                    if cnt >= 3 { 
                        writeln!(output, "{}", idx + 1)?;
                        return Ok(())
                    }
                }
            }
        }
    }
    Ok(())
}
