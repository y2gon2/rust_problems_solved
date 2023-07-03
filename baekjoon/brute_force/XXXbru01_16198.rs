// https://www.acmicpc.net/problem/16198
// 에너지 모으기

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;



fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(stdin().lock)?;
    let split = input.split_ascii_whitespace();
    let nums: Vec<usize> = split
        .skip(2)
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    let n = nums.len();
    let mut removed: Vec<bool> = vec![false; n];
    let mut energy = 0usize;
    let sum = 0usize;

    for i in 1..n {
        removed[i] = true;
        sum += get_energy(nums, cur, removed, energy);
    }   
    let sum = get_energy(nums, cur, removed, energy);

    let mut output = stdout().lock();
    writeln!(output, "{}", sum);

    Ok(())
}