//! https://www.acmicpc.net/problem/1753
//! 최단경로 성공
//! 
//! source code
//! https://www.acmicpc.net/source/56594687


use std::fmt::Write;
use std::io;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};
use std::collections::{BinaryHeap, HashMap};


#[derive(Debug, Clone)]
struct Node{
    visit : bool,
    distance : Option<u32>,
    neighbors : HashMap<usize, u32>,
}

impl Node{
    fn new() -> Self{
        Self{
            visit : false,
            distance : None,
            neighbors : HashMap::new(),
        }
    }

    fn add_edge(&mut self, idx : usize, weight : u32){
        if let Some(x) = self.neighbors.get_mut(&idx){
            if weight < *x {
                *x = weight;
            }
        } else {
            self.neighbors.insert(idx, weight);
        }
    }
}

struct Graph(Vec<Node>);

#[derive(Eq, PartialEq, Debug)]
struct QueueItem(usize, u32);

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type PQueue = BinaryHeap<QueueItem>;

impl Graph {
    fn new(length : usize) -> Self{
        Self(vec![Node::new(); length])
    }

    fn print(&self){
        let mut output = String::new();

        for node in self.0.iter(){
            match node.distance {
                Some(d) => writeln!(output, "{d}"),
                None => writeln!(output, "INF")
            }.unwrap()
        }

        println!("{output}");
    }

    fn dijkstra(&mut self, start : usize){
        let mut queue = PQueue::new();
        
        self.0[start].distance = Some(0);
        queue.push(QueueItem(start, 0));

        while let Some(QueueItem(idx, distance)) = queue.pop() {
            if self.0[idx].visit{
                continue;
            } else {
                self.0[idx].visit = true;
            }

            for (to, weight) in self.0[idx].neighbors.clone().drain(){
                let new_d = distance + weight;
                let mut flag = false;

                match self.0[to].distance {
                    Some(x) => {
                        if new_d < x {
                            self.0[to].distance = Some(new_d);
                            flag = true;
                        }
                    },
                    None => {
                        self.0[to].distance = Some(new_d);
                        flag = true;
                    }
                }

                if flag {
                    queue.push(QueueItem(to, new_d));
                }
            }
        }
    }
}

fn main(){
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (num_nodes, num_edges, start_node) = (input(), input(), input());
    let mut graph = Graph::new(num_nodes);

    for _i in 0..num_edges{
        let (idx1, idx2, weight) = (input(), input(), input() as u32);
        graph.0[idx1 - 1].add_edge(idx2 - 1, weight);
    }

    graph.dijkstra(start_node - 1);
    graph.print();
}
