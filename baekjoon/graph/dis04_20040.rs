//! https://www.acmicpc.net/problem/20040
//! 사이클 게임

use std::io::{stdin, stdout, Write, Read};
use std::error::Error;

fn union(parent: &mut Vec<usize>, a: usize, b: usize) -> bool {
    let parent_a = find(parent, a);
    let parent_b = find(parent, b);

    if parent_a == parent_b { true }
    else {
        parent[parent_a.max(parent_b)] = parent_a.min(parent_b);
        false
    }
}

fn find(parent: &mut Vec<usize>, mut a: usize) -> usize {
    while parent[a] != a { 
        a = parent[a];
    }
    return a
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_n()?, get_n()?);
    let mut parent: Vec<usize> = (0..n).collect();

    for i in 1..=m {
        if union(&mut parent, get_n()?, get_n()?) {
            writeln!(output, "{}", i)?;
            return Ok(())
        }
    }

    writeln!(output, "0")?;
    Ok(())
} 