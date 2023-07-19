//! https://www.acmicpc.net/problem/14500
//! 테트로미노
//! 
//! 
use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::cmp::max;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// ㅏ, ㅓ, ㅗ, ㅜ
const SHAPES : [[(i32, i32); 3]; 4] = [ 
    [(1, 0), (1, 1), (2, 0)],
    [(1, 0), (1, -1), (2, 0)],
    [(0, 1), (-1, 1), (0, 2)],
    [(0, 1), (1, 1), (0, 2)],
];

fn get_info() -> Result<(usize, usize, Vec<Vec<usize>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let (n, m) = (get_num()?, get_num()?);
    let mut matrix = vec![vec![0usize; m]; n];

    for i in 0..n {
        for j in 0..m {
            matrix[i][j] = get_num()?;
        }
    }
    Ok((n, m, matrix))
}

fn dfs(
    i: i32, 
    j: i32, 
    n:usize, 
    m:usize, 
    matrix: &Vec<Vec<usize>>, 
    visited: &mut Vec<Vec<bool>>,
    result: &mut usize,
    cnt: &mut u8,
    sum: &mut usize,
) {
    if *cnt == 4 { 
        *result = max(*result, *sum);
        // println!("({}, {})  dfs sum: {}", i, j, *sum);
        return;
    }
    
    for (dy, dx) in DIRECTIONS {
        let y = i + dy;
        let x = j + dx;

        if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
            let ny = y as usize;
            let nx = x as usize;

            if visited[ny][nx] { continue; }

            visited[ny][nx] = true;
            dfs(y, x, n, m, matrix, visited, result, &mut (*cnt + 1 as u8), &mut (*sum + matrix[ny][nx]));
            visited[ny][nx] = false;
        } 
    }
}

fn vowel(i: usize, j: usize, n: usize, m: usize, matrix: &Vec<Vec<usize>>, result: &mut usize) {
    for s in SHAPES {
        let mut temp = matrix[i][j];
        for (vy, vx) in s {
            let y = i as i32 + vy;
            let x = j as i32 + vx;

            if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                temp += matrix[y as usize][x as usize];
            } else {
                break;
            }
        }
        // println!("vowel sum: {}", temp);
        *result = max(*result, temp);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut result = 0usize;

    let (n, m, matrix) = get_info()?;
    let mut visited = vec![vec![false; m]; n];

    for i in 0..n {
        for j in 0..m {
            visited[i][j] = true;
            if i > 0 { visited[i - 1][j] == true; }
            if j > 0 { visited[i][j - 1] == true; }
            dfs(i as i32, j as i32, n, m, &matrix, &mut visited, &mut result, &mut 1, &mut matrix[i][j].clone());
            visited[i][j] = false;
            if i > 0 { visited[i - 1][j] == false; }
            if j > 0 { visited[i][j - 1] == false; }
            vowel(i, j, n, m, &matrix, &mut result);
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}