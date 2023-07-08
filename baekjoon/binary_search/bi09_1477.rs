//! https://www.acmicpc.net/problem/1477
//! 휴게소 세우기
//! 
//! 문제풀이
//! - 아직 이분탐색 이해가 부족한듯 하다. 
//!   1. 최초 이분탐색으로 해당 문제의 풀이 방법을 설정하는데까지 너무 시간이 오래 걸렸다. 
//!   2. 간격 중에 현재 탐색 길이 기준으로 어떻게 필요한 답을 얻을 수 있는지 결국 다른 해결 방법을 참조했다.
//!   -> 간격 둘 중에 우선 나눠질 수 있다는 건 최대값인지 알 수 없지만 
//!      중간에 몫의 갯수만큼 휴게소를 새울 수 있다는 의미이다.
//!   -> 만약 나머지 없이 나누어 떨어진다면, 이때는 몫이 실제 사이 갯수보다 1개 더 많이 나오므르 -1 해준다.
//!   -> 만약 사이의 갯수를 세었을때 원하는 갯수와 같다면(if cnt == m),
//!      이는 아직 나누는 포인트가 한쪽으로 치우친 (최적의 값이 아닌) 경우일 수 있다. 
//!      따라서, 이를 최적으로 하기 위해서는 cnt == m 중에서 그 길이가 가잘 짧은 길이를 선택해야 한다. 
//!      -> if cnt == m 은 mx 를 작게 움직이는 경우에 포함되어야 한다. 
//!   -> 최종 이분탐색에서 선택되어야 값이 mn,  mx 중 앞의 선택 조건에 따라 mx 는 마지막 반복문에서
//!      mid 값보다 더 작아질 수 있으므로, mn 을 return (이건 논리적으로 정확한 추론인지 모르겠다.)

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;

fn binary_search(m: usize, distances: &mut Vec<usize>) -> usize {
    let mut mn = 1usize;
    let mut mx = distances[0];

    while mn <= mx {
        let mid = (mn + mx) / 2;

        let mut cnt = 0usize;
        for d in distances.iter() {
            cnt +=  *d / mid;
            if d % mid == 0 { cnt -= 1; } 
        }

        println!("mn:{}   mid:{}   mx:{}  cnt:{}", mn, mid, mx, cnt);
        if cnt > m {
            mn = mid + 1;
        } else {
            mx = mid - 1;
        }
    }   
    mn    
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(stdin().lock())?;
    let mut read = input.split_ascii_whitespace();
    
    let n = read.next().unwrap().parse::<usize>()?;
    let m = read.next().unwrap().parse::<usize>()?;
    let l = read.next().unwrap().parse::<usize>()?;

    let mut rests: Vec<usize> = read
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    rests.push(0);
    rests.sort();
    rests.push(l);

    let mut distances: Vec<usize> = Vec::new();

    for i in 1..n + 2{
        distances.push(rests[i] - rests[i - 1]); 
    }
    distances.sort_by(|a, b| b.cmp(a));

    let result = binary_search(m, &mut distances);

    let mut output = stdout().lock();
    writeln!(output, "{}", result)?;

    Ok(())
}

