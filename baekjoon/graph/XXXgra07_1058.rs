//! https://www.acmicpc.net/problem/1058
//! 친구

use std::io::{stdin, stdout, Read, Write};
use std::collections::VecDeque;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();

    let n = input
        .next()
        .unwrap()
        .parse::<usize>()?;

    // let mut visited = vec![vec![false; n]; n];
    // let mut result = 0usize;
    let mut graph = vec![Vec::<usize>::new(); n];
    for i in 0..n {
        let mut temp = Vec::<usize>::new();
        for (idx, c) in input.next().unwrap().char_indices() {
            if c == 'Y' {
                temp.push(idx);
            }     
        }
        graph[i] = temp;
    }

    for i in 0..n {
        let mut queue = VecDeque::new();
        queue.push_back(i);


        while let Some(prev) = queue.pop_front() {

        }
    }

    Ok(())
}