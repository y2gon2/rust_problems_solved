
// https://www.acmicpc.net/problem/1004
// 어린왕자

use std::io::*;

fn main() -> Result<()> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    stdin().lock().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..n {
        let pos: Vec<i32> = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let from = (pos[0], pos[1]);
        let to = (pos[2], pos[3]);

        let m = lines.next().unwrap().parse::<usize>().unwrap();
        let mut cross: usize = 0;

        for _ in 0..m {
            let nums: Vec<i32> = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            let dist_from_cir = (from.0 - nums[0]).pow(2)+ (from.1 - nums[1]).pow(2);

            let dist_to_cir = (to.0 - nums[0]).pow(2) + (to.1 - nums[1]).pow(2);

            let r_pow = nums[2].pow(2);
            if (dist_from_cir >= r_pow && dist_to_cir < r_pow ) ||
                (dist_from_cir < r_pow && dist_to_cir >= r_pow) {
                cross += 1;
            }
            // println!("r : {},  from-cir : {},  to_cir : {}", r, dist_from_cir, dist_to_cir);
        }

        writeln!(output, "{}", cross).unwrap();
    }

    Ok(())
}