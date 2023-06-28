// https://www.acmicpc.net/problem/1260
// DFS 와 BFS

use std::io::{stdin, stdout, BufWriter, BufRead, Write};
use std::error::Error;
use std::collections::{HashMap, HashSet, VecDeque};

fn dfs(map: &HashMap<usize, Vec<usize>>, v: usize) -> String {
    let mut stack: Vec<usize> = Vec::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut result = String::new();

    stack.push(v);
    while !stack.is_empty() {
        let mut num: usize = 0;
        if let Some(n) = stack.pop() { num = n; };

        if !visited.contains(&num) {
            result.push_str(&(num.to_string() + " "));
            visited.insert(num);

            let mut nodes: Vec<usize> = Vec::new();
            if let Some(x) = map.get(&num) { nodes = x.to_vec(); }; 
    
            for node in nodes.iter().rev() {
                stack.push(*node);
            }
        } 
        // println!("{:?}", &stack);
    }    
    result
}

fn vfs(map: &HashMap<usize, Vec<usize>>, v: usize) -> String {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut result = String::new();
    let mut visited: HashSet<usize> = HashSet::new();

    queue.push_back(v);
    while !queue.is_empty() {
        let mut num: usize = 0;
        if let Some(n) = queue.pop_front() { num = n; };
        
        if !visited.contains(&num) {
            result.push_str(&(num.to_string() + " "));
            visited.insert(num);

            let mut nodes: Vec<usize> = Vec::new();
            if let Some(n) = map.get(&num) { nodes = n.to_vec(); };

            for node in nodes.iter() {
                queue.push_back(*node);
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut input = stdin().lock().lines().map(|line| line);

    let mut line = || -> Result<Vec<usize>, Box<dyn Error>> {
        match input.next().ok_or("No more input")? {
            Ok(line) => {
                let nums: Result<Vec<usize>, _> = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>())
                    .collect();
                nums.map_err(|e| e.into())
            },
            Err(e) => Err(e.into()),
        }
    };

    let first_info = line()?;
    let (_, m, v) = (first_info[0], first_info[1], first_info[2]);
    
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..m {
        let link = line()?;
        map.entry(link[0]).and_modify(|v| v.push(link[1])).or_insert(vec![link[1]]);
        map.entry(link[1]).and_modify(|v| v.push(link[0])).or_insert(vec![link[0]]);
    }

    for (_, v) in map.iter_mut() {
        v.sort();
    }

    // for (k, v) in map.iter() {
    //     println!("k:{}\t v:{:?}", k, v);
    // }
    
    let dfs = dfs(&map, v);
    let vfs = vfs(&map, v);

    writeln!(output, "{}\n{}", dfs, vfs)?;
    
    Ok(())
}


// let mut line = || -> Result<Vec<usize>, Box<dyn std::error::Error>> {
//     match input.next() {
//         Some(Ok(line)) => {
//             let numbers: Result<Vec<usize>, _> = line
//                 .split_ascii_whitespace()
//                 .map(|s| s.parse::<usize>())
//                 .collect();
//             numbers.map_err(|e| e.into())
//         }
//         Some(Err(e)) => Err(e.into()),
//         None => Err("No more input".into()),
//     }
// };r