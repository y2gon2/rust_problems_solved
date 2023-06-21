// https://www.acmicpc.net/problem/1253
// 좋다

use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    
    stdin().lock().read_to_string(&mut buf)?;
    let mut nums: Vec<i32> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()?;

    let n = nums.remove(0) as usize;
    nums.sort();

    let mut result: usize = 0;

    for i in 0..n {
        let mut left_idx: usize = 0;
        let mut right_idx: usize = n - 1;
        
        for _ in 0..n - 1 {
            if i == left_idx {
                left_idx += 1;
                continue;
            } else if i == right_idx {
                right_idx -= 1;
                continue;
            }

            let sum_val = nums[right_idx] + nums[left_idx];

            // println!("left_idx:{}\t right_idx:{}\t sum_val:{}", left_idx, right_idx, sum_val);

            if sum_val == nums[i] {
                result += 1;
                break;
            } else if sum_val < nums[i] {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }
    }
    writeln!(output, "{}", result)?;

    Ok(())
}