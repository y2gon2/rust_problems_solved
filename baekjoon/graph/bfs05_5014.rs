//! https://www.acmicpc.net/problem/5014
//! 스타트링크 
//!
//! 문제 해결
//! 1. 방문 가능한 floor 의 높이를 주어진 최대 높이 * 2 - 1 로 잘못 설정한 것을 수정.
//! 
//! 효울성 개선
//! 1. bFS 탐색이므로 목적 층에 도달하는 순간 최단 횟수만에 온 것이므로 도달했을 때 탐색을 중지함.
//!    24ms -> 20ms
//! 2. 방문했을 표시하는 vec 을 기존에 최소 도달횟수를 저장 (usize) 했었으나, 
//!    탐색 횟수는 queue 에 저장하고, vec 에는 방문 여부(bool) 만 표시 20ms -> 8ms
//! 3. 기존 입력 값을 가독성을 위해 새로운 변수에 배정했던 것을, 참조자로 가져왔서 약간의 메모리 사용
//!    감소를 예상했으나, 그 양이 많지 않아서 인지 메모리 사용에 변화는 없었음.

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;
use std::collections::VecDeque;

fn bfs(data: Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let (f, s, g, u, d) = (&data[0], &data[1], &data[2], &data[3], &data[4]);
    let mut floors = vec![false; f + 1];
    floors[*s] = true;

    let mut queue = VecDeque::<(usize, usize)>::new(); 
    queue.push_back((*s, 0));
    
    while let Some((cur, times)) = queue.pop_front() {
        if cur == *g { return Ok(times); }

        let times = times + 1;
        let up_f = cur + u;
        let down_f = cur as i32 - *d as i32;

        if up_f <= *f {
            if !floors[up_f] {
                floors[up_f] = true;
                queue.push_back((up_f, times));
            } 
        } 

        if down_f > 0 {
            if !floors[down_f as usize] {
                floors[down_f as usize] = true;
                queue.push_back((down_f as usize, times));
            }
        }
    }
    Ok(usize::MAX)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let input = read_to_string(stdin().lock())?;
    let data: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    let ans = bfs(data)?;

    if ans == usize::MAX {
        writeln!(output, "use the stairs")?;
    } else {
        writeln!(output, "{}", ans)?;
    }
    Ok(())
}