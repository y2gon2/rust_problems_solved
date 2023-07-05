//! https://www.acmicpc.net/problem/25682
//! 체스판 다시 칠하기 2
//! 
//! 풀이과정
//! 정상적인 체스판이 검정색 시작인 경우와 흰색 시작인 경우, 이 
//! 두가지 모두 고려해야 한다는 아이디어를 참고한 후 문제를 풀었다. 
//! 두가지 경우를 모두 고려하는 것을 살짝 생각했으나 이는 너무 시간 소모가 많을 것이라고 생각했는데
//! 잘 정리해서 문제를 풀 경우, 결국 동일 시간 복잡도 내에서 과정이 추가되었을 뿐이라는 것을 
//! 확인 할 수 있었다. 

use std::io::{stdin, stdout, Read, Write, read_to_string};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut result = usize::MAX;

    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf);

    let mut input = buf.lines();
    
    let mut info = input.next().unwrap().split_ascii_whitespace();
    let (n, m, k) = (
        info.next().unwrap().parse::<usize>()?,
        info.next().unwrap().parse::<usize>()?,
        info.next().unwrap().parse::<usize>()?,
    );

    let mut black_acc = vec![vec![0usize; m + 1]; n + 1]; // 검정색 시작 체스판
    let mut white_acc = vec![vec![0usize; m + 1]; n + 1]; // 흰색 시작 체스판
    for y in 1..=n {
        let row = input.next().unwrap();
        for (j, c) in row.char_indices() {
            let x = j + 1;

            // 이전 오차의 누적값을 받아옴
            black_acc[y][x] = black_acc[y - 1][x] + black_acc[y][x - 1] - black_acc[y - 1][x - 1];
            white_acc[y][x] = white_acc[y - 1][x] + white_acc[y][x - 1] - white_acc[y - 1][x - 1];

            // 현재 위치에서 오차 발생시 추가
            if (y + x) % 2 == 0 {
                if c == 'B' {
                    white_acc[y][x] += 1;  
                } else {
                    black_acc[y][x] += 1;
                }               
            } else {
                if c == 'B' {
                    black_acc[y][x] += 1;
                } else {
                    white_acc[y][x] += 1;
                }
            }
        }
    } 

    // for i in 0..n {
    //     println!("black:{:?}\twhite:{:?}", black_acc[i], white_acc[i]);
    // }

    for i in k..=n {
        for j in k..=m {
            let black_diff = black_acc[i][j] + black_acc[i - k][j - k] - black_acc[i - k][j] - black_acc[i][j - k];
            let white_diff = white_acc[i][j] + white_acc[i - k][j - k] - white_acc[i - k][j] - white_acc[i][j - k];

            if black_diff < white_diff {
                result = result.min(black_diff);
            } else {
                result = result.min(white_diff);
            }
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}
