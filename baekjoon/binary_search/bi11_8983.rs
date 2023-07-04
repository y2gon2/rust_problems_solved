//! https://www.acmicpc.net/problem/8983
//! 사냥꾼
//! 
//! 풀이과정
//! 이분탐색 카테고리 내 문제지만 two pointer 와 유사한 방법으로 풀면 더 적은 시간 복잡도로 
//! 풀수 있을것 같아 시도 하였으나, 사대 위치가 사냥감 보다 x 축 기준 왼쪽(작은값을 지닌)에 있는 경우, 
//! 범위안에 들어왔을때와 벗어났을 때의 처리가 알관되지 않아 문제점이 발생한다.
//! 
//! 이를 사냥감 기준으로 모두 탐색하면서 이분 탐색으로 범위 안에 들어오는 사대가 있는지를 찾는 방법으로 해결


use std::io::{stdin, stdout, Write, BufRead, read_to_string};
use std::error::Error;

fn input_line() -> Result<Vec<usize>, Box<dyn Error>> {
    let mut buf = String::new();
    stdin().lock().read_line(&mut buf)?;
    
    let line: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    Ok(line)
}

fn input_lines() -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let input = read_to_string(stdin().lock())?;

    let points: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let v: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|i| i.parse::<usize>())
                .collect::<Result<_, _>>()
                .map_err(|e| e)?;
            Ok(v)
        })
        .collect::<Result<_, Box<dyn Error>>>()
        .map_err(|e| e)?;

    Ok(points)
}

fn hunting(shootings:Vec<usize>, prey: Vec<Vec<usize>>, info: Vec<usize>) -> usize {
    let mut cnt = 0usize;
    let (s_len, p_len, range) = (info[0], info[1], info[2]);
 
    for i in 0..p_len {
        let mut left = 0usize;
        let mut right = s_len - 1;

        while left <= right {
            let mid = (left + right) / 2;

            let shootable = (prey[i][0] as i32 - shootings[mid] as i32 ).abs() as usize + prey[i][1];

            // println!("prey:{:?}  shootings:{} left:{}  mid:{}  right:{}  shootable:{}  cnt:{}", prey[i], shootings[mid], left, mid, right, shootable, cnt);
            if shootable <= range {
                cnt += 1;
                break;
            } else {
                if prey[i][0] == shootings[mid] {
                    break;
                } else if prey[i][0] < shootings[mid]  {
                    if mid >= 1 { 
                        right = mid - 1; 
                    } else {
                        break;
                    }
                } else {
                    left  = mid + 1;
                }
            }
        }
    }
    cnt
}

fn main() -> Result<(), Box<dyn Error>> {
    let info = input_line()?;
    let mut shootings = input_line()?;
    shootings.sort();

    let prey = input_lines()?;

    // println!("{:?}", prey);

    let mut output = stdout().lock();
    let result = hunting(shootings, prey, info);

    writeln!(output, "{}", result)?;
    Ok(())
}