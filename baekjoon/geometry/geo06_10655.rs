// https://www.acmicpc.net/problem/10655
// 마라톤 1

use std::io::{Result, Write, stdin, BufRead, stdout, BufWriter};

fn main() -> Result<()> {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut line = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut shortcut_adv: usize = 0;
    let mut prev1 = line();
    let mut prev2 = line();
    let mut sum: usize = ((prev1[0] - prev2[0]).abs() + (prev1[1] - prev2[1]).abs()) as usize;

    for _ in 0..n - 2 {
        let cur = line();
        let jump_dist: usize = ((prev1[0] - cur[0]).abs() + (prev1[1] - cur[1]).abs()) as usize;
        let dist1: usize = ((prev1[0] - prev2[0]).abs() + (prev1[1] - prev2[1]).abs()) as usize;
        let dist2: usize = ((prev2[0] - cur[0]).abs() + (prev2[1] - cur[1]).abs()) as usize;
        
        let long_dist = dist1 + dist2;
        if jump_dist < long_dist && (long_dist - jump_dist) > shortcut_adv {
            shortcut_adv = long_dist - jump_dist;
        }

        prev1 = prev2;
        prev2 = cur;

        sum += dist2;
    }

    writeln!(output, "{}", sum - shortcut_adv)?;

    Ok(())
}
