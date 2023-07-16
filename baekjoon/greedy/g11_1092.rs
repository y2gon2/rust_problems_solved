//! https://www.acmicpc.net/problem/1092
//! 배
//! 
//! 문제 풀이
//! 큰 crane 이 아닌 작은 crane 부터 본인의 할당량을 채워 나간다. 
//! 
//!  case1 
//!  할당량 (contain 수랑 % 남은 cran 수 == 0 이면 몫 != 0 이면 몫 + 1) 을 모두 채우면,
//!  다음 crane 에 다음 container 부터 처리수량을 채워나감
//!  case2
//!  현재 container 가 허용중량 초과의 경우, 다음 crane 으로 넘김, 
//!  이때 남은 crane 수와 container 수를 가지고 할당량을 다시 정함.
//! 
//! 그러나 case0 를 초반에 잘못 설정하여 좀 시간 걸림
//!  case0
//!  최고 중량 crane 이 최고 중량 container 를 처리 못하면 -> 무조건 -1


use std::io::{stdin, stdout, BufRead, Write};
use std::error::Error;
use std::cmp::max;

fn get_num() -> Result<usize, Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_line(&mut buf);
    let n = buf.trim().parse::<usize>()?;

    Ok(n)
}

fn get_vec() -> Result<Vec<usize>, Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_line(&mut buf);
    let nums = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| e)?;

    Ok(nums)
}

fn main() -> Result<(), Box<dyn Error>> { 
    let mut output = stdout().lock(); 
    let mut result = 0usize;

    let n = get_num()?;
    let mut cranes = get_vec()?;
    cranes.sort();

    let m = get_num()?;
    let mut containers = get_vec()?;
    containers.sort();

    let mut remain = m;
    let mut consume = vec![0usize; n];
    let mut cnt = 0usize;
    
    #[allow(unused_assignments)]
    let mut quota = 0usize;
    match remain % n {
        0 => quota = remain / n,
        _ => quota = remain / n + 1,
    }

    if cranes[n - 1] < containers[m - 1] { 
        writeln!(output, "-1")?;
        return Ok(())
    }

    for (idx, crane) in cranes.iter().enumerate() {
        for j in cnt..m {
            if crane >=  &containers[j] && consume[idx] < quota {
                consume[idx] += 1;
                remain -= 1;
                cnt += 1;
            } else {
                if idx < n - 1 {
                    match remain % (n - 1 - idx) {
                        0 => { quota = remain / (n - 1 - idx); },
                        _ => { quota = (remain / (n - 1 - idx)) + 1; },
                    }
                    break;
                }
            }
            // println!("cnt:{} quota:{},   {:?}", cnt, quota, consume);
        }
    }

    for i in consume {
        result = max(result, i);
    }

    writeln!(output, "{}", result)?;
    Ok(())
}