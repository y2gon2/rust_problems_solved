//! https://www.acmicpc.net/problem/10216
//! Count Circle Groups

use std::io::stdin;
use std::fmt::Write;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
struct Point{
    x: usize,
    y: usize,
    r: usize,
}

impl Point {
    fn from_vec(vec: Vec<usize>) -> Self {
        Self {
            x: vec[0],
            y: vec[1],
            r: vec[2],
        }
    }
}

fn get_line() -> Result<Vec<usize>, ParseIntError> {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);

    let result: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    Ok(result)
} 

fn disjoint() -> Result<i32, Box<dyn Error>> {
    let n = get_line()?[0];
    let mut groups: Vec<Point> = Vec::<Point>::new();
    let mut result = n as i32;
    let mut parent: Vec<usize> = (0..=n).collect();
    
    for i in 0..n {
        let p1 = Point::from_vec(get_line()?);
        groups.push(p1.clone());
        
        for j in 0..i {
            let p2 = groups[j].clone();

            if is_neighbor(p1.clone(), p2) && (find(&mut parent, i) != find(&mut parent, j)) { 
                union(&mut parent, i, j);
                result -= 1;
            }
            // println!("parent :{:?} result: {}", parent, result);
        }
    }
    Ok(result)
}

fn union(parent: &mut Vec<usize>, n1: usize, n2: usize) {
    let r1 = find(parent, n1);
    let r2 = find(parent, n2);

    if r1 < r2 {
        parent[r2] = r1;
    } else {
        parent[r1] = r2;
    }
}

fn find(parent:&mut Vec<usize>, n: usize) -> usize {
    if parent[n] != n {
        parent[n] = find(parent, parent[n]);
    }
    return parent[n]
}

fn is_neighbor(p1: Point, p2: Point) -> bool {
    let distance = (p1.x as i64 - p2.x as i64).pow(2) + (p1.y as i64 - p2.y as i64).pow(2);

    if distance <= (p1.r + p2.r).pow(2) as i64 {
        return true
    } else {
        return false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = String::new();
    let t = get_line()?[0];

    for _ in 0..t {
        writeln!(result, "{}", disjoint()?)?;
    }

    println!("{result}");
    Ok(())
}