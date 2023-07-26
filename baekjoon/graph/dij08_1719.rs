//! https://www.acmicpc.net/problem/1719
//! 택배
//! 
//! 풀이과정
//! 1. priority queue 에 모든 경로 vec 을 포함한 dijkstra 구성
//!     14232KB / 64ms
//! 
//! 2. 모든 경로를 포함하지 않고, 두번째 경로까지만 포함, 그 이후는 버림
//!     14232KB / 48ms
//! 
//! 3. 경로 vec 이 아닌 두번째 경우 값만 u8 로 넣고 이를 저장할 table 도 u8 로 변경
//!     13984KB / 32ms
//! 
//! 4. dijkstra 탐색 중 처음에는 넣어졌으나 상대적 우선순위가 밀려 후순위로 꺼내진 것들은
//!    더이상 탐색을 할 필요가 없으므로 if dist_table[cur] < acc { continue; } 코드 추가
//!     13984KB / 20ms

use std::io::{stdin, Read};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Clone, Debug, Eq, PartialEq)]
struct QueueItem(usize, usize, u8);

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

    for _ in 0..m {
        let (from, to, weight) = (get_num()?, get_num()?, get_num()?);
        graph[from].push((to, weight));
        graph[to].push((from, weight));
    }

    Ok((n, graph))
}

fn dijkstra (n: usize, graph: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<u8>> {
    let mut ans_table = vec![vec![0u8; n + 1]; n + 1];

    for i in 1..=n {
        let mut dist_table = vec![usize::MAX; n + 1];
        let mut priority_queue = BinaryHeap::<QueueItem>::new();
        priority_queue.push(QueueItem(i, 0, u8::MAX));
    
        dist_table[i] = 0;
        while let Some(QueueItem(cur, acc, route)) = priority_queue.pop() {
            if dist_table[cur] < acc { continue; }

            for (next, weight) in graph[cur].iter() {
                let sum = acc + weight;
    
                if sum > dist_table[*next] { continue; }
                let mut c_route = route.clone();
                if c_route == u8::MAX { c_route = *next as u8; }
                
                dist_table[*next] = sum;
                ans_table[i][*next] = c_route;

                priority_queue.push(QueueItem(*next, sum, c_route));
            }
        }    
    } 
    ans_table
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let (n, graph) = get_info()?;

    let ans_table = dijkstra(n, &graph);

    for i in 1..=n {
        for j in 1..=n {
            if i == j  {
                output += "- ";
            } else {
                output += &ans_table[i][j].to_string();
                output += " ";
            }
        }
        output += "\n";
    }
    println!("{output}");
    Ok(())
}

// fn floyd_warshall_with_path(graph: &mut Vec<Vec<i32>>) -> Vec<Vec<Option<u8>>> {
    //     let len = graph.len();
    //     let mut prevs: Vec<Vec<_>> = graph
    //         .iter()
    //         .enumerate()
    //         .map(|(start, row)| {
    //             row.iter()
    //                 .map(|&dist| (dist != i32::MAX).then(|| start as u8))
    //                 .collect()
    //         })
    //         .collect();
    
    //     for stopby in 0..len {
    //         for start in 0..len {
    //             for end in 0..len {
    //                 let new_dist = graph[start][stopby].saturating_add(graph[stopby][end]);
    
    //                 if new_dist < graph[start][end] {
    //                     graph[start][end] = new_dist;
    //                     prevs[start][end] = prevs[stopby][end];
    //                 }
    //             }
    //         }
    //     }
    
    //     prevs
    // }