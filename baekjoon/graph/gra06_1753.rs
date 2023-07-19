//! https://www.acmicpc.net/problem/1753
//! 최단경로
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn get_info() -> Result<(usize, usize, Vec<Vec<(usize, u8)>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ =  stdin().lock().read_to_string(&mut buf)?;
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap();

    let (n, e, start) = (
        get_num().parse::<usize>()?,
        get_num().parse::<usize>()?,
        get_num().parse::<usize>()?,
    );

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..e {
        let from = get_num().parse::<usize>()?;
        let to = get_num().parse::<usize>()?;
        let weight = get_num().parse::<u8>()?;

        graph[from].push((to, weight));
    }

    Ok((n, start, graph))
}

fn explore(
    from: usize, 
    dest: usize, 
    graph: &Vec<Vec<(usize, u8)>>,
    sum: usize,
    result: &mut Vec<usize>,
    visited: &mut Vec<bool>
) {
    if from == dest {
        result.push(sum);
        return;
    }

    for (next, weight) in graph[from].iter() {
        if visited[*next] { continue; }

        visited[*next] = true;
        explore(from, dest, graph, sum, result, visited);
        visited[*next] = false;
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let (n, start, graph) = get_info()?;
    let mut result = vec![usize::MAX; n + 1];
    let sum = 0usize;

    for dest in 1..=n {
        let mut visited = vec![false; n + 1];
        visited[start] = true;
        explore(start, dest, &graph, sum, &mut result, &mut visited);
    }


    let ans: String = result
        .iter()
        .skip(1)
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    writeln!(output, "{}", ans)?;
    Ok(())
}