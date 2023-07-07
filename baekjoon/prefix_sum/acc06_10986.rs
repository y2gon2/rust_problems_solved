//! https://www.acmicpc.net/problem/10986
//! 나머지 합
//! 
//! 풀이과정
//! 수학적 이해가 없이 prefix sum 만 이용하여 플었더니 시간 초과.
//! modular 연산의 이해와 이에 대한 구현히 추가적으로 필요
//! 
//! 1. (sum[i] - sum[j]) % M == 0 이면  counter += 1;
//! 
//! 2. 연속된 수들의 합 :   sum[i] - sum[j] 
//!    합의 나머지가 0  :  (sum[i] - sum[j]) % M == 0
//!                  ->   sum[i] % M = sum[j] % M   
//! 
//!    -> 누적 합 % M != 0 이 아니더라도 해당 수 이전 누적합 % M 의 값이 같다면 
//!    -> counter += 1; 해줘야
//!    -> 이는 곧 Modular 연산의 결과값(나머지) 가 동일한 경우의  n 중에 2개를 선택하는 조합 
//!       nC2 의 갯수 만큼 결과값에 반영해주어야 함. 
//! 
//! 참고: https://justicehui.github.io/ps/2019/04/04/BOJ10986/

use std::io::{stdin, stdout, Write, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut result = 0usize;
    let mut output = stdout().lock();
    
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut line_iter = buf.lines();
    let mut data = || line_iter
        .next()
        .unwrap()
        .split_ascii_whitespace();

    let mut info = data();
    let (n, m) = (
        info.next().unwrap().parse::<usize>()?,
        info.next().unwrap().parse::<usize>()?,
    );

    let mut nums = [0usize; 1_000_001];
    let mut rest_cnt = [0usize; 1001];          // 누적합을 m 으로 나눴을 때 나머지가 같은 것들끼리 count 
    let mut input = data();
    for i in 1..=n {
        let num = input.next().unwrap().parse::<usize>()?;
        nums[i] = nums[i - 1] + num;

        let rest = nums[i] % m;
        if rest == 0 {
            result += 1;
        }
        rest_cnt[rest] += 1;
    }

    for i in 0..m {
        let rest = rest_cnt[i];
        if rest != 0 {
            result += rest * (rest - 1) / 2;
        }
    }

    writeln!(output, "{}", result)?;
    Ok(())
}