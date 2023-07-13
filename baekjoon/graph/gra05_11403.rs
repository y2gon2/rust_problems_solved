//! https://www.acmicpc.net/problem/11403
//! 경로 찾기
//!
//! 풀이과정
//! 오래 걸렸다.;; 
//! 일전에 방향성 있는 그래프라는 부분을 제대로 이해하지 못해 문제 이해를 하지 못했었다. 
//! 
//! i -> j 에 대해서 그래프를 정리하여 이를 순환하려고 했지만,
//! 이보다 그냥 matrix 형태 그대로 받고 해당 matrix 를 순환하면서 연결 그래프를 따라
//! 방문한 곳을 visitd 에 행 별로 채워 나가는게 좀더 직관적이여서 규현이 쉬웠다.  


use std::io::{stdin, stdout, Write, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();
    let mut result = String::new();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let n = get_num()?;
    let mut links = Vec::<Vec<usize>>::new();
    links.push(vec![0;1]);
    let mut links = vec![vec![false; n + 1]; n + 1];
    let mut visited = vec![vec![false; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            if get_num()? == 1 {
                links[i][j] = true;
            }
        }
    }

    for i in 1..=n {
        for j in 1..=n {
            if !links[i][j] { continue; }

            let mut stack = Vec::<usize>::new();
            stack.push(j);
            visited[i][j] = true;

            while let Some(x) = stack.pop() {    
                for (idx, x2) in links[x].iter().enumerate() {
                    if *x2 && !visited[i][idx] {
                        stack.push(idx);
                        visited[i][idx] = true;
                    }
                }
            }

            // println!("----------------------------------------------------------");
            // for q in 1..=n {
            //     println!("{:?}", visited[q]);
            // }
            // println!("----------------------------------------------------------");
        }
    }

    
    for i in 1..=n {
        for j in 1..=n {
            if visited[i][j] {
                result += "1 ";
            } else {
                result += "0 ";
            }
        }
        result += "\n";
    }

    writeln!(output, "{}", result)?;
    
    Ok(())
}
 