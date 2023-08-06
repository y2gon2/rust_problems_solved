//! https://www.acmicpc.net/problem/20040
//! 사이클 게임

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn union(points: &mut Vec<usize>, p1: usize, p2: usize) {
    let root_p1 = find(points, p1);
    let root_p2 = find(points, p2);

    if root_p1 < root_p2 {
        points[p2] = root_p2;
    } else {
        points[p1] = root_p2;
    }
}

fn find(points: &mut Vec<usize>, p: usize) -> usize {
    if points[p] != p {
        points[p] = find(points, points[p]);
    }
    return points[p]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = String::new();
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_n()?, get_n()?);
    let mut points: Vec<usize> = (0..=n).collect();

    Ok(())
} 