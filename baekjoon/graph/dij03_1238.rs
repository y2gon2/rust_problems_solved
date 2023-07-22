//! https://www.acmicpc.net/problem/1238
//! 파티
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, Eq, PartialEq)]
struct QueueItem(usize, usize); // next, weight

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

fn get_input() -> Result<(usize, usize, Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m, start) = (get_num()?, get_num()?, get_num()?);
    let mut graph_forward = vec![vec![]; n + 1];
    let mut graph_backward = vec![vec![];  n + 1];

    for _ in 0..m {
        let (from, to, weight) = (get_num()?, get_num()?, get_num()?);
        graph_forward[from].push((to, weight));
        graph_backward[to].push((from, weight));
    }

    Ok((n, start, graph_forward, graph_backward))
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, usize)>>, distances: &mut Vec<usize>) {   
    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    priority_queue.push(QueueItem(start, 0));

    while let Some(QueueItem(cur, acc)) = priority_queue.pop() {
        for (next, weight) in graph[cur].iter() {
            let sum = acc + *weight;
            if sum >= distances[*next] { continue; }

            distances[*next] = sum;
            priority_queue.push(QueueItem(*next, sum));
        }
    }}


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let (
        n, 
        start, 
        graph_forward, 
        graph_backward
    ) = get_input()?;

    let mut distances_go = vec![usize::MAX; n + 1];
    let mut distances_come = vec![usize::MAX; n + 1];

    distances_go[start] = 0;
    distances_come[start] = 0;

    dijkstra(start, &graph_forward, &mut distances_go);
    dijkstra(start, &graph_backward, &mut distances_come);
    
    let result = distances_go
        .into_iter()
        .skip(1)
        .zip(distances_come.into_iter().skip(1))
        .map(|(g, c)| g + c)
        .max()
        .unwrap();

    writeln!(output, "{}", result)?;
    Ok(())
}