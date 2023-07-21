//! https://www.acmicpc.net/problem/4485
//! 녹색 옷 입은 애가 젤다지?

use std::io::{stdin, Read};
use std::error::Error;
use std::str::SplitAsciiWhitespace;
use std::num::ParseIntError;
use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Ordering};
use std::fmt::Write;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Eq, PartialEq, Debug)]
struct QueueItem(i32, i32, u16);

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.2.cmp(&self.2))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }   
}

fn dijkstra(n: usize, matrix: Vec<Vec<u16>>) -> u16 {
    let mut dij_matrix = vec![vec![u16::MAX; n]; n];
    let mut priority_queue = BinaryHeap::<QueueItem>::new();

    priority_queue.push(QueueItem(0, 0, matrix[0][0]));
    dij_matrix[0][0] = matrix[0][0];

    while let Some(QueueItem(cy, cx, weight)) = priority_queue.pop() {
        // println!("{:?}", priority_queue);
        if cy as usize == n - 1 && cx as usize == n - 1 { 
            // for i in 0..n {
            //     println!("{:?}", dij_matrix[i]);
            // }
            // println!("---------------------------");

            return dij_matrix[n - 1][n - 1]; 
        }
        
        for (dy, dx) in DIRECTIONS {
            let iy = cy + dy;
            let ix = cx + dx;

            if iy >= 0 && ix >= 0 && iy < n as i32 && ix < n as i32 {
                let y = iy as usize;
                let x = ix as usize;

                let sum = weight + matrix[y][x];
                if dij_matrix[y][x] > sum { 
                    dij_matrix[y][x] = sum; 
                    priority_queue.push(QueueItem(iy, ix, sum));
                }
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

    let mut n = get_usize(&mut input)?;
    let mut cnt = 1u8;
    while n != 0 {
        let matrix = get_matrix(n, &mut input)?;
        writeln!(output, "Problem {}: {}", cnt, dijkstra(n, matrix))?;
        
        cnt += 1;
        n = get_usize(&mut input)?;
    }
    println!("{}", output);
    Ok(())
}