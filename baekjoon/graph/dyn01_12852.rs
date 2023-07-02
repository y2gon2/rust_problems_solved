//! https://www.acmicpc.net/problem/12852
//! 1로 만들기 2
//! 
//! 문제 접근
//! - 일반적인 BFS 로 풀어보려가 했으나, 최단 경로를 기억해야 한다는 점이 문제가 됨
//! - 모든 경로를 queue 에 함께 넣으니까 시간 초과됨
//! 
//! 문제 해결 
//! - 목적지가 항상 '1' 이므로 1에서 부터 경로를 탐색하는 것으로 변경
//! - queue 에 모든 경로를 담지 않고, memoization 을 각 포인트마다 지금까지 온 경로를 저장
//!   358708KB / 436ms
//! 
//! - 출력 방법 개선 (vec 을 모두 string 으로 변환하여 한번의 출력만 진행)
//!   358712KB / 428ms
//!   -> 거의 비슷
//! 
//! 성능 향상
//! - 모든 경로를 계속 vec 에 담고 있는 것이 메모리 낭비라고 생각되서, 저장된 경로를 queue 싣고
//!   pop 될 경우, 해당 용량이 절약될 것이라고 생각됨
//!   107340KB / 356ms 
//!   -> 메모리 사용량을 줄이는데 효과가 있음. 
//!      queue 에서 꺼내온 값이 목적지에 도달하면 바로 작업을 종료하기 때문에 약간의 시간 절약
//! 
//! - 다른 풀이에서 memoization 에서 모든 경로를 저장하는 것이 아닌, 각 포인트에서 이전에 어디서 와야
//!   가장 최단인지 이전 포인트만 저자아는 방법이 있음을 확인, 이를 적용
//!   28788KB / 16ms
//!   -> 메모리 사용과 시간이 매우 개선됨.
//!   -> 핵심은 각 포인트가 자신까지 도달하는데 최단 거리만 기억하면 된다는것.
//!   -> 이 방법을 최초에 대략 생각했었으나, 임의의 출발점에서 1로 가는 방법으로는 생각할 수 없었음.
//!   -> 추가로 경로를 출력 할때도 주어진 출발점에서 부터 1을 향해 가다보면 1에서는 0 을 가지고 있기 때문에
//!   -> '0' 의 값을 가진 곳까지 출력하면 된다는 것도 생각하지 못했음.

use std::io::{stdin, stdout, Write, read_to_string};
use std::error::Error;

fn bfs(num: usize) -> (Vec<usize>, Vec<usize>) {
    let mut cnt = vec![0usize; num + 1];
    let mut routes = vec![0usize; num + 1];

    for i in 2..=num {
        cnt[i] = cnt[i - 1] + 1;
        routes[i] = i - 1;

        if i % 2 == 0 {
            if cnt[i] > cnt[i / 2] + 1 {
                cnt[i] = cnt[ i / 2] + 1;
                routes[i] = i / 2;
            }
        }

        if i % 3 == 0 {
            if cnt[i] > cnt[i / 3] + 1 {
                cnt[i] = cnt[i / 3] + 1;
                routes[i] = i / 3;
            }
        }
    }

    (cnt, routes)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(stdin().lock())?;
    let num = input.trim().parse::<usize>()?;

    let (cnt, routes) = bfs(num);

    let mut output = stdout().lock();
    writeln!(output, "{}", cnt[num])?;

    let mut route = num;
    let mut result = Vec::<usize>::new();
    
    while route > 0 {
        result.push(route);
        route = routes[route];
    }

    let result = result
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(output, "{}", result)?;

    Ok(())
}