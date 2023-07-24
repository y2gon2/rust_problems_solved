//!  https://www.acmicpc.net/problem/9370
//! 미확인 도착지


use std::io::{Read, stdin};
use std::fmt::Write;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::str::SplitAsciiWhitespace;
use std::num::ParseIntError;

const LIMIT: usize = (usize::MAX / 3) - 1;

#[derive(Debug, Eq, PartialEq)]
struct QueueItem(usize, usize); // next, weigth

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

#[derive(Debug)]
struct Target {
    start: usize,
    in_the_middle: (usize, usize),
    expectation: Vec<usize>,
    destinations: Vec<usize>,
}

impl Target {
    fn new(
        start: usize, 
        in_the_middle: (usize, usize), 
        expectation: Vec<usize>, 
    ) -> Self {
        Self {
            start,
            in_the_middle,
            expectation,
            destinations: Vec::<usize>::new(),
        }
    }

    fn print_destinations(&self) -> String {
        let mut dest = self.destinations.clone();
        dest.sort();

        dest
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}


#[derive(Debug)]
struct Tailing {
    target: Target,
    graph: Vec<Vec<(usize, usize)>>,
}

impl Tailing {
    fn new(target: Target, graph: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            target,
            graph,
        }
    }

    fn get_tailing_data(buf_iter: &mut SplitAsciiWhitespace<'_>) -> Result<Tailing, Box<dyn Error>> {
        let n = get_num(buf_iter)?;
        let m = get_num(buf_iter)?;
        let t = get_num(buf_iter)?;
        let start = get_num(buf_iter)?;
        let in_the_middle: (usize, usize) = (get_num(buf_iter)?, get_num(buf_iter)?);
        
        let mut graph = vec![vec![]; n + 1];
        for _ in 0..m {
            let from = get_num(buf_iter)?;
            let to = get_num(buf_iter)?;
            let weight = get_num(buf_iter)?;
            
            graph[from].push((to, weight));
            graph[to].push((from, weight));
        }

        let mut expectation = vec![0usize; t];
        for i in 0..t {
            expectation[i] = get_num(buf_iter)?;
        }

        let target = Target::new(
            start, 
            in_the_middle, 
            expectation, 
        );

        Ok((Self::new(target, graph)))
    }

    fn dijkstra(&mut self, from: usize, to: usize) -> usize {
        let mut shortcuts = vec![LIMIT; self.graph.len() + 2];
        shortcuts[from] = 0;

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(QueueItem(from, 0));

        while let Some(QueueItem(cur, acc)) = priority_queue.pop() {
            if cur == to { return shortcuts[cur] }
            
            for (next, weight) in &self.graph[cur] {
                let sum = acc + *weight;

                if sum > shortcuts[*next] { continue; }

                shortcuts[*next] = sum;
                priority_queue.push(QueueItem(*next, sum));
            }
        }
        shortcuts[to]
    }

    fn find_destination(&mut self) {
        let ex = self.target.expectation.clone();

        for e in ex.iter() {
            // 출발지에서 예상 목적지까지 한번에 갔을 때 최단 거리
            let s_to_t = self.dijkstra(self.target.in_the_middle.0, *e);

            // start -> A or B
            let s_to_a = self.dijkstra(self.target.start, self.target.in_the_middle.0);
            let s_to_b = self.dijkstra(self.target.start, self.target.in_the_middle.1);
            
            // A <-> B
            let a_to_b = self.dijkstra(self.target.in_the_middle.0, self.target.in_the_middle.1);

            // A or B -> target
            let b_to_e = self.dijkstra(self.target.in_the_middle.1, *e);
            let a_to_e = self.dijkstra(self.target.in_the_middle.0, *e);

            println!("[{e}] {s_to_t} : [{}] {s_to_a} -> [{}] {a_to_b} -> [{}] {b_to_e}  / [{}] {s_to_b} -> [{}] {a_to_b} -> [{}] {a_to_e}", self.target.start, self.target.in_the_middle.0, self.target.in_the_middle.1, self.target.start, self.target.in_the_middle.1, self.target.in_the_middle.0);
            if s_to_t == (s_to_a + a_to_b + b_to_e) || s_to_t == (s_to_b + a_to_b + a_to_e) {
                self.target.destinations.push(*e);
            }
        }
    }
}

fn get_num(buf_iter: &mut SplitAsciiWhitespace<'_>) 
    -> Result<usize, ParseIntError> {
    buf_iter.next().unwrap().parse::<usize>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();

    let t = get_num(&mut input)?;
    for _ in 0..t {
        let mut tailing = Tailing::get_tailing_data(&mut input)?;
        tailing.find_destination();

        // println!("{:?}", tailing);
        writeln!(output, "{}", tailing.target.print_destinations())?;
    }
    println!("{output}");
    Ok(())
}


