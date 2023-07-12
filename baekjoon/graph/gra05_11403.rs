// https://www.acmicpc.net/problem/11403
// 경로 찾기

use std::io::{stdin, stdout, BufWriter, Read};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut result = String::new();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_num = || input.next().unwrap().parse::<usize>();

    let n = get_num()?;
    let mut links = Vec::<Vec<usize>>::new();
    links.push(vec![0;1]);
    let mut visited = vec![vec![false; n + 1]; n + 1];

    for i in 1..=n {
        let mut row = Vec::<usize>::new();
        for j in 1..=n {
            if get_num()? == 1 {
                row.push(j);
            }
        }
        links.push(row);
    }

    for i in 1..=n {
        let mut stack = links[i].clone();

        while let Some(x) = stack.pop() {
            visited[i][x] = true;

            for x2 in  links[x].iter() {
                if visited[][]
            }
        }
    }

    for i in 1..=n {
        println!("{:?}", links[i]);
    }

    
    Ok(())
}
 