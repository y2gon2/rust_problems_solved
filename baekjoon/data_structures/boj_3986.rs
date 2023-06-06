// https://www.acmicpc.net/problem/3986

use std::io::*;

fn main() -> Result<()> {
    let mut buffer = String::new();
    
    stdin().read_line(&mut buffer).unwrap();
    let n = buffer.trim().parse::<usize>().unwrap();

    let mut words: Vec<_> = Vec::new();
    for _ in 0..n {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let word = buffer.trim().parse::<String>().unwrap();
        words.push(word);
    }

    let mut stack: Vec<_> = Vec::new();
    let mut result: usize = 0;

    for _ in 0..n {
        if let Some(word) = words.pop() {
            if word.len() % 2 != 0 { continue; }
            
            for c in word.chars() {
                if stack.is_empty() {
                    stack.push(c);
                } else if c == stack[stack.len() - 1] {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }            

            if stack.is_empty() { result += 1; }
        }
        stack.clear();
    }

    println!("{}", result);

    Ok(())
}