// https://www.acmicpc.net/problem/2583
// 영역 구하기

use std::io::{stdin, stdout, BufWriter, Write, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    
    let mut input = stdin().lock().lines().map(|line| line);

    let mut line = || -> Result<Vec<usize>, Box<dyn Error>> {
        match input.next() {
            Some(Ok(s)) => {
                let nums: Result<Vec<usize>, _> = s
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>())
                    .collect();
                nums.map_err(|e| e.into())
            },
            Some(Err(e)) => Err(e.into()),
            None => Err("No more input".into()),
        }
    };    

    let first = line()?;
    let (n, m, k) = (first[0], first[1], first[2]);
 
    let mut sq = vec![vec![0usize; 4]; k];
    for i in 0..k {
        sq[i] = line()?;

    }

    let mut matrix = vec![vec![true; m]; n];
    for s in sq.iter() {
        for i in s[1]..s[3] {
            for j in s[0]..s[2] {
                matrix[i][j] = false;
            }
        }
    }

    // for i in 0..n{
    //     println!("{:?}", matrix[i]);
    // }

    let mut rooms = Vec::<usize>::new();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] {
                let mut stack = vec![(i, j)];
                let mut cnt: usize = 0;

                while let Some(p) = stack.pop() {
                    matrix[p.0][p.1] = false;
                    cnt += 1;

                    for d in directions.iter() {
                        let y = p.0 as i32 + d.0;
                        let x = p.1 as i32 + d.1;
                        if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                            if matrix[y as usize][x as usize] && !stack.contains(&(y as usize, x as usize)) {
                                stack.push((y as usize, x as usize));        
                            }
                        } 
                    }
                    // println!("{:?}", stack);
                }
                rooms.push(cnt);
            }
        }
    }
    
    rooms.sort();
    writeln!(output, "{}", rooms.len())?;
    for r in rooms.iter() {
        write!(output, "{} ", r)?;
    }

    Ok(())
}