// https://www.acmicpc.net/problem/16234
// 인구이동

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::VecDeque;

fn union(lands:&mut Vec<Vec<usize>>, info: Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let (n, mn, mx) = (info[0], info[1], info[2]);
    let result = 0usize;

    let directions = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    
    loop {
        let mut visited = vec![vec![false; n]; n];
        let mut is_moved = 0usize;

        for i in 0..n {
            for j in 0..n {

                let mut queue = VecDeque::<(usize, usize)>::new();  // y, x
                let mut sub_sum = 0usize;
                let mut sub_union = Vec::<(usize, usize)>::new();

                queue.push_back((i, j));
                visited[i][j] = true;
                sub_sum += 1;
                sub_union.push((i, j));

                while let Some((py, px)) = queue.pop_front() {
                    for (dy, dx) in directions.iter() {
                        let y = py as i32 + dy;
                        let x = px as i32 + dx;

                        if y >= 0 && x >= 0 && y < n && x < n {
                            let y = y as usize;
                            let x = x as usize;

                            if visited[y][x] { continue; } 

                            let diff  = (lands[py][px] as i32 - lands[y][x] as i32).abs() as usize;
                            if diff >= mn && diff <= mx {
                                queue.push_back((y, x));
                                visited[y][x] = true;

                                sub_sum += lands[y][x];
                                sub_union.push((i, j));
                            }
                        }
                    }                    
                }

                let average = sub_sum / sub_union.len();
                for (x, y) in sub_union {
                    lands[i][j] = average;
                }

                is_moved += 1;
            }
        }

        if is_moved == 0 { break; }
        result += 1;
    }
    

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = String::new();
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut line = buf.lines();
    
    let info = line
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| e)?;

    let mut lands = Vec::<Vec<usize>>::new();
    for _ in 0..info[0] {
        lands.push(
            line.next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        );
    }
    
    let result = union(&mut lands, info)?;
    writeln!(output, "{}", result)?;
    Ok(())
}