//! https://www.acmicpc.net/problem/14938
//! 서강그라운드

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd, max};
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
struct QueueItem(usize, u16);

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

#[derive(Debug, Clone)]
struct Farming {
    n: usize,
    m: u8,
    graph: Vec<Vec<(usize, u8)>>,
    items: Vec<u8>,
    shortcuts: Vec<u16>,
}

impl Farming {
    fn new(n: usize, m: u8, graph: Vec<Vec<(usize, u8)>>, items: Vec<u8>, shortcuts: Vec<u16>) -> Self {
        Self {
            n,
            m,
            graph, 
            items,
            shortcuts,
        }
    }    

    fn get_info() -> Result<Self, ParseIntError> {
        let mut buf = String::new();
        let _ = stdin().read_to_string(&mut buf);
        let mut input = buf.split_ascii_whitespace();
        let mut str_iter = || input.next().unwrap();

        let n = str_iter().parse::<usize>()?;
        let m = str_iter().parse::<u8>()?;
        let r = str_iter().parse::<u8>()?;

        let mut items = vec![0u8; n + 1];
        for i in 1..=n {
            items[i] = str_iter().parse::<u8>()?;
        } 

        let mut graph = vec![vec![]; n + 1];
        for _ in 0..r {
            let from = str_iter().parse::<usize>()?;
            let to = str_iter().parse::<usize>()?;
            let weight = str_iter().parse::<u8>()?;

            graph[from].push((to, weight));
            graph[to].push((from, weight));
        }

        let shortcuts = vec![u16::MAX; n + 1];
        Ok(Farming::new(n, m, graph, items, shortcuts))
    }

    fn dijkstra(&mut self, from: usize) {
        let mut priority_queue = BinaryHeap::<QueueItem>::new();
        priority_queue.push(QueueItem(from, 0));
        self.shortcuts[from] = 0;

        while let Some(QueueItem(cur, acc)) = priority_queue.pop() {
            for (next, weight) in self.graph[cur].iter() {
                let sum = acc + *weight as u16;

                if sum > self.shortcuts[*next] { continue; }

                self.shortcuts[*next] = sum;
                priority_queue.push(QueueItem(*next, sum));
            }
        }
        // println!("{:?}", self.shortcuts); 
    }

    fn collecting(&mut self) -> u16 {
        let mut result = 0u16;

        for i in 1..=self.n {
            self.dijkstra(i);
            
            let mut sum = 0u16;
            for j in 1..=self.n {
                if self.shortcuts[j] <= self.m as u16 {
                    sum += self.items[j] as u16;
                }
            }

            result = max(result, sum);
            self.shortcuts = vec![u16::MAX; self.n + 1];  
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut farming = Farming::get_info()?;

    writeln!(output, "{}", farming.collecting())?;
    // println!{"{:?}", farming};
    Ok(())
}