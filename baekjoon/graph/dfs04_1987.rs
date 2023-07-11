//! https://www.acmicpc.net/problem/1987
//! 알파벳
//! 
//! 풀이과정
//! DFS 풀이에 재귀가 아닌 stack 상 해당 포인트 까지 온  경로를 같이 저장하는 방식으로 구현했을때,
//! 시간 초과 발생   
//! 최초 재귀 구현 : 13164KB / 588ms
//! 
//! 주어지는 matrix 를 이중 vec 이 아닌 제한 size 로 고정된 array 로 수정
//! -> 13156KB / 492ms
//! 
//! rust 의 ownership 문제 때문에 재귀함수 구현에 RC<RefCell<_>> 을 고려하였으나,
//! 순수 (mutable & immutable) reference type paremeter 만으로 구현 가능.
//! 

use std::io::{stdin, stdout, Write, Read};
use std::error::Error;
use std::cmp::max;

const MAX: usize = 20;
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_input() -> Result<(i32, i32, [[char; 20]; 20]), Box<dyn Error>> {
    let mut board = [[0 as char; MAX]; MAX];
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf)?;

    let mut buf_iter = buf
        .split_ascii_whitespace();

    let (r, c)  = (
        buf_iter.next().unwrap().parse::<i32>()?,
        buf_iter.next().unwrap().parse::<i32>()?
    );

    for i in 0..r as usize {
        for (j, ch) in buf_iter.next().unwrap().char_indices() {
            board[i][j] = ch;
        }
    }

    Ok((r, c, board))
}

fn dfs(
    board: &[[char; 20]; 20], 
    visited: &mut [bool; 26], 
    r: i32, 
    c: i32,
    cnt: usize,
    y: i32,
    x: i32,
    result: &mut usize
) {
    *result = max(*result, cnt);
    // println!("cnt:{}, result:{}", cnt, result);

    for (dy, dx) in DIRECTIONS.iter() {
        let ny = y + *dy;
        let nx = x + *dx;

        if ny >= 0 && nx >= 0 && ny < r  && nx < c {
            let ny = ny as usize;
            let nx = nx as usize; 
            
            let index = (board[ny][nx] as u8 - b'A') as usize;
            if visited[index] { continue; }

            visited[index] = true;

            dfs(
                board,
                visited,
                r,
                c,
                cnt + 1,
                ny as i32,
                nx as i32,
                result,
            );

            visited[index] = false;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let (r, c, board) = get_input()?;
    let mut visited = [false; 26];

    visited[(board[0][0] as u8 - b'A') as usize] = true;

    let mut result = 1usize;
    
    dfs(
        &board, 
        &mut visited, 
        r, 
        c, 
        1, 
        0, 
        0,
        &mut result
    );

    writeln!(output, "{}", result)?;
    Ok(())
}
