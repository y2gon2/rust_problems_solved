// https://www.acmicpc.net/problem/14226
// 이모티콘

use std::io::{stdin, stdout, read_to_string, Write};
use std::error::Error;
use std::collections::VecDeque;

const MX: usize = 1001;

fn bfs(n: usize) -> usize {
    let mut visited = [[false; MX]; MX]; // (position,  clipboard)
    let mut queue = VecDeque::<(usize, usize, usize)>::new();
    let mut result = 0usize;

    visited[1][0] = true;
    queue.push_back((1, 0, 0)); // position, clipboard, counter
    while let Some((pos, clip, cnt)) = queue.pop_front() {
        if pos == n { 
            result = cnt; 
            break;
        }

        if !visited[pos][pos] {
            queue.push_back((pos, pos, cnt + 1)); // 현재 화면에 있는 이모티콘 복사
            visited[pos][pos] = true;
        }

        if clip > 0 && pos + clip < n + 1 {
            if !visited[pos + clip][clip] {
                queue.push_back((pos + clip, clip, cnt + 1)); // clipboard 붙여넣기
                visited[pos + clip][clip] = true;
            }
        }

        if pos > 1 {
            if !visited[pos - 1][clip] {
                queue.push_back((pos - 1, clip, cnt + 1)); // 화면의 이모티콘 1개 삭제
                visited[pos - 1][clip] = true;
            }
        }            
        // println!("{:?}", queue);
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = read_to_string(stdin().lock())?;
    let n = buf.trim().parse::<usize>()?;

    let mut output = stdout().lock();
    writeln!(output, "{}", bfs(n))?;
    Ok(())
}