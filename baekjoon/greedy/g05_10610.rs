// https://www.acmicpc.net/problem/10610
// 30

use std::io::*;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    
    let mut nums: Vec<u64> = buffer
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    nums.sort_by(|a, b| b.cmp(a));

    let mut sum: u64 = 0;
    for n in nums.iter() {
        sum += n;
    }

    let mut result: String = "-1".to_string();

    if nums.len() > 0 && nums[nums.len() - 1] == 0 && sum % 3 == 0 {
        result = nums.iter().map(|c| c.to_string()).collect();
    } 

    println!("{}", result);

}