//! https://www.acmicpc.net/problem/10972
//! 다음 순열
//! 참고: https://passwd.tistory.com/entry/BOJ-10972-%EB%8B%A4%EC%9D%8C-%EC%88%9C%EC%97%B4next-permutation

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf)?;
    
    let mut input = buf.split_ascii_whitespace(); 
    
    let n = input.next().unwrap().parse::<usize>()?;
    let mut nums = input
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| e)?;

    let mut i = n - 1;
    while i > 0 && nums[i - 1] > nums[i] {
        i -= 1;

        if i <= 0 {
            writeln!(output, "-1")?;
            return Ok(())
        }
    }

    let mut j = n - 1;
    while nums[j] < nums[i - 1] {
        j -= 1;
    }

    let temp = nums[i - 1];
    nums[i - 1] = nums[j];
    nums[j] = temp;

    let mut sub_nums: Vec<usize> = nums.splice(i.., []).collect();
    sub_nums.sort_by(|a, b| b.cmp(a));
    nums.append(&mut sub_nums);

    let result = nums
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("i: {}, j: {} {:?}", i, j, sub_nums);
    writeln!(output, "{}", result)?;
    Ok(())
}
