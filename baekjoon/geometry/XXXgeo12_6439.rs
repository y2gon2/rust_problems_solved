//! https://www.acmicpc.net/problem/6439
//! 교차

use std::borrow::BorrowMut;
use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;

struct Xy {
    x: i32,
    y: i32,
}

impl Xy {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

struct S {
    btn_left: Xy,
    top_right: Xy,
}

impl S {
    pub fn new() -> Self {
        Self {
            btn_left: Xy::new(),
            top_right: Xy::new(),
        }
    }
}

struct L {
    start: Xy,
    end: Xy,
}

impl L {
    pub fn new() -> Self {
        Self {
            start: Xy::new(),
            end: Xy::new(),
        }
    }
}

fn ccw(l: &L, x3: i32, y3: i32) -> i8 {
    let result = ((l.end.x - l.start.x) * (y3 - l.start.y) - (x3 - l.start.x) * (l.end.y - l.start.y));
    if result == 0 {
        return 0
    } else if result > 0 {
        return 1
    }else {
        return -1
    }
}

fn is_cross(s: S, l: L) -> bool {
    // 선분 끝점 중 한점이라도 사각형 내부에 들어 있는 경우 true
    if (l.start.x >= s.btn_left.x && l.start.y >= s.btn_left.x 
        && l.start.x <= s.top_right.x && l.start.y <= s.top_right.y) 
    || (l.end.x >= s.btn_left.x && l.end.y >= s.btn_left.x 
        && l.end.x <= s.top_right.x && l.end.y <= s.top_right.y)  {
        return true
    }

    // 선분의 끝점 모두 사각형의 큰 값보다 크거나, 작은값보다 작은 쪽에 치우쳐 있는 경우 false
    if (l.start.x < s.btn_left.x  && l.end.x < s.btn_left.x) 
    || (l.start.y < s.btn_left.y && l.end.y < s.btn_left.y)
    || (l.start.x > s.top_right.x && l.end.x > s.top_right.x)
    || (l.start.y > s.top_right.y && l.end.y > s.top_right.y) {
        return false
    }

    // 사각형을 기준으로 선분의 양 끝점이 나누어진 공간에 위치하는 경우
    if (ccw(&l, s.btn_left.x, s.btn_left.y) * ccw(&l, s.btn_left.x, s.top_right.y)) < 0
    || (ccw(&l, s.btn_left.x, s.top_right.y) * ccw(&l, s.top_right.x, s.top_right.y)) < 0
    || (ccw(&l, s.top_right.x, s.top_right.y) * ccw(&l, s.top_right.x, s.btn_left.y)) < 0
    || (ccw(&l, s.top_right.x, s.btn_left.y) * ccw(&l, s.btn_left.x, s.btn_left.y)) < 0 {
        return true
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = String::new();

    let mut buf = String::new();
    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<i32>();

    let n = get_n()?;

    for _ in 0..n {
        let mut s = S::new();
        let mut l = L::new();

        (l.start.x, l.start.y) = (get_n()?, get_n()?);
        (l.end.x, l.end.y) = (get_n()?, get_n()?);

        let (x1, y1, x2, y2) = (get_n()?, get_n()?, get_n()?, get_n()?);

        if x1 < x2 {
            s.btn_left.x = x1;
            s.top_right.x = x2;
        } else {
            s.btn_left.x = x2;
            s.top_right.x = x1;
        }

        if y1 < y2 {
            s.btn_left.y = y1;
            s.top_right.y = y2;
        } else {
            s.btn_left.y = y2;
            s.top_right.y = y1;
        }

        if is_cross(s, l) {
            writeln!(result, "T")?;
        } else {
            writeln!(result, "F")?;
        }
    }

    println!("{result}");
    Ok(())
}
