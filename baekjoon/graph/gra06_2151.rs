//! https://www.acmicpc.net/problem/2151
//! 거울 설치


use std::io::{stdin, stdout, Read};
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, Clone)]
struct QueueItem(u8, u8, u8); // y, x, mirrors

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

fn get_info() -> Result<(usize, Vec<Vec<u8>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    
    let n = input.next().unwrap().parse::<usize>()?;
    let mut room = vec![vec![0u8; n]; n];   

    for i in 0..n {
        let line = input.next().unwrap();
        for (j, c) in line.char_indices() {
            // void: 0, mirror_point: 1, door: 2, wall: 3
            match c {
                '.' => room[i][j] = 0,
                '!' => room[i][j] = 1,
                '#' => room[i][j] = 2,
                '*' => room[i][j] = 3,
                _ => Err("Inavaliable input".into()),
            }
        }
    }
    
    Ok((n, room))
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    writeln!(output, "{}", )?;
    Ok(())
}