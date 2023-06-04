// https://www.acmicpc.net/problem/18258

use std::io::*;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct MyQueue<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> MyQueue<T> {
    fn new() -> Self {
        MyQueue { head: None }
    }
    
    fn push(&mut self, data: T) {

    }
}


fn my_pop(input: &mut Vec<String>) -> i32{
    if input.len() == 0 {
        return -1;        
    } else {
        
    }
    
}



fn main() -> Result<()> {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).unwrap();
    let num_of_orders = buffer.trim().parse::<usize>().unwrap();
    
    let mut orders = Vec::new();
    for _ in 0..num_of_orders {
        buffer.clear();

        stdin().read_line(&mut buffer).unwrap();
        let order: Vec<String> = buffer
            .split_ascii_whitespace()
            .map(|s| s.parse::<String>().unwrap())
            .collect();

        orders.push(order);
    }
    Ok(())
}