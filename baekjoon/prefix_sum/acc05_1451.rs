//! https://www.acmicpc.net/problem/1451
//! 직사각형으로 나누기
//! 
//! 풀이과정
//! 다양한 경우의 수를 모두 포함할 수 있는 최대한 통합되고 일관성 있는 규칙을 찾는데 시간이 걸림.
//! 분할의 선이 기준이 아닌 4분할(3분할로 통합 가능) 되는 점이 하나만 존재할 때와
//! 그렇지 못한 경우(평행선으로 나누어지는 경우)만 나누어 생각하는 것이 그나마 찾은 방법 중 가장 일반적 규칙에 해당하여 이를 적용


use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0usize;
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut lines = buf.lines();
    
    let mut info = lines
        .next()
        .unwrap()
        .split_ascii_whitespace();

    let (n, m) = (
        info.next().unwrap().parse::<usize>()?,
        info.next().unwrap().parse::<usize>()?,
    );

    let mut sq = vec![vec![0usize; m + 1]; n + 1];

    for i in 1..=n {
        let row = lines.next().unwrap();

        for (idx, ch) in row.char_indices() {
            let j = idx + 1;
            sq[i][j] = ch.to_digit(10).unwrap() as usize + sq[i - 1][j] + sq[i][j - 1] - sq[i - 1][j - 1];
        }
    }

    // println!("{:?}", sq);
    
    for i in 1..=n {
        for j in 1..=m {
            let first = sq[i][j];

            if i == n && j == m {
                break;
            } else if i == n {
                for se_j in (j + 1)..=m {
                    let second = sq[i][se_j] - first;
                    let third = sq[n][m] - sq[i][se_j];

                    result = result.max(first * second * third);
                }
            } else if j == m {
                for se_i in (i + 1)..=n {
                    let second = sq[se_i][j] - first;
                    let third = sq[n][m] - sq[se_i][j];

                    result = result.max(first * second * third);
                }
            } else {
                let second = sq[i][m] - first;
                let third = sq[n][j] - first;
                let forth = sq[n][m] - first - second - third;

                // println!("frist:{} \t second:{}\t thrid:{}\t forth:{}", first, second, third, forth);

                result = result.max((first + second) * third * forth);
                result = result.max((first + third) * second * forth);
                result = result.max(first * second * (third + forth));
                result = result.max(first * third * (second + forth));
            }
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}