// https://www.acmicpc.net/problem/2467
// 용액

use std::io::{stdin, stdout, Read, Write, BufWriter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf)?;
    let mut nums: Vec<i32>  = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()?;

    let n = nums.remove(0) as usize;
    nums.sort();

    let mut left_idx: usize = 0;
    let mut right_idx: usize = n - 1;
    let mut left_val = nums[left_idx];
    let mut right_val = nums[right_idx];
    let mut closest_sum = right_val + left_val;

    for _ in 1..n{
        let cur_sum = nums[right_idx] + nums[left_idx];

        if closest_sum.abs() >= cur_sum.abs() {
            right_val = nums[right_idx];
            left_val = nums[left_idx];
            closest_sum = cur_sum;
        }

        if cur_sum == 0{
            break;
        } else if cur_sum > 0 {
            right_idx -= 1;
        } else {
            left_idx += 1; 
        } 
    }

    writeln!(output, "{} {}", left_val, right_val)?;

    Ok(())
}