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
    let mut visited = vec![vec![false; c]; r];
    let dirctions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut aphabets = Vec::<char>::new();
    let mut stack = Vec::<(usize, usize, char)>::new(); // y, x, alphabet

    stack.push((0, 0, board[0][0]));
    while let Some((py, px, ch)) = stack.pop() {
        visited[py][px] = true;
        aphabets.push(ch);
    }

    Ok(aphabets.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let (r, c) = get_nums()?;
    
    let mut board: Vec<Vec<char>> = get_board()?;

    writeln!(output, "{}", dfs(board, r, c)?)?;
    Ok(())
}
