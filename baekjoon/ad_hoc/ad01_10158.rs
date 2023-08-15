//! https://www.acmicpc.net/problem/10158
//! 개미
//! 

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout();
    let mut buf = String::new();

    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<u32>();

    let (w, h, p, q) = (get_n()?, get_n()?, get_n()?, get_n()?);
    let t = get_n()?;

    let mut x = t + p;
    let mut y = t + q;

    // y 축으로 무한으로 연장된 범위로 가정하면 앞으로 계속 전진하면서 좌우로 부딪히며 이동한다.
    // 만약 마지막에 우측으로 가면서 끝났다면, w 의 나머지가 x 값
    // 만약 좌측으로 가면서 끝났다면, (w - 나머지) 가 x 값
    if x % (2 * w) >= w {
        x = w - (x % w);
    } else {
        x %= w;
    }

    // x 축으로 무한으로 연장된 범위로 가정하면 계속 우측으로 가면서 위아래로 부딪히며 이동한다.
    // 만약 마지막에 올라가면서 끝났다면,  의 나머지가 y 값
    // 만약 내려오면서 끝났다면, (h - 나머지) 가 y 값
    if y % (2 * h) >= h {
        y = h - (y % h);
    } else {
        y %= h;
    }

    writeln!(output, "{} {}", x, y)?;
    Ok(())
}

