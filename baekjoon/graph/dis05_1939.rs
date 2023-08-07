//! https://www.acmicpc.net/problem/1939
//! 중량제한

use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;

fn union(parent: &mut Vec<(usize, usize)>, n1: usize, n2: usize, weight: usize) {
    let (r1, w1) = find(parent, n1, weight);
    let (r2, w2) = find(parent, n2, weight);

    let weight = w1.min(w2);
    let root = r1.min(r2);

    parent[n1] = (root, weight);
    parent[n2] = (root, weight); 
}

fn find(parent:&mut Vec<(usize, usize)>, n: usize, weight: usize) -> (usize, usize) {
    if parent[n].0 != n {
        parent[n] = find(parent, parent[n].0, parent[n].1.min(weight));
    }
    return (parent[n].0, parent[n].1.min(weight))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rusult = 0usize;
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_n()?, get_n()?);

    let mut parent = vec![(0usize, 0usize); n + 1];
    for i in 0..=n {
        parent[i].0 = i; 
    }

    for _ in 0..m {
        union(&mut parent, get_n()?, get_n()?, get_n()?);
    }

    let _ = get_n()?;

    println!("{}", find(&mut parent, get_n()?, 0).1);

    Ok(())
}