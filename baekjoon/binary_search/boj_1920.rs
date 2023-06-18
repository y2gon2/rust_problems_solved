// https://www.acmicpc.net/problem/1920
// to be done (unsolved)

use std::io::{stdin, stdout, BufWriter, BufRead, Write};

fn main() {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());

    let mut line = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let n = line()[0] as usize;
    let mut sorted_nums = line();
    sorted_nums.sort();

    let _ = line();
    let nums = line();

    let mut result = String::new();
    for num in nums {
        let mut status = false;
        let mut mn: usize = 0;
        let mut mx: usize = n - 1;

        while mn <= mx {
            let mid = (mn + mx) / 2;
            if num == sorted_nums[mid] {
                status = true;
                break;
            } else if num > sorted_nums[mid] {
                mn = mid + 1;
            } else {
                if mid > 0 {
                    mx = mid - 1;
                } else {
                    break;
                }
            }
        }
        if status { result.push_str("1\n"); }
        else { result.push_str("0\n"); }
    }
    writeln!(output, "{}", result).unwrap();
}