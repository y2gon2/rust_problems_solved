//! https://www.acmicpc.net/problem/2151
//! 거울 설치

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd};


const TURNS: [DIRECTION; 4] = [
    DIRECTION::North, 
    DIRECTION::East, 
    DIRECTION::South, 
    DIRECTION::West
]; 

#[derive(Clone, Debug, Eq, PartialEq)]
enum DIRECTION {
    North, 
    East,
    South,
    West,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueItem(i32, i32, u8, DIRECTION); // y, x, cnt, current direction

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

fn get_info() -> Result<(i32, Vec<Vec<u8>>, Vec<(usize, usize)>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    
    let n = input.next().unwrap().parse::<usize>()?;
    let mut room = vec![vec![0u8; n]; n];    
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
                },
                '*' => {
                    room[i][j] = 3; 
                },
                _ => panic!("Inavaliable input"),
            }
        }
    }
    room[door[0].0][door[0].1] = 0; 

    Ok((n as i32, room, door))
}

fn next_position(cy: i32, cx: i32, d: &DIRECTION) -> (i32, i32) {
    match &d {
        DIRECTION::North => return (cy - 1, cx),
        DIRECTION::East => return (cy, cx + 1),
        DIRECTION::South => return (cy + 1, cx),
        DIRECTION::West => return (cy, cx - 1),
    }
}

fn turn_right(d: &DIRECTION) -> DIRECTION {
    match d {
        DIRECTION::East => return DIRECTION::South,
        DIRECTION::South => return DIRECTION::West,
        DIRECTION::West => return DIRECTION::North,
        DIRECTION::North => return DIRECTION::East,
    }
}

fn turn_left(d: &DIRECTION) -> DIRECTION {
    match d {
        DIRECTION::East => return DIRECTION::North,
        DIRECTION::South => return DIRECTION::East,
        DIRECTION::West => return DIRECTION::South,
        DIRECTION::North => return DIRECTION::West,
    }
}

fn refraction() -> Result<u8, Box<dyn Error>> {
    let (n, room, door) = get_info()?;
    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    let (sy, sx) = (door[0].0, door[0].1);

    for sd in TURNS {
        priority_queue.push(QueueItem(sy as i32, sx as i32, 0, sd));
    }

    while let Some(QueueItem(cy, cx, cnt, d)) = priority_queue.pop() {
        match room[cy as usize][cx as usize] {
            0 => {
                let (y, x) = next_position(cy, cx, &d);
                
                if y >= 0 && x >= 0 && y < n && x < n {
                    priority_queue.push(QueueItem(y, x, cnt, d.clone()));
                }
            },
            1 => {
                // go striaght
                let (fy, fx) = next_position(cy, cx, &d);
                
                if fy >= 0 && fx >= 0 && fy < n && fx < n {
                    priority_queue.push(QueueItem(fy, fx, cnt, d.clone()));
                }

                let nd = turn_left(&d);                    
                let (ly, lx) = next_position(cy, cx, &nd);
                
                if ly >= 0 && lx >= 0 && ly < n && lx < n {   
                    priority_queue.push(QueueItem(ly, lx, cnt + 1, nd));
                }
 
                let nd = turn_right(&d);                    
                let (ry, rx) = next_position(cy, cx, &nd);
                
                if ry >= 0 && rx >= 0 && ry < n && rx < n {
                    priority_queue.push(QueueItem(ry, rx, cnt + 1, nd));
                }
            },
            2 => return Ok(cnt),
            3 => continue,
            _ => panic!("Inavaliable number"),
        } 
    }
    Ok(0)
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();

    writeln!(output, "{}", refraction()?)?;
    Ok(())
}