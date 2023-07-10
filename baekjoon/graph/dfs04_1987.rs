//! https://www.acmicpc.net/problem/1987
//! 알파벳
//! 

use std::io::{stdin, stdout, Write, Read, BufRead};
use std::error::Error;
use std::collections::VecDeque;

fn get_nums() -> Result<(usize, usize), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_line(&mut buf)?;

    let info: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;  

    Ok((info[0], info[1]))
}

fn get_board() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let board: Vec<Vec<char>> = buf
        .split_ascii_whitespace()
        .map(|s| {
            s.chars().map(|c| c).collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    
    Ok(board)
}

fn dfs(board: Vec<Vec<char>>, r: usize, c: usize) -> Result<usize, Box<dyn Error>> {
    let mut visited = vec![vec![0u8; c]; r];
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut aphabets = Vec::<char>::new();
    let mut stack = Vec::<(usize, usize)>::new(); // y, x

    stack.push((0, 0));
    visited[0][0] = 1;
    aphabets.push(board[0][0]);
    while let Some((py, px)) = stack.pop() {
        
        for (dy, dx) in directions {
            let y = dy + py as i32;
            let x = dx + px as i32;

            if y >= 0 && x >= 0 && y < r as i32 && x < c as i32 {
                let y = y as usize;
                let x = x as usize;
                
                if aphabets.contains(&board[y][x]) { continue; }
                
                visited[y][x] = visited[py][px] + 1;
                // aphabets.push(board[y][x]);
                stack.push((y, x));
            }
        }
    }

    let mut result = 0usize;
    for i in 0..r {
        for j in 0..c {
            result = result.max(visited[i][j]);
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let (r, c) = get_nums()?;
    
    let mut board: Vec<Vec<char>> = get_board()?;

    writeln!(output, "{}", dfs(board, r, c)?)?;
    Ok(())
}
