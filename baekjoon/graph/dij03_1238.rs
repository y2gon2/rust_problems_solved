//! https://www.acmicpc.net/problem/1238
//! 파티
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd, max};

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

fn get_input() -> Result<(usize, usize, Vec<Vec<(usize, usize)>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m, start) = (get_num()?, get_num()?, get_num()?);
    let mut graph = vec![vec![]; n + 1];
    for _ in 0..m {
        graph[get_num()?].push((get_num()?, get_num()?));
    }

    Ok((n, start, graph))
}

fn go_dijkstra(n:usize, start: usize, graph: &Vec<Vec<(usize, usize)>>, distances: &mut Vec<usize>) {   
    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    priority_queue.push(QueueItem(start, 0));
    distances[start] = 0;
    
    while let Some(QueueItem(cur, acc)) = priority_queue.pop() {
        for (next, weight) in graph[cur].iter() {
            let sum = acc + *weight;
            if sum >= distances[*next] { continue; }

            distances[*next] = sum;
            priority_queue.push(QueueItem(*next, sum));
        }
    }
}

fn come_dijkstra(n:usize, from: usize, to: usize, graph: &Vec<Vec<(usize, usize)>>, distances: &mut Vec<usize>)
{

}


fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0usize;
    let mut output = stdout().lock();

    let (n, start, graph) = get_input()?;
    let mut distances_go = vec![usize::MAX; n + 1];
    let mut distances_come = vec![usize::MAX; n + 1];
    distances_come[start] = 0;
    
    go_dijkstra(n, start, &graph, &mut distances_go);

    for i in 1..=n {
        if i == start { continue; }
        come_dijkstra(n, i, to, &graph, &mut distances_come);
    }

    for i in 1..=n {
        let d = distances_go[i] + distances_come[i];
        result = max(result, d);
    }

    writeln!(output, "{}", result)?;
    Ok(())
}