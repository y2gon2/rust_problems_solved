// https://www.acmicpc.net/problem/1389
// 케빈 베이컨의 6단계 법칙

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut result: usize = 0;

    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf)?;

    let mut input = buf.split_ascii_whitespace();

    let people = input.next().unwrap().parse::<usize>()?;
    let edges = input.next().unwrap().parse::<usize>()?;

    let mut nodes: Vec<HashSet<usize>> = vec![HashSet::new(); people + 1];

    for _ in 0..edges {
        let a = input.next().unwrap().parse::<usize>()?;
        let b = input.next().unwrap().parse::<usize>()?;

        nodes[a].insert(b);
        nodes[b].insert(a);
    }

    let mut sum = vec![0 as usize ; people + 1];

    for i in 1..=people {
        let mut counters = vec![std::usize::MAX; people + 1];
        let mut visited = HashSet::<usize>:: new();
        let mut queue = VecDeque::<usize>::new();

        counters[0] = 0;
        counters[i] = 0;

        queue.push_back(i);
        while let Some(person) = queue.pop_front() {
            if !visited.contains(&person) {
                visited.insert(person);

                for n in nodes[person].iter() {
                    queue.push_back(*n);
                    counters[*n] = counters[*n].min(counters[person] + 1);
                }
            }
        }    

        for i in 1..=people {
            sum[i] += counters[i];
        }
    }

    let mut mn = usize::MAX;
    for i in 1..=people {
        if mn > sum[i] {
            mn = sum[i];
            result = i;
        }
    }
    
    writeln!(output, "{}", result)?;

    Ok(())
}
