// https://www.acmicpc.net/problem/14226
// 이모티콘

use std::io::{stdin, stdout, read_to_string, Write};
use std::error::Error;
use std::collections::VecDeque;

fn bfs(n: usize) -> Result<usize, Box<dyn Error>> {
    let mut imojis: Vec<usize> = vec![0; n * 2 - 1];
    let mut visited = vec![false; n * 2 - 1];
    let mut queue = VecDeque::<(usize, usize)>::new();

    visited[1] = true;
    imojis[1] = 1;
    imojis[2] = 2;

    queue.push_back((2, 2));
    while imojis[n] > 0 {
        let (position, cnt) = queue.pop_front().unwrap();
        visited[position] = true;

        let back = position - 1;
        let forward = position + 1;
        let jump = position * 2;
        if position - 1 > 1 && position * 2 < n * 2 - 1 {

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