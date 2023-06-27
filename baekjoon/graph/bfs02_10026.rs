// https://www.acmicpc.net/problem/10026
// 적록 색약

use std::io::{stdin, read_to_string};
use std::error::Error;

fn segregation(grid: &Vec<Vec<u8>>, color_weakness: bool) -> Result<usize, Box<dyn Error>> {
    let n = grid.len();
    let mut visited = vec![vec![false; n]; n];
    let mut cnt = 0usize;
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for i in 0..n {
        for j in 0..n {
            if visited[i][j] { continue; }
            let mut stack = vec![(grid[i][j], i, j)];

            while let Some(cur) = stack.pop() {
                visited[cur.1][cur.2] = true;

                for (d_y, d_x) in directions.iter() {
                    let y = cur.1 as i32 + d_y;
                    let x = cur.2 as i32 + d_x;

                    if y >= 0 && x >= 0 && y < n as i32 && x < n as i32 {
                        if visited[y as usize][x as usize] { continue; }
                        if stack.contains(&(cur.0, y as usize, x as usize)) { continue; }
                        if color_weakness {
                            match (cur.0, grid[y as usize][x as usize]) {
                                (b'R' | b'G', b'B') | (b'B', b'R' | b'G') => continue,
                                (_, _) => (),
                            }
                        } else if cur.0 != grid[y as usize][x as usize]{ continue; }
                        stack.push((cur.0, y as usize, x as usize));
                    } 
                }
            }
            cnt += 1;
        }
    }
    Ok(cnt)
} 

fn main() -> Result<(), Box<dyn Error>> {
    // let (n, mut grid) = input()?;
    let buf = read_to_string(stdin().lock())?;
    let grid: Vec<Vec<u8>> = buf.lines().skip(1).map(|line| line.as_bytes().to_vec()).collect();

    let normal = segregation(&grid, false)?;
    let color_weakness = segregation(&grid, true)?;

    println!("{} {}", normal, color_weakness);
    Ok(())
}


// use std::io::BufRead;
// fn input() -> Result<(usize, Vec<Vec<char>>), Box<dyn Error>> {
//     let mut input = stdin().lock().lines().map(|line| line);
    
//     let n = input.next().unwrap()?.trim().parse::<usize>()?;
//     let mut line = || -> Result<Vec<char>, Box<dyn Error>> {
//         match input.next() {
//             Some(Ok(s)) => {
//                 let st = s.trim();
//                 let ch = st.chars().map(|c| c).collect::<Vec<char>>();
//                 Ok(ch)
//             },
//             Some(Err(e)) => Err(e.into()),
//             None => Err("No more input".into()),
//         }
//     };

//     let mut grid: Vec<Vec<char>> = Vec::new();
//     for _ in 0..n {
//         grid.push(line()?);
//     }
//     return Ok((n, grid));
// }
