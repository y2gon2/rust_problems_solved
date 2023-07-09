//! https://www.acmicpc.net/problem/14502
//! 연구소
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn blocking(rooms: [[usize; 8]; 8], zeros: Vec<(usize, usize)>, n: usize, m: usize) -> usize {
    let mut result = 0usize;
    let z_len = zeros.len();
    for z1 in 0..z_len {
        for z2 in z1 + 1..z_len {
            for z3 in z2 + 1..z_len {
                let mut c_rooms = rooms.clone();

                c_rooms[zeros[z1].0][zeros[z1].1] = 1;
                c_rooms[zeros[z2].0][zeros[z2].1] = 1;
                c_rooms[zeros[z3].0][zeros[z3].1] = 1; 

                let cnt = safe_rooms(&mut c_rooms, n, m);

                result = result.max(cnt);
            }
        }
    }
    result
}

fn safe_rooms(c_rooms: &mut [[usize; 8]; 8], n: usize, m: usize) -> usize {
    let mut cnt_safe = 0usize;
    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for i in 0..n {
        for j in 0..m {
            if c_rooms[i][j] != 2 { continue; }

            let mut virus_stack = Vec::<(usize, usize)>::new(); 
            virus_stack.push((i, j));
            while let Some((py, px)) = virus_stack.pop() {
                for (dy, dx) in directions.iter() {
                    let y = py as i32 + dy;
                    let x = px as i32 + dx;

                    if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                        let y = y as usize;
                        let x = x as usize;

                        if c_rooms[y][x] == 0 {
                            c_rooms[y][x] = 2;
                            virus_stack.push((y, x));
                        }
                    } 
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if c_rooms[i][j] == 0 { 
                cnt_safe += 1; 
            }
        }
    }

    cnt_safe
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut buf_split = buf.split_ascii_whitespace();
    let mut get_num = || buf_split
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_num()?, get_num()?);  
    let mut rooms = [[0usize; 8]; 8];
    let mut zeros = Vec::<(usize, usize)>::new();
    
    for i in 0..n {
        for j in 0..m {
            let num = get_num()?;
            rooms[i][j] = num;

            if num == 0 {
                zeros.push((i, j));
            }
        }
    }

    let result = blocking(rooms.clone(), zeros, n, m);
    writeln!(output, "{}", result)?;
    Ok(())
}