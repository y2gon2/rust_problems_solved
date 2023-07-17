//! https://www.acmicpc.net/problem/2583
//! 영역 구하기

use std::io::{stdin, stdout, Write, BufRead};
use std::error::Error;
use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn bfs(
    matrix: &mut Vec<Vec<bool>>,
    result: &mut Vec<usize>,
    n: usize,
    m: usize 
) -> usize {
    let mut cnt = 0usize;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] { continue; }

            cnt += 1;
            let mut sub_sq = 1usize;
            let mut queue = VecDeque::<(usize, usize)>::new();

            queue.push_back((i, j));
            matrix[i][j] = true;

            while let Some((cy, cx)) = queue.pop_front() {
                for (dy, dx) in DIRECTIONS {
                    let y = cy as i32 + dy;
                    let x = cx as i32 + dx;
                    
                    if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                        let y = y as usize;
                        let x = x as usize;

                        if matrix[y][x] { continue; }
                        
                        queue.push_back((y, x));
                        sub_sq += 1;
                        matrix[y][x] = true;
                    }
                }
            }
            result.push(sub_sq);
        }
    }
    cnt
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut lines = stdin().lock().lines();

    let mut get_nums = || lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| e);
        
    let info = get_nums()?;
    let mut matrix = vec![vec![false; info[1]]; info[0]];
    
    for _ in 0..info[2] {
        let sq = get_nums()?;

        for y in sq[1]..sq[3] {
            for x in sq[0]..sq[2] {
                if !matrix[y][x] {
                    matrix[y][x] = true;
                } 
            }
        } 
    } 

    let mut result = Vec::<usize>::new();
    let cnt = bfs(&mut matrix, &mut result, info[0], info[1]);

    result.sort();
    let ans = result
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(output, "{}\n{}", cnt, ans)?;     
    Ok(())
}