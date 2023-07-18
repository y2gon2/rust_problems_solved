//! https://www.acmicpc.net/problem/3190
//! 뱀

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::VecDeque;

enum Directions {
    East,
    South,
    West,
    North,
}

fn get_info() -> Result<(usize, Vec<Vec<u8>>, [char; 10001]), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_element = || input.next().unwrap();

    let n = get_element().parse::<usize>()?;
    let mut map = vec![vec![0u8; n]; n];

    let points = get_element().parse::<u8>()?;
    for _ in 0..points {
        let y = get_element().parse::<usize>()? - 1;
        let x = get_element().parse::<usize>()? - 1;

        map[y][x] = u8::MAX;
    }   

    let t = get_element().parse::<usize>()?;
    let mut turns = ['A'; 10_001];
    for _ in 0..t {
        let a = get_element().parse::<usize>()?;
        turns[a] = get_element().parse::<char>()?;
    } 
    Ok((n, map, turns))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cnt = 0usize;
    let mut output = stdout().lock();
    
    let (n, mut map, turns) = get_info()?;

    let mut queue = VecDeque::<(i8, i8)>::new();
    queue.push_back((0, 0));
    map[0][0] = 1;

    let mut head = (0i8, 0i8);
    let mut cur_direction = Directions::East;
    loop {
        if turns[cnt] == 'D' {
            match cur_direction {
                Directions::East => cur_direction = Directions::South,
                Directions::South => cur_direction = Directions::West,
                Directions::West => cur_direction = Directions::North,
                Directions::North => cur_direction = Directions::East,
            }
        } else if turns[cnt] == 'L' {
            match cur_direction {
                Directions::East => cur_direction = Directions::North,
                Directions::South => cur_direction = Directions::East,
                Directions::West => cur_direction = Directions::South,
                Directions::North => cur_direction = Directions::West,
            }
        }

        #[allow(unused_assignments)]
        let (mut y, mut x) = (0i8, 0i8);

        match cur_direction {
            Directions::East => {
                y = head.0;
                x = head.1 + 1;
            },
            Directions::South => {
                y = head.0 + 1;
                x = head.1;
            },
            Directions::West => {
                y = head.0;
                x = head.1 - 1;
            },
            Directions::North => {
                y = head.0 - 1;
                x = head.1;
            },
        }
        
        cnt += 1;

        if y >= 0 && x >= 0 && y < n as i8 && x < n as i8 { 
            let ny = y as usize;
            let nx = x as usize;
    
            if map[ny][nx] == 1 { break; }
            if !map[ny][nx] == u8::MAX { 
                let (ry, rx) = queue.pop_front().unwrap();
                map[ry as usize][rx as usize] = 0; 
            }   
    
            map[ny][nx] = 1;
            head = (y, x);    
            queue.push_back((y, x)); 
        } else {
            break;
        }
        // println!("----------------------------------------------");
        // println!("cnt: {}" , cnt);
        // for i in 0..n {
        //     println!("{:?}", map[i]);
        // }
    }
    writeln!(output, "{}", cnt)?;
    Ok(())
}