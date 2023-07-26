//! https://www.acmicpc.net/problem/2151
//! 거울 설치


use std::io::{stdin, stdout, Read};
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};


const DIRECTIONS: [(i16, i16); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; 

#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueItem(usize, usize, u8); // y, x, mirrors

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.2.cmp(&self.2))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}

fn get_info() -> Result<(usize, Vec<Vec<u8>>, Vec<(usize, usize)>, Vec<Vec<bool>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    
    let n = input.next().unwrap().parse::<usize>()?;
    let mut room = vec![vec![0u8; n]; n];   
    let mut visited = vec![vec![false; n]; n];   
    let mut door = Vec::<(usize, usize)>::new();


    for i in 0..n {
        let line = input.next().unwrap();
        for (j, c) in line.char_indices() {
            // void: 0, mirror_point: 1, door: 2, wall: 3
            match c {
                '.' => room[i][j] = 0,
                '!' => room[i][j] = 1,
                '#' => { 
                    room[i][j] = 2; 
                    door.push((i, j)); 
                    visited[i][j] = true;
                },
                '*' => {
                    room[i][j] = 3; 
                    visited[i][j] = true;
                },
                _ => panic!("Inavaliable input"),
            }
        }
    }
    Ok((n, room, door, visited))
}

fn refraction() -> Result<u8, Box<dyn Error>> {
    let (n, room, door, visited) = get_info()?;
    let mut result = 0u8;

    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    priority_queue.push(QueueItem(door[0].0, door[0].1, result));    
    
    while let Some(QueueItem(cur_y, cur_x, cnt)) = priority_queue.pop() {

    }
    
    Ok(result)
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    writeln!(output, "{}", )?;
    Ok(())
}