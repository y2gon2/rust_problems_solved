// https://www.acmicpc.net/problem/1198
// 삼각형으로 자르기

use std::io::{Result, stdin, BufRead, BufWriter, stdout};

fn main() -> Result<()> {
    let mut input = stdin().lock().lines().map(|line| line.unwrap());
    let mut _output = BufWriter::new(stdout().lock());
    let mut mx_area: i64 = 0;

    let n = input.next().unwrap().parse::<i64>().unwrap();
    let mut line = || input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    let mut points: Vec<Vec<i64>> = Vec::new();

    for _ in 0..n {
        points.push(line());
    }

    // 삼각형 세점의 좌표를 알때 넓이 구하기
    // s = 1/2 * |(x1y2 + x2y3 + x3y1) - (x1y3 + x3y2 + x2y1)|
    for i in 0..n as usize{
        for j in (i + 1)..n as usize{
            for k in (j + 1)..n as usize {
                let sq = (
                    (points[i][0] * points[j][1] + points[j][0] * points[k][1] + points[k][0] * points[i][1])
                    - (points[j][0] * points[i][1] + points[k][0] * points[j][1] + points[i][0] * points[k][1])                
                ).abs();
    
                mx_area = mx_area.max(sq);
            }
        }
    }

    if mx_area % 2 == 1 {println!("{}.5", mx_area / 2 ); }
    else { println!("{:.1}", mx_area / 2);} 

    Ok(())
}