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
        let new_node = Box::new (Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut old_node) => {
                self.head = old_node.next.take();
                Some(old_node.data)
            }
        }
    }

    fn size(&self) -> usize {
        let mut cnt: usize = 0;
        let mut cnt_node = &self.head;

        loop {
            match cnt_node {
                None => break,
                Some(next_node) => {
                    cnt_node = &next_node.next;
                    cnt += 1;
                }
            }
        }

        return cnt;
    }

    fn empty(&self) -> usize {
        match &self.head {
            None => return 1,
            Some(_) => return 0, 
        }
    }

    fn front(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(next_node) => Some(&next_node.data), 
        }
    }

    fn back(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(mut cur_node) => {
                loop {
                    match &cur_node.next {
                        None => return Some(&cur_node.data),
                        Some(and_then) => cur_node = and_then,
                    }
                }
            }
        }
    }

}


fn main() -> Result<()> {
    // let mut buffer = String::new();

    // stdin().read_line(&mut buffer).unwrap();
    // let num_of_orders = buffer.trim().parse::<usize>().unwrap();
    
    // let mut orders = Vec::new();
    // for _ in 0..num_of_orders {
    //     buffer.clear();

    //     stdin().read_line(&mut buffer).unwrap();
    //     let order: Vec<String> = buffer
    //         .split_ascii_whitespace()
    //         .map(|s| s.parse::<String>().unwrap())
    //         .collect();

    //     orders.push(order);
    // }

    let mut my_queue = MyQueue::<usize>::new();
    
    my_queue.push(10);
    my_queue.push(20);
    my_queue.push(30);
    if let Some(x) = my_queue.pop() {
        println!("pop : {}", x);
    }

    println!{"size : {}", my_queue.size()};

    if let Some(x) = my_queue.front() {
        println!("front: {}", x);
    }

    Ok(())
}