// https://www.acmicpc.net/problem/7569
// 토마토

use std::io::{stdin, stdout, BufRead, Write};
use std::error::Error;
use std::collections::VecDeque;

fn bfs(size: Vec<usize>, boxes: &mut Vec<Vec<Vec<i32>>>, start_q: &mut VecDeque::<(usize, usize, usize)>) -> Result<i32, Box<dyn Error>> {
    let (n, m, h) = (size[0], size[1], size[2]);
    let mut result: i32 = 0;

    let direction = [[0,0,1],[0,0,-1],[0,1,0],[0,-1,0],[1,0,0],[-1,0,0]];

    while let Some(origin) = start_q.pop_front()  {
        let mut infested_q = VecDeque::<((usize, usize, usize), i32)>::new();
        infested_q.push_back((origin, 1));
        
        while let Some(((iz, iy, ix), cnt)) = infested_q.pop_front() {
            for d in direction {
                let z = iz as i32 + d[0];
                let y = iy as i32 + d[1];
                let x = ix as i32 + d[2];
                
                if z >= 0 && y >= 0 && x >= 0 && z < h as i32 && y < m as i32 && x < n as i32 {
                    let z = z as usize;
                    let y = y as usize;
                    let x = x as usize;
                    let cnt = cnt + 1; 
            
                    if boxes[z][y][x] == -1 || boxes[z][y][x] == 1 { continue; }
                    if boxes[z][y][x] > cnt {
                        boxes[z][y][x] = cnt;
                        infested_q.push_back(((z, y, x), cnt));
                    }
                }
            }
        } 
    }
    // println!("{:?}", boxes);

    for i in 0..h {
        for j in 0..m {
            for k in 0..n {
                if boxes[i][j][k] == i32::MAX {
                    return Ok(-1);
                }
                result = result.max(boxes[i][j][k]);
            }
        }
    } 
    return Ok(result - 1);
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut input = stdin().lock().lines().map(|line| line);

    let size: Vec<usize> = input
        .next()
        .unwrap()?
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;


    let mut boxes = Vec::<Vec<Vec<i32>>>::new();
    let mut start_q = VecDeque::<(usize, usize, usize)>::new();
    for i in 0..size[2] {
        let mut inside = Vec::<Vec<i32>>::new();

        for j in 0..size[1] {
            let mut input: Vec<i32> = input
            .next()
            .unwrap()?
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()
            .map_err(|e| e)?;
            
            for k in 0..size[0] {
                if input[k] == 1 {
                    start_q.push_back((i, j, k));
                } else if input[k] == 0 {
                    input[k] = i32::MAX;
                }
            } 
            inside.push(input);
        }
        boxes.push(inside);
    }

    let mut output = stdout().lock();
    writeln!(output, "{}", bfs(size, &mut boxes, &mut start_q)?)?;

    Ok(())
}