// https://www.acmicpc.net/problem/11725
// 트리의 부모 찾기

use std::io::{stdin, stdout, Write, BufRead, BufWriter};
use std::error::Error;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());

    let mut lines = stdin().lock().lines().map(|line| line);
    let mut line = || -> Result<Vec<usize>, Box<dyn Error>> {
        match lines.next() {
            Some(Ok(x)) => {
                let v: Result<Vec<usize>, _>  = x
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>())
                    .collect();
                v.map_err(|e| e.into() )
            },
            Some(Err(e)) => Err(e.into()),
            None => Err("No more input".into()),
        }
    };

    let v = line()?;
    let n = v[0];
    let mut map: Vec<Vec<usize>> = vec![Vec::<usize>::new(); n + 1];

    for _ in 0..n - 1 {
        let v = line()?;
        map[v[0]].push(v[1]);
        map[v[1]].push(v[0]);
    }

    // println!("{:?}", map);

    let mut queue = VecDeque::<usize>::new();
    queue.push_back(1);
    
    let mut parent = vec![0; n + 1];
    while !queue.is_empty() {
        // println!("{:?}", queue);
        let mut num: usize = 0;
        if let Some(n) = queue.pop_front() { num = n; };

        let children: Vec<usize> = map[num].to_vec();
 
        for child in children.iter() {
            if parent[*child] == 0 {
                parent[*child] = num;
                queue.push_back(*child);
            }        
        }    
    }

    let mut ans = String::new();
    for i in 2..n + 1 {
        ans = ans + &parent[i].to_string() + "\n";
    }

    writeln!(output, "{}", ans)?;
    Ok(())
}