//! https://www.acmicpc.net/problem/2665
//! 미로만들기
//! 
//! 풀이 현황
//! DFS 로 목적지까지 이동하되, 벽을 뚫을 횟수를 같이 저장하면서
//! 또한 벽을 뚫으면서 이동한 경우 각 뚫은 횟수마다 visite 여부를 각각 저장
//! +
//! 그냥 모두 저장하면서 모든 경우의 수를 brute force 형태로 모두 탐색하면
//! 시간 초과 또는 메모리 초과가 발생하므로, 우선순위 큐의 형태로 
//! 가장 적게 벽을 뚫고 온 경우를 먼저 탐색하도록 해야함. 
//! -> 19368KB / 8ms
//! 
//! 메모리 사용을 줄이기 위해 visited 에서 벽을 뚫는 횟수에 대한 공간을 전체 벽의 수만큼만 구성
//! -> 16200KB / 8ms
//! 
//! 기존의 우선순위 큐를  BinaryHeap 을 사용하여 구현하였으나
//! 더 효율적 구현을 위해 기존의 VecDeque 를 사용하되,
//! 현재 탐색에서 벽을 뚫지 않는 경우는 push_front 로 앞에 넣어주고
//! 벽을 뚫는 경우는 push_back 으로 뒤에 넣어서 탐색
//! -> 16299KB / 4ms


use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;
use std::collections::VecDeque;

const DIRECTIONS:[(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_maze() -> Result<(usize, Vec<Vec<bool>>, usize), std::num::ParseIntError> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();

    let n = input
        .next()
        .unwrap()
        .parse::<usize>()?;

    let mut maze = vec![vec![false; n]; n];
    let mut cnt = 0usize;
    for i in 0..n {
        let row = input
            .next()
            .unwrap()
            .trim();
        for (idx, ch) in row.char_indices() {
            if ch == '0' {
                maze[i][idx] = true;
                cnt += 1;
            }
        }
    }
    Ok((n, maze, cnt))
}

fn make_maze() -> Result<u8, Box<dyn Error>> {
    let (n, maze, walls) = get_maze()?;
    let ni8 = n as i8;

    let mut priority_queue = VecDeque::<(u8, i8, i8)>::new();
    let mut visited = vec![vec![vec![false; walls + 1]; n]; n];
    let mut wrecked = 0u8;

    if maze[0][0] { wrecked = 1; }
    priority_queue.push_back((wrecked, 0, 0));
    visited[0][0][0] = true;
    
    while let Some((cnt, cy, cx)) = priority_queue.pop_front() {
        // println!("cnt: {}, y: {}, x: {}", cnt, cy, cx);
        if cy == ni8 - 1 && cx == ni8 - 1 {
            return Ok(cnt);
        }

        for (dy, dx) in DIRECTIONS {
            let iy = cy + dy;
            let ix = cx + dx;

            if iy >= 0 && ix >= 0 && iy < ni8  && ix < ni8 {
                let y = iy as usize;
                let x = ix as usize;
                let mut c = cnt;

                if maze[y][x] { 
                    c += 1; 
                    if visited[y][x][cnt as usize] { continue; } 

                    visited[y][x][cnt as usize] = true;
                    priority_queue.push_back((c, iy, ix));
                } else {
                    if visited[y][x][cnt as usize] { continue; } 

                    visited[y][x][cnt as usize] = true;
                    priority_queue.push_front((c, iy, ix));
                }
            }
        }
        // println!("{:?}", priority_queue);
    }

    Ok(wrecked)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = make_maze()?;

    let mut output = String::new();
    writeln!(output, "{}", result)?;
    println!("{}", output);
    Ok(())
}