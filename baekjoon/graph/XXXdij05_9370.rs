//!  https://www.acmicpc.net/problem/9370
//! 미확인 도착지


use std::io::{Read, stdin};
use std::fmt::Write;
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::str::SplitAsciiWhitespace;
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
struct QueueItem(usize, usize, Vec<usize>); // next, weigth, route

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
    priority_queue: BinaryHeap<QueueItem>,
    shortcuts: Vec<usize>,
}

impl Tailing {
    fn new(target: Target, graph: Vec<Vec<(usize, usize)>>, shortcuts: Vec<usize>) -> Self {
        Self {
            target,
            graph,
            priority_queue: BinaryHeap::<QueueItem>::new(),
            shortcuts,
        }
    }

    fn get_tailing_data(buf_iter: &mut SplitAsciiWhitespace<'_>) -> Result<Tailing, Box<dyn Error>> {
        let n = get_num(buf_iter)?;
        let m = get_num(buf_iter)?;
        let t = get_num(buf_iter)?;
        let start = get_num(buf_iter)?;
        let shortcuts = vec![usize::MAX; n + 1];
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

        Ok((Self::new(target, graph, shortcuts)))
    }

    fn find_destination(&mut self) {
        self.shortcuts[self.target.start] = 0;
        self.priority_queue.push(QueueItem(self.target.start, 0, vec![self.target.start]));

        while let Some(QueueItem(cur, acc, route)) = self.priority_queue.pop() {
            if self.target.expectation.contains(&cur) {
                for i in 1..route.len() {
                    if (route[i - 1] == self.target.in_the_middle.0 
                    && route[i] == self.target.in_the_middle.1)
                    || (route[i] == self.target.in_the_middle.0 
                    && route[i - 1] == self.target.in_the_middle.1) {
                        if self.target.destinations.contains(&cur) { continue; }
                        self.target.destinations.push(cur);

                        // println!("route:{:?}", &route);
                        // println!("dest :{:?}", self.target.destinations);
                    }
                }
            }
            
            for (next, weight) in &self.graph[cur] {
                let sum = acc + *weight;

                if sum > self.shortcuts[*next] { continue; }

                self.shortcuts[*next] = sum;
                let mut new_route = route.clone();
                new_route.push(*next);

                self.priority_queue.push(QueueItem(*next, sum, new_route));
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


