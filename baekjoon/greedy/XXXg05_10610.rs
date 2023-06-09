// https://www.acmicpc.net/problem/10610
// 30

use std::io::*;
use std::cmp::max;

const RESPECT: i32 = 30;

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let mut nums: Vec<char> = buffer
        .trim()
        .chars()
        .map(|c| c)
        .collect();
    
    nums.sort_by(|a, b| b.cmp(a));

    let length = nums.len();

    for n in nums.clone() {
        println!("{}", n);
    }

    Ok(())
}