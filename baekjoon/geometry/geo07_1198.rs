// https://www.acmicpc.net/problem/1198
// 삼각형으로 자르기

use std::io::{Result, stdin, Write, BufRead, BufWriter, stdout};

fn main() -> Result<()> {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(stdout().lock());
    let mut mx_area: f64 = 0.0;

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut line = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    
    let mut points: Vec<Vec<usize>> = Vec::new();

    for _ in 0..n {
        points.push(line());
    }

    points.push(points[0]);
    points.push(points[1]);

    for i in 0..n {
        for j in 
    }

    Ok(())
}