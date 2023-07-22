//! https://www.acmicpc.net/problem/11779
//! 최소비용 구하기 2 
//! 

use std::io::{stdin, Read};
use std::fmt::Write;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;
use std::error::Error;


#[derive(Debug, Eq, PartialEq)]
struct QueueItem(usize, usize, Vec<usize>);  // next, weight, route

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

fn get_info() -> Result<(usize, usize, usize, Vec<Vec<(usize, usize)>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_num()?, get_num()?);
    let mut graph = vec![vec![]; n + 1];

    for _ in 1..=m {
        graph[get_num()?].push((get_num()?, get_num()?));
    }
    
    let (from, to) = (get_num()?, get_num()?);

    Ok((n, from, to, graph))
}

fn dijkstra() -> Result<(usize, Vec<usize>), Box<dyn Error>> {
    let (n, start, goal, graph) = get_info()?;
    let mut consuming_list = vec![usize::MAX; n + 1];

    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    priority_queue.push(QueueItem(start, 0, vec![start]));
    consuming_list[start] = 0 ;

    while let Some(QueueItem(cur, acc, route)) = priority_queue.pop() {
        if cur == goal {
            return Ok((acc, route));
        }

        for (next, weight) in graph[cur].iter() {
            let sum = acc + weight;

            if sum < consuming_list[*next] {
                consuming_list[*next] = sum;

                let mut next_route = route.clone();
                next_route.push(*next);
                
                priority_queue.push(QueueItem(*next, sum, next_route));
            }
        }
    }
    Err("Could not find answers".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();

    let (weight, route) = dijkstra()?;

    writeln!(output, "{}\n{}\n{}", 
        weight,
        route.len(),
        route.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")
    )?;
    println!("{output}");
    Ok(())
}