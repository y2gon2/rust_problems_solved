// https://www.acmicpc.net/problem/14226
// 이모티콘

use std::io::{stdin, stdout, read_to_string, Write};
use std::error::Error;
use std::collections::VecDeque;

fn bfs(n: usize) -> Result<usize, Box<dyn Error>> {
    let mut imojis: Vec<usize> = vec![usize::MAX; n * 2 - 1];
    let mut visited = vec![false; n * 2 - 1];
    let mut queue = VecDeque::<(usize, usize)>::new();

    visited[1] = true;
    imojis[1] = 1;
    imojis[2] = 2;

    queue.push_back((2, 2));
    while imojis[n] < usize::MAX {
        let (pos, cnt) = queue.pop_front().unwrap();
        visited[pos] = true;

        if !visited[pos - 1] && imojis[pos - 1] > cnt + 1 {
            queue.push_back((pos - 1, cnt + 1));
        }
        if pos * 2 < n * 2 - 1 {
            queue.push_back((pos * 2, cnt + 2));
        }
    }
    Ok(imojis[n])
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = read_to_string(stdin().lock())?;
    let n = buf.trim().parse::<usize>()?;

    let mut output = stdout().lock();
    writeln!(output, "{}", bfs(n)?)?;
    Ok(())
}