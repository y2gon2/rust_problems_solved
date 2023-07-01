//! https://www.acmicpc.net/problem/1926
//! 그림
//! 
//! 성능 개선
//! 1. 여러줄 입력을 이중 Vec type value 를 생성하는 방법을 code 를 참조 반영 함.
//!    (source code : https://www.acmicpc.net/source/61678941)
//! 2. 다른 사람들 보다 메모리 사용량이 높음을 확인하고, 불필요한 변수 설정 및 작업 제거
//!    - 경로 탐색시 기존 방문한 위치를 표시할 수 있는 vec 을 별도로 설정하지만,
//!      해당 문제의 경우, 기본으로 주어진 data 에 방문 여부를 제외하고 추가 정보를 필요로하지 않음으로
//!      입력 vec 에 바로 방문 여부를 기록하므로써, 중복 작업 및 불필요한 메모리 사용을 제거함.
//! 

#[allow(unused_imports)]
use std::io::{BufRead, stdin, stdout, Write};
use std::error::Error;


fn dfs(pixels: &mut Vec<Vec<bool>>, n: usize, m: usize) -> Result<(usize, usize), Box<dyn Error>> {
    // let (n, m) = (info[0], info[1]);
    // let mut visited = vec![vec![false; m]; n];
    let mut pics = 0usize;
    let mut largest = 0usize;

    let dirction = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for i in 0..n {
        for j in 0..m {
            if pixels[i][j] == false { continue; }

            let mut stack = Vec::<(usize, usize)>::new(); // y, x, size
            stack.push((i, j));
            pics += 1;

            let mut length = 1usize;
            
            while let Some((py, px)) = stack.pop() {
                pixels[py][px] = false;

                for (dy, dx) in dirction.iter() {
                    let y = py as i32 + *dy;
                    let x = px as i32 + *dx;

                    if y >= 0 && x >= 0 && y < n as i32 && x < m as i32 {
                        let y = y as usize;
                        let x = x as usize;

                        if pixels[y][x] == false { continue; }
                       

                        stack.push((y, x));
                        pixels[y][x] = false;
                        length += 1;
                    }
                }
            }

            largest = largest.max(length);
        }
    }
    
    Ok((pics, largest))
}

fn main() -> Result<(), Box<dyn Error>> {
    // let info = line_input()?;
    // let mut pixels = Vec::<Vec<usize>>::new();

    // for _ in 0..info[0] {
    //     pixels.push(line_input()?);
    // }

    let buf = std::io::read_to_string(stdin().lock())?;
    let mut input = buf.split_ascii_whitespace().flat_map(|s| s.parse::<usize>());
    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut pixels: Vec<Vec<_>> = (0..n)
        .map(|_| input.by_ref().take(m).map(
            |num| num == 1).collect()
        )
        .collect();
    
    let (pics, largest) = dfs(&mut pixels, n, m)?;
    let mut output = stdout().lock();

    writeln!(output, "{}\n{}", pics, largest)?;
    Ok(())
}

// fn line_input() -> Result<Vec<usize>, Box<dyn Error>> {
//     let mut buf = String::new();
//     stdin().lock().read_line(&mut buf);

//     let input: Vec<usize> = buf 
//         .split_ascii_whitespace()
//         .map(|s| s.parse::<usize>())
//         .collect::<Result<_, _>>()
//         .map_err(|e| e)?;

//     Ok(input)
// }
