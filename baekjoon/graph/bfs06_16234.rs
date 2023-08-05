//! https://www.acmicpc.net/problem/16234
//! 인구이동
//! 
//! * 문제 풀이
//! BFS 및 문제의 논리 구현에 익숙하지 않아 문제 풀이까지 오래 걸렸다. 
//! 추가로 결과를 도출하기 위해 필요한 적절한 자료 구조의 변수 선언과 이의 활용도 쉽지 않았다. 이를 우선 정리해보면
//!  - 기본 자료 : 정사각형 가로세로 칸수(n), 
//!               이동조건 최소값, 이동조건 최대값, 
//!               정사각형의 (n * n) 각 칸의 초기 인구 수
//!  - 각 칸의 방문여부를 저장 visited: Vec<Vec<false>>
//!  - 각 칸의 한차례 인구 이동 후 table  -> 주어진 초기 인구 테이블에 덮어씌워서 사용
//!  - 인구 이동이 발생했는지 여부 is_moved: bool
//!  - 전체 테이블을 모두 돌았을 때 인구 이동이 발생했다면 이를 count: usize
//!  - 각 칸을 순회 중 아직 방문하지 않은 칸의 경우, BFS 탐색 진행의 위한 queue : VecDeque<(usize, usize)> (좌표값 저장)
//!  - BFS 탐색 하면서 통합가능성이 있는 칸들을 기록하기 위한 sub_union : Vec<(usize, usize)> (좌표값 저장)
//!  - sub_list 에 담긴 칸들의 인구의 총합을 알아야 후에 평균값으로 재분배 가능하므로
//!    sub_list 인구를 계속 누적시킬 sub_sum : usize
//!  - BFS 탐색 4 방향 좌표 변환 저장 directions: [(usize, usize); 4]
//! 
//! 아래의 과정을 무한 반복하되 is_moved == false 일 때 해당 loop 를 빠져나온다.
//!  - 이중 for 으로 y, x 를 순회하면서 visited == false 다음 과정을 실행
//!  - queue 현재 좌표 넣고, sub_sum, sub_union 을 생성
//!  - while 문 내부 BFS 로직은 최대 최소 범위 내 포함 여부만 추가한 일반적인 BFS 구현
//!  - BFS 탐색이 완료되면 해당과정에서 저장된 sub_union 과 sub_sum 으로 인구 재분배를 진행
//!  - 전체를 탐색하면서 인구 재분배가 발생하지 않았다면 loop 에서 빠져나옴
//! 
//! * 오류 수정
//!  - 전체 탐색하는 과정에서 매번 방문 여부를 확인해야 하는데 이를 누락하여 제출 실패 발생
//! 
//! * 성능 개선 
//!  - 최초 풀이 : 13312KB / 368ms
//!  - BFS 탐색 진입 전 항상 sub_union, sub_sum 에 최초 진입 좌표 및 인구수를 넣으면서 시작했는데 
//!    실제로 BFS 탐색 조건 추가로 발견될 경우보다 1회 탐색으로 끝나버릴 경우가 높으므로
//!    이를 해당 조건을 queue 탐색이 추가 진행 되었을 때만 진행 
//!    -> 13312KB / 244ms
//!  - visited table 을 기존 Vec -> array 수정했으나, 필요없는 칸들이 중간 중간 존재하여 오히려 성능하락
//!    -> 13308KB / 264ms
//!  - sub_union 을 Vec -> HashMap 으로 변경하였으나 역시 성능 하락
//!    13356KB / 252ms

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::VecDeque;

fn union(lands:&mut Vec<Vec<usize>>, info: Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let (n, mn, mx) = (info[0], info[1], info[2]);
    let mut result = 0usize;

    let directions = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    
    loop {
        let mut visited = vec![vec![false; n]; n];
        let mut is_moved = false;

        for i in 0..n {
            for j in 0..n {
                if visited[i][j] { continue; }

                let mut queue = VecDeque::<(usize, usize)>::new();  // y, x
                let mut sub_sum = 0usize;
                let mut sub_union = Vec::<(usize, usize)>::new();
                  
                queue.push_back((i, j));
                visited[i][j] = true;

                while let Some((py, px)) = queue.pop_front() {
                    for (dy, dx) in directions.iter() {
                        let y = py as i32 + dy;
                        let x = px as i32 + dx;

                        if y >= 0 && x >= 0 && y < n as i32 && x < n as i32 {
                            let y = y as usize;
                            let x = x as usize;

                            if visited[y][x] { continue; } 

                            let diff  = (lands[py][px] as i32 - lands[y][x] as i32).abs() as usize;
                            if diff >= mn && diff <= mx {
                                queue.push_back((y, x));
                                visited[y][x] = true;

                                sub_sum += lands[y][x];
                                sub_union.push((y, x));
                            }
                        }
                    }                    
                }

                if sub_union.len() == 0 { continue; }
                
                // 성능 개선 부분
                sub_sum += lands[i][j];
                sub_union.push((i, j));

                is_moved = true;

                // println!("sub_union: {:?}, sub_sum :{}", sub_union, sub_sum);
                
                let average = sub_sum / sub_union.len();
                for (n_y, n_x) in sub_union {
                    lands[n_y][n_x] = average;
                }
            }
        }
        // println!("{:?}, {:?}, {}", lands, visited, result);
    
        if !is_moved { break; }
        result += 1;
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = stdout().lock();

    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut line = buf.lines();
    
    let info = line
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|e| e)?;

    let mut lands = Vec::<Vec<usize>>::new();
    for _ in 0..info[0] {
        lands.push(
            line.next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        );
    }
    
    let result = union(&mut lands, info)?;
    writeln!(output, "{}", result)?;
    Ok(())
}