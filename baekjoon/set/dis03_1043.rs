//! https://www.acmicpc.net/problem/1043
//! 거짓말
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn union(parents: &mut Vec<usize>, truth: &Vec<usize>, guest1: usize, guest2: usize) {
    let r1 = find_root(parents, guest1);
    let r2 = find_root(parents, guest2);


    if truth.contains(&r2) {
        parents[r1] = r2;
    } else { 
        parents[r2] = r1;
    }
}

fn find_root(parents: &mut Vec<usize>, guest: usize) -> usize {
    if parents[guest] != guest {
        parents[guest] = find_root(parents, parents[guest]);
    }
    return parents[guest]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0usize;
    let mut output = stdout();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_n()?, get_n()?);
    let known = get_n()?;

    if known == 0 {
        writeln!(output, "{}", m)?;
        return Ok(())
    } else {
        
    }
    
    let mut truth = vec![0usize; known];
    for i in 0..known {
        truth[i] = get_n()?;
    }

    let mut parents: Vec<usize> = (0..=n).collect();
    let mut parties: Vec<usize> = vec![0usize; m];

    for i in 0..m {
        let length = get_n()?;
        parties[i] = get_n()?;

        for k in 0..length - 1 {
            union(&mut parents, &truth, parties[i], get_n()?);
        }
    }

    for party in parties {
        if !truth.contains(&find_root(&mut parents, party)) {
            result += 1;
        } 
    }

    writeln!(output, "{}", result)?;
    Ok(())
}