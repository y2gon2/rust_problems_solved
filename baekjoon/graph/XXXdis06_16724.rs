//! https://www.acmicpc.net/problem/10216
//! 피리 부는 사나이

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn dfs(map: &Vec<Vec<char>>, n: usize, m: usize) -> usize{
    let mut parent = vec![vec![(usize::MAX); m]; n];

    let mut result = 0usize;

    for i in 0..n {
        for j in 0..m {
            if parent[i][j] == usize::MAX {
                parent[i][j] = result;
                disjoint(&map, &mut parent, i, j, result);
                result += 1;
            }
        }
    }
    result
}

fn disjoint(
    map: &Vec<Vec<char>>, 
    parent: &mut Vec<Vec<usize>>, 
    mut sy: usize, 
    mut sx:usize,
    result: usize,
) {
    loop {
        let mut y = sy;
        let mut x = sx;
    
        match map[sy][sx] {
            'U' => y += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            'R' => x += 1,
            _ => panic!("Invalid Input"),
        }

        if parent[y][x] == usize::MAX {  
            parent[y][x] = result;
    
        } else if 
    
        parent[y][x] = result;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);

    let mut input = buf.split_ascii_whitespace();
    
    let (n, m) = (
        input.next().unwrap().parse::<usize>()?,
        input.next().unwrap().parse::<usize>()?
    );

    let mut map: Vec<Vec<char>> = vec![vec!['a'; m]; n];
    for (idx,line) in input.enumerate() {
        map[idx] = line.chars().collect();
    }

    writeln!(stdout(), "{}", dfs(&map, n, m))?;
    Ok(())
}