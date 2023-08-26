//! https://www.acmicpc.net/problem/11055
//! 가장 큰 증가하는 부분 수열

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::max;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut output = stdout();
    // let mut result = 0usize;

    // let mut buf = String::new();
    // let _ = stdin().read_to_string(&mut buf);
    // let mut input = buf.split_ascii_whitespace();
    // let mut get_n = || input
    //     .next()
    //     .unwrap()
    //     .parse::<usize>();
    
    // let n = get_n()?;
    // let mut nums = vec![0usize; n];
    
    // for i in 0..n {
    //     nums[i] = get_n()?;
    // }

    // for i in 0..n - 1 {
    //     let mut from = nums[i];
    //     let mut temp_acc = vec![0usize; n];

    //     for j in (i + 1)..n {
    //         if from < nums[j] {
    //             let 
    //             temp_acc += nums[j];
    //             prev = nums[j];
    //         }
    //     }

    //     println!("i: {}  teme_acc: {}", i, temp_acc);
    //     result = max(result, temp_acc);
    // }
    // writeln!(output, "{}", result)?;
    Ok(())
}