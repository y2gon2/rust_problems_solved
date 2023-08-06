//! https://www.acmicpc.net/problem/1976
//! 여행 가자

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn union(parents:&mut Vec<usize>, n1: usize, n2: usize) {
    let n1_root = find(parents, n1);
    let n2_root = find(parents, n2);

    if n1_root < n2_root {
        parents[n2_root] = n1_root;
    } else {
        parents[n1_root] = n2_root;
    }
}

fn find(parents:&mut Vec<usize>, x: usize) -> usize {
    if parents[x] != x {
        parents[x] = find(parents, parents[x]);
    }
    return parents[x]
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

    let n = get_n()?;
    let m = get_n()?;
    let mut parents: Vec<usize> = (0..=n).collect();
    
    for i in 1..=n {
        for j in 1..=n {
            if get_n()? == 1 {
                union(&mut parents, i, j);
            }
        }
    }
    
    let start_parent = parents[get_n()?];

    for _ in 1..m {
        if parents[get_n()?] != start_parent {
            writeln!(output, "NO")?;
            return Ok(())
        }
    }

    writeln!(output, "YES")?;
    Ok(())
}