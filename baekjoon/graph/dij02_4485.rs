//! https://www.acmicpc.net/problem/4485
//! 녹색 옷 입은 애가 젤다지?
//! 

use std::io::{stdin, stdout, Read};
use std::error::Error;
use std::str::SplitAsciiWhitespace;
use std::num::ParseIntError;
use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt::Write;

#[derive(Eq, PartialEq, Debug)]
struct QueueItem(usize, usize, u16);

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.2.cmp(&other.2))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.cmp(&other.2)
    }   
}

fn dijkstra(n: usize, matrix: Vec<Vec<u16>>) -> u16 {
    let mut dij_matrix = vec![vec![u16::MAX; n]; n];
    let mut priority_queue = BinaryHeap::<QueueItem>::new();

    priority_queue.push(QueueItem(0, 0, matrix[0][0]));
    dij_matrix[0][0] = matrix[0][0];

    while let Some(QueueItem(cy, cx, weight)) = priority_queue.pop() {
        let y = cy + 1;
        if y < n {
            let sum = dij_matrix[cy][cx] + matrix[y][cx];
            if sum < dij_matrix[y][cx] { 
                dij_matrix[y][cx] = sum; 
                priority_queue.push(QueueItem(y, cx, sum));
            }

        }

        let x = cx + 1;
        if x < n {
            let sum = dij_matrix[cy][cx] + matrix[cy][x];
            if sum < dij_matrix[cy][x] { 
                dij_matrix[cy][x] = sum; 
                priority_queue.push(QueueItem(cy, x, sum));
            }

        }
    }

    dij_matrix[n - 1][n - 1]
}


fn get_usize(input: &mut SplitAsciiWhitespace<'_>) -> Result<usize, ParseIntError> {
    input.next().unwrap().parse::<usize>()
}

fn get_matrix(n: usize, input: &mut SplitAsciiWhitespace<'_>) -> Result<Vec<Vec<u16>>, ParseIntError> {
    let mut matrix = vec![vec![0u16; n]; n];
        
    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = input.next().unwrap().parse::<u16>()?;
        }
    }
    Ok(matrix)
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();

    let times = get_usize(&mut input)?;
    for _ in 0..times {
        let n = get_usize(&mut input)?;
        let matrix = get_matrix(n, &mut input)?;
        writeln!(output, "{}", dijkstra(n, matrix))?;
    }
    println!("{}", output);
    Ok(())
}