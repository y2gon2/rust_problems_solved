// https://www.acmicpc.net/problem/2644
// 촌수계산

use std::io::{stdin, stdout, Write, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf)?;
    
    let mut input = buf.split_ascii_whitespace();
    let mut read = || -> Result<&str, Box<dyn Error>> {
        match input.next() {
            Some(x) => Ok(x),
            None => Err("No more input".into()),
        }
    };

    let people = read()?.parse::<usize>()?;
    let start = read()?.parse::<usize>()?;
    let finish = read()?.parse::<usize>()?;
    let _link = read()?.parse::<usize>()?;

    let mut nodes = vec![vec![false; people + 1]; people + 1];
    while let Some(a) = input.next() {
        let a = a.parse::<usize>()?;
        let b = input.next().unwrap().parse::<usize>()?;

        nodes[a][b] = true;
        nodes[b][a] = true;
    }

    let mut stack = Vec::from([start]);
    let mut visited = vec![false; people + 1];
    let mut counters = vec![-1; people + 1];

    counters[start] = 0;
    let mut is_done = false;
    while let Some(cur) = stack.pop() {
        if !visited[cur] {
            visited[cur] = true;

            for i in 1..=people {
                if nodes[cur][i] {
                    stack.push(i);
                    counters[i] = counters[cur] + 1;

                    if i == finish { 
                        is_done = true;
                        break; 
                    }
                }
            }
        } 
        if is_done { break; }
    }

    writeln!(output, "{}", counters[finish])?;
    Ok(())
}
