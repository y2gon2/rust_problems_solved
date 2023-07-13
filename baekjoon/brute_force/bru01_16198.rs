//! https://www.acmicpc.net/problem/16198
//! 에너지 모으기
//! 
//! back tracking - 재귀함수 구현에 익숙해져야.. 

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;
use std::cmp::max;

fn back_tracking(nums: &mut Vec<u32>, n: usize, sum: u32, result: &mut u32){
    if nums.len() == 2 {
        *result = max(*result, sum);
        return
    }
    
    for i in 1..n - 1 {
        let sum = sum + nums[i - 1] * nums[i + 1];
        let num = nums.remove(i);
       
        back_tracking(nums, n - 1, sum, result);
        nums.insert(i, num);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(stdin().lock())?;
    let split = input.split_ascii_whitespace();
    let mut nums: Vec<u32> = split
        .skip(1)
        .map(|s| s.parse::<u32>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    let n = nums.len();
    let sum = 0u32;
    let mut result = 0u32;
 
    back_tracking(&mut nums, n, sum, &mut result);

    let mut output = stdout().lock();
    writeln!(output, "{}", result)?;

    Ok(())
}