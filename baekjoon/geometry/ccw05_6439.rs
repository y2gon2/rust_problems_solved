//! https://www.acmicpc.net/problem/6439
//! 교차

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

fn ccw(x1: i32, x2: i32, x3: i32, y1: i32, y2: i32, y3: i32) -> i8 {
    let result = (x3 - x1) * (y2 - y1) - (x2 - x1) * (y3 - y1);
    
    if result < 0 { return -1 } 
    else if result == 0 { return 0 } 
    else { return 1 }
}

fn is_cross(s: S, l: L) -> bool {
    // 선분 끝점 중 한점이라도 사각형 내부 (경계선 포함)에 들어 있는 경우 true
    if (l.start.x >= s.btn_left.x && l.start.y >= s.btn_left.x 
        && l.start.x <= s.top_right.x && l.start.y <= s.top_right.y) 
    || (l.end.x >= s.btn_left.x && l.end.y >= s.btn_left.x 
        && l.end.x <= s.top_right.x && l.end.y <= s.top_right.y)  {
        println!("000");
        return true
    }

    let mx_x = l.start.x.max(l.end.x);
    let mn_x = l.start.x.min(l.end.x);
    let mx_y = l.start.y.max(l.end.y);
    let mn_y = l.start.y.min(l.end.y);

    // 선분의 양 끝점이 모두 직사각형의 각 변의 한쪽 밖에 몰려 있는 경우 false
    if mn_x > s.top_right.x || mn_y > s.top_right.y || mx_x < s.btn_left.x || mx_y < s.btn_left.y {
        println!("1111");
        return false
    } 

    // 선분의 양 끝점 모두 직사각형 위 또는 내부에 존재하지 않는 경우
    // c1: \ 방향 대각선, c2: / 방향 대각선
    let l_to_c1 = ccw(l.start.x, l.end.x, s.btn_left.x, l.start.y, l.end.y, s.top_right.y) 
        * ccw(l.start.x, l.end.x, s.top_right.x, l.start.y, l.end.y, s.btn_left.y);

    let c1_to_l = ccw(s.btn_left.x, s.top_right.x, l.start.x, s.top_right.y, s.btn_left.y, l.start.y) 
        * ccw(s.btn_left.x, s.top_right.x, l.end.x, s.top_right.y, s.btn_left.y, l.end.y);

    let l_to_c2 = ccw(l.start.x, l.end.x, s.btn_left.x, l.start.y, l.end.y, s.btn_left.y) 
        * ccw(l.start.x, l.end.x, s.top_right.x, l.start.y, l.end.y, s.top_right.y);
    
    let c2_to_l = ccw(s.btn_left.x, s.top_right.x, l.start.x, s.btn_left.y, s.top_right.y, l.start.y) 
        * ccw(s.btn_left.x, s.top_right.x, l.end.x, s.btn_left.y, s.top_right.y, l.end.y);

    println!(" l_to_c1:{}  c1_to_l:{},  l_to_c2:{}, c2_to_l:{}",  l_to_c1, c1_to_l, l_to_c2, c2_to_l);        
    if (l_to_c1 <= 0 && c1_to_l <= 0) || (l_to_c2 <= 0 && c2_to_l <= 0) {
        println!("2222");
        return true
    } 

    println!("333");
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
            result += "T\n";
        } else {
            result += "F\n";
        }
    }

    print!("{result}");
    Ok(())
}


// test case 들  (다 통과했는데 ㅜㅜ)

// 1
// 3 1 4 1 0 0 7 7
// # answer : T

// 1
// 4 9 11 2 1 5 7 1
// F
// 1
// -1 1 1 -1 0 0 7 7
// T

// 1
// -1 1 1 -1 7 0 0 7
// T

// 3
// 7 5 7 9 0 0 7 7
// -1 1 1 -1 7 0 0 7
// -1 1 1 -1 0 0 7 7
// T
// T
// T

// 1
// -1 -1 2 16 1 7 7 1 
// F

// 3
// -1 -1 3 16 1 7 7 1 
// -1 -1 3 15 1 7 7 1 
// -1 -1 4 16 1 7 7 1 
// F
// T
// T

// 1
// 0 0 2 2 1 1 2 1
// T

// 1
// 1 1 -1 -1 1 -1 0 0 
// T