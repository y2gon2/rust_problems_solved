//! https://www.acmicpc.net/problem/11659
//! 구간 합 구하기 4
//! 

use std::io::{stdin, Read};
use std::fmt::Write;

fn main() {
    let mut output = String::new();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let (n, m) = (get_num(), get_num());
    let mut nums = vec![0usize; n + 1];
    for i in 1..=n {
        nums[i] = get_num() + nums[i - 1];
    }

    for _ in 0..m {
        let (_from, to) = (get_num(), get_num());
        let result = nums[to] - nums[get_num() - 1];
        writeln!(output, "{}", result).unwrap();
    }
    println!("{output}");
}