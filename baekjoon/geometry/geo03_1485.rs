// https://www.acmicpc.net/problem/1485
// 정사각형

use std::io::{Result, stdin, Read};

fn cal_dist_sqrt(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
    (x1 as i64 - x2 as i64).pow(2) + (y1 as i64 - y2 as i64).pow(2)
}
fn main() -> Result<()> {
    // let mut output = BufWriter::new(stdout().lock());
    let mut result: Vec<usize> = Vec::new();

    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let n = lines.next().unwrap().trim().to_string().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut points: Vec<Vec<i32>> = Vec::new(); 
        for _ in 0..4 {
            let point: Vec<i32> = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect();
            points.push(point);
        }

        let mut dists: Vec<i64> = Vec::new();
        for j in 0..4 {
            let dist = cal_dist_sqrt(
                points[j][0], points[j][1], points[(j + 1) % 4][0], points[(j + 1) % 4][1]
            );
            dists.push(dist);
        }
        dists.push(cal_dist_sqrt(points[0][0], points[0][1], points[2][0], points[2][1]));
        dists.push(cal_dist_sqrt(points[1][0], points[1][1], points[3][0], points[3][1]));

        dists.sort();

        // (0..6).for_each(|x| println!("{}", &dists[x]));

        if dists[4] == dists[5] && dists[0] == dists[2] && dists[0] == dists[3] && dists[4] == dists[5] {
            result.push(1);
        } else {
            result.push(0);
        }
    }

    for r in result.iter() {
        println!("{}", r);
    }
    Ok(())
}