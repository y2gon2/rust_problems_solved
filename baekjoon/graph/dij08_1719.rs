//! https://www.acmicpc.net/problem/1719
//! 택배

use std::io::{stdin, Read};
use std::cmp::{Ord, Ordering, PArtialOrd};
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Clone, Debug, Eq, PartialEq)]
struct QueueItem(usize, usize);

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}


fn get_info() -> Result<(usize, Vec<Vec<(usize, usize)>>), ParseIntError> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_num()?, get_num()?);
    let mut graph = vec![vec![]; n + 1];
    let mut from_to = vec![vec![]; n + 1];

    for _ in 0..m {
        let (from, to, weight) = (get_num()?, get_num()?, get_num()?);
        graph[from].push((to, weight));
        graph[to].push((from, weight));
    }

    Ok((n, graph))
}

fn dijkstra (start: usize, start_w: usize, graph: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    priority_queue.push(QueueItem(start, start_w));

    let mut table = vec![usize::MAX; graph.len() + 1];
    table[start] = start_w;

    while let Some(QueueItem(cur, acc)) = priority_queue.pop() {
        for (next, weight) in graph[cur].iter() {
            let sum = acc + weight;

            if sum > table[*next] { continue; }

            priority_queue.push(QueueItem(*next, sum));
            table[*next] = sum;
        }
    }
    
    table
}

fn main() -> Result<(), Box<dyn Error>> {
    let (n, graph) = get_info()?;

    for i in 1..=n {
        let mut next_list = vec![0usize; n + 1];
        for (j, w) in graph[i].iter() {

        }

    }

    println!("{output}");
    Ok(())
}