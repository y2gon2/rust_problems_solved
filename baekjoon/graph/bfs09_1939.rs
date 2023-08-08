//! https://www.acmicpc.net/problem/1939
//! 중량제한

use std::io::{stdin, Read};
use std::error::Error;
use std::collections::VecDeque;

fn find(graph: &Vec<Vec<(usize, usize)>>, start: usize, destination: usize, mid: usize) -> bool {
    let mut queue = VecDeque::<usize>::new();
    let mut visited = vec![false; graph.len() + 1];

    queue.push_back(start);

    while let Some(from) = queue.pop_front() {
        if from == destination { return true }
        
        for (next, weight) in graph[from].iter() {
            if visited[*next] || *weight < mid { continue; }
            
            queue.push_back(*next);
            visited[*next] = true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_n()?, get_n()?);

    let mut graph = vec![<Vec<(usize, usize)>>::new(); n + 1];
    let mut max_w = 0usize;
    let mut min_w = 0usize;

    for i in 0..m {
        let n1 = get_n()?;
        let n2 = get_n()?;
        let weight = get_n()?;

        graph[n1].push((n2, weight));
        graph[n2].push((n1, weight));

        max_w = max_w.max(weight);
        min_w = min_w.min(weight);
    }

    let start = get_n()?;
    let destination = get_n()?;

    while min_w <= max_w {
        let mid = (min_w + max_w) / 2;

        if find(&graph, start, destination, mid) {
            min_w = mid + 1;
        } else {
            max_w = mid - 1;
        }
    }

    println!("{}", max_w);
    Ok(())
}