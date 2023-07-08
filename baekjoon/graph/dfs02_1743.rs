//! https://www.acmicpc.net/problem/1743
//! 음식물 피하기
//! 


use std::io::{stdin, stdout, Read, Write};
// use std::error::Error;

fn main() {
    let mut result = 0;
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf).unwrap();
    let mut buf_iter = buf.split_ascii_whitespace();
    let mut get_num = || buf_iter
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    
    let (n, m, k) = (get_num(), get_num(), get_num());

    let mut aisle = [[false; 100]; 100];

    for _ in 0..k {
        let (y, x) = (get_num(), get_num());

        aisle[y - 1][x - 1] = true;
    }

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut stack = Vec::<(usize, usize)>::new();

    for i in 0..n {
        for j in 0..m {
            if !aisle[i][j] { continue; }

            stack.push((i, j));
            aisle[i][j] = false;

            let mut cnt = 0;
            while !stack.is_empty() {
                let (py, px) = stack.pop().unwrap();
                cnt += 1;

                for (dy, dx) in directions.iter() {
                    let y = py as i32 + dy;
                    let x = px as i32 + dx;

                    if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                        let y = y as usize;
                        let x = x as usize;

                        if !aisle[y][x] { continue; }

                        stack.push((y, x));
                        aisle[y][x] = false;

                    }
                }
            }
            result = result.max(cnt);
        }
    }
    writeln!(output, "{}", result);
    
}