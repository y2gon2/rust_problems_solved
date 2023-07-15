//! https://www.acmicpc.net/problem/10844
//! 쉬운 계단 수
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

const BIG: usize = 1_000_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let n = buf.trim().parse::<usize>()?;

    let mut nums: Vec<Vec<usize>> = vec![vec![0; 10]; n];
    
    nums[0] = vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    let mut i = 1usize;
    while i < n {
        for j in 0..=9 {
            if j != 0 { nums[i][j] += nums[i - 1][j - 1]; }
            if j != 9 { nums[i][j] += nums[i - 1][j + 1]; }
        }

        if nums[i][0] > BIG && nums[i][9] > BIG {
            for j in 0..=9 {
                nums[i][j] %= BIG;
            }
        }
        i += 1;
    }
    
    // println!("{:?}", nums[n - 1]);
    // for i in 0..n {
    //     println!("{:?}", nums[i]);
    // }

    writeln!(output, "{}", nums[n - 1].iter().sum::<usize>() % BIG)?;

    Ok(())
}