// https://www.acmicpc.net/problem/1213
// 팰린드롬 만들기

use std::io::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    
    let length: usize = input.len();

    let mut hash = HashMap::<char, usize>::new();

    for c in input.chars() {
        let _ = match hash.get(&c) {
            None => hash.insert(c, 1),
            Some(v) => hash.insert(c, v + 1),
        };
    }

    let mut rename = vec!['a'; length];
    let mut cnt: usize = 0;
    let mut odd_check: usize = 0;

    for c in 'A'..='Z' {
        match hash.get(&c) {
            None => (),
            Some(v) => {
                if v % 2 == 1 {
                    if odd_check >= 2 {
                        break;
                    } else {
                        odd_check += 1;
                        let half_v = v / 2;

                        for i in 0..half_v {
                            rename[cnt + i] = c;
                            rename[length - 1 - (cnt + i)] = c;
                        }

                        rename[length / 2] = c;

                        cnt += half_v;
                    }
                } else {
                    let half_v = v / 2;
                    
                    for i in 0..half_v {
                        rename[cnt + i] = c;
                        rename[length - 1 - (cnt + i)] = c;
                    }
                    cnt += half_v;
                }
            },
        }
    }

    if odd_check >= 2 {
        println!("I'm Sorry Hansoo");
    } else {
        let result: String = rename.into_iter().collect();
        println!("{}", result);
    }

    Ok(())
}