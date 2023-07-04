//! https://www.acmicpc.net/problem/2417
//! 정수 제곱근 
//! 
//! 음의 아닌 정수 -> 0 일때도 생각해줘야

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let input = read_to_string(stdin().lock())?;
    let n = input.trim().parse::<u128>()?;
    #[allow(unused_assignments)] // 사용했는데;;
    let mut result = 0u64;
    
    if n <= 1 {
        result = n as u64;
    } else {
        let mut left = 1u128;
        let mut right = n;
    
        while left <= right {
            let mid = (left + right) / 2;

            if mid.pow(2) < n {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        result = left as u64;        
    }

    writeln!(output, "{}", result)?;
    Ok(())
}