//! https://www.acmicpc.net/problem/1504
//! 특정한 최단 경로


use std::io::{stdin, stdout, Read, Write};
use std::collections::BinaryHeap;
use std::cmp::{min, Ord, Ordering, PartialOrd};

const LIMIT: usize = usize::MAX / 3 - 1;

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug)]
struct Route {
    n: usize,
    waypoints: (usize, usize),
    graph: Vec<Vec<(usize, usize)>>,
}

impl Route {
    fn new(n: usize, waypoints: (usize, usize), graph: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            n,
            waypoints,
            graph,
        }
    }

    fn create_from_input() -> Self {
        let mut buf = String::new();
        let _ = stdin().read_to_string(&mut buf);
        let mut input = buf.split_ascii_whitespace();
        let mut get_num = || input
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let n = get_num();
        let m = get_num();
        let mut graph = vec![vec![]; n + 1];

        for _ in 0..m {
            let (from, to, weight) = (get_num(), get_num(), get_num());

            graph[from].push((to, weight));
            graph[to].push((from, weight));            
        }
        
        let waypoints: (usize, usize) = (get_num(), get_num());
        
        Route::new(n, waypoints, graph)
    }

    fn dijkstra(&mut self, start: usize, target: usize) -> usize{
        let mut routing_table = vec![LIMIT; self.n + 1];
        let mut priority_queue = BinaryHeap::<QueueItem>::new();

        priority_queue.push(QueueItem(start, 0));
        while let Some(QueueItem(cur, acc)) = priority_queue.pop() {
            if cur == target {
                return acc
            }

            for (next, weight) in self.graph[cur].iter() {
                let sum = acc + *weight;

                if sum > routing_table[*next] { continue; }

                routing_table[*next] = sum;
                priority_queue.push(QueueItem(*next, sum));
            }
            // println!("cur: {cur} acc: {acc}  {:?} {:?}", &priority_queue, &routing_table);
        }
        routing_table[target]
    }

    fn measurement(&mut self) -> usize {
        // 1 -> A or B
        let one_to_a = self.dijkstra(1, self.waypoints.0);
        let one_to_b = self.dijkstra(1, self.waypoints.1);

        // A or B -> n
        let b_to_n = self.dijkstra(self.waypoints.1, self.n);
        let a_to_n = self.dijkstra(self.waypoints.0, self.n);

        // min(A, B) + (A -> B)
        let mut result = min(one_to_a + b_to_n, one_to_b + a_to_n);        
        result += self.dijkstra(self.waypoints.0, self.waypoints.1);

        // println!("{result}");
        result
    }
}


fn main() {
    let mut output = stdout();
    let mut route = Route::create_from_input();

    let result = route.measurement();

    if result >= LIMIT {
        writeln!(output, "-1").unwrap();
    } else {
        writeln!(output, "{}", result).unwrap();
    }
}