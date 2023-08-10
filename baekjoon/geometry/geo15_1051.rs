//! https://www.acmicpc.net/problem/1051
//! 숫자 정사각형

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let (n, m) = (
        input.next().unwrap().parse::<usize>()?,
        input.next().unwrap().parse::<usize>()?,
    );

    let mut sq = vec![vec![0u8; m]; n];
    for i in 0..n {
        sq[i] = input
            .next()
            .unwrap()
            .as_bytes()
            .to_vec();
    }

    let mut length = n.min(m);

    while length > 1 {
        for i in 0..n - length + 1 {
            for j in 0..m - length + 1 {
                let top_left = sq[i][j];
                let top_right = sq[i][j + length - 1];
                let btn_left = sq[i + length - 1][j];
                let btn_right = sq[i + length - 1][j + length - 1];

                // println!("len {}", length);
                // println!("{}, {}, {}, {}", top_left as char, top_right as char, btn_left as char, btn_right as char);

                if top_left == top_right 
                && top_left == btn_left
                && top_left == btn_right {
                    writeln!(stdout(), "{}", length.pow(2))?;
                    return Ok(())
                }
            }
        }
        length -= 1;
    }

    writeln!(stdout(), "1")?;
    Ok(())
}