// https://www.acmicpc.net/problem/10610
// 30

use std::io::*;
use std::cmp::max;

const RESPECT: i32 = 30;

fn get_permutations(s: &str) -> Vec<String> {
    if s.len() == 1 {
        return vec![s.to_string()];
    }

    let mut result = Vec::new();

    for (i, ch) in s.chars().enumerate() {
        let rest = [&s[..i], &s[i + 1..]].concat();

        for perm in get_permutations(&rest) {
            result.push(ch.to_string() + &perm);
        }
    }

    result
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let input: &str = buffer.trim();

    let permutations = get_permutations(input);

    let mut max_result: i32 = -1;

    for p in permutations.iter() {
        let temp: i32 = p.parse::<i32>().unwrap();
        if temp % RESPECT == 0 {
            max_result = max(max_result, temp);
        }   
    }

    println!("{}", max_result);

    Ok(())
}