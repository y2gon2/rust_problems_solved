//!  https://www.acmicpc.net/problem/9370
//! 미확인 도착지
//! https://www.acmicpc.net/board/view/76830


use std::io::stdin;
use std::fmt::Write;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::num::ParseIntError;
use std::slice::from_raw_parts;

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
    expectation: Vec<usize>,
    destinations: Vec<usize>,
}

impl Target {
    fn new() -> Self {
        Self {
            start: 0,
            expectation: Vec::<usize>::new(),
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
    in_the_middle: (usize, usize),
    graph: Vec<Vec<(usize, usize)>>,
}

impl Tailing {
    fn new() -> Self {
        Self {
            target: Target::new(),
            in_the_middle: (0, 0),
            graph: Vec::<Vec<(usize, usize)>>::new(),
        }
    }

    fn get_tailing_data(&mut self) -> Result<(), Box<dyn Error>> {
        let (n, m, t) = get_nums()?;
        let (start, middle1, middle2) = get_nums()?;
        
        let mut graph = vec![vec![]; n + 1];
        for _ in 0..m {
            let (from, to, weight) = get_nums()?;

            graph[from].push((to, weight));
            graph[to].push((from, weight));
        }

        let mut expectation = vec![0usize; t];
        for i in 0..t {
            expectation[i] = get_num()?;
        }

        self.target.start = start;
        self.target.expectation = expectation;
        self.target.destinations = Vec::<usize>::new();
        self.in_the_middle = (middle1, middle2);
        self.graph = graph;

        Ok(())
    }

    fn dijkstra(&mut self, from: usize) -> Vec<usize> {
        let mut shortcuts = vec![LIMIT; self.graph.len() + 1];
        shortcuts[from] = 0;

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(QueueItem(from, 0));

        while let Some(QueueItem(cur, acc)) = priority_queue.pop() {            
            for (next, weight) in &self.graph[cur] {
                let sum = acc + *weight;

                if sum > shortcuts[*next] { continue; }

                shortcuts[*next] = sum;
                priority_queue.push(QueueItem(*next, sum));
            }
        }
        shortcuts
    }

    fn find_destination(&mut self) {
        let ex = self.target.expectation.clone();

        for e in ex.iter() {
            // 출발지에서 모든 위치까지 최단 거리
            let from_start = self.dijkstra(self.target.start);
            // println!("from_start: {:?}", from_start);

            // 목적지에서 모든 위치까지 최단 거리
            let from_e = self.dijkstra(*e);
            // println!("from_e: {:?}", from_e);

            let mut smelling_cross_weight: usize = 0usize;
            for (middle2, weight) in self.graph[self.in_the_middle.0].iter() {
                if middle2 == &self.in_the_middle.1 {
                    smelling_cross_weight = *weight;
                }
            }

            if from_start[*e] == (from_start[self.in_the_middle.0] + smelling_cross_weight + from_e[self.in_the_middle.1]) 
            || from_start[*e] == (from_start[self.in_the_middle.1] + smelling_cross_weight + from_e[self.in_the_middle.0]) {
                self.target.destinations.push(*e);
            }
        }
    }

    fn clear(&mut self) {
        self.target =  Target::new();
        self.in_the_middle = (0, 0);
        self.graph = Vec::<Vec<(usize, usize)>>::new();
    } 
}

fn get_num() -> Result<usize, ParseIntError> {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    
    buf.trim().parse::<usize>()
}

fn get_nums() -> Result<(usize, usize, usize), ParseIntError> {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    
    let mut nums_iter = buf.split_ascii_whitespace();
    let mut get_n = || nums_iter
        .next()
        .unwrap()
        .parse::<usize>();

    Ok((get_n()?, get_n()?, get_n()?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut tailing = Tailing::new();

    let t = get_num()?;
    for _ in 0..t {
        tailing.get_tailing_data()?;
        tailing.find_destination();

        // println!("{:?}", tailing);
        writeln!(output, "{}", tailing.target.print_destinations())?;
        tailing.clear();
    }
    println!("{output}");
    Ok(())
}