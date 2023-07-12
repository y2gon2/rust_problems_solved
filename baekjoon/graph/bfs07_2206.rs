//! https://www.acmicpc.net/problem/2206
//! 벽 부수고 이동하기
//! 
//! 풀이과정
//! DFS BFS 에 대한 이해 부족으로 최초 DFS 로 접근함. 
//! 다음 블로그에서 아래 3가지 조언을 얻어 이에 맞게 다시 풀이 진행 (https://kscodebase.tistory.com/66)
//!  - 가중치가 없는 최단 경로는 모조건 BFS 입니다.
//!  - 모든 칸을 전부 0으로 하나씩 바꾸어 BFS 를 돌리는 경우 시간초과 발생 -> O(NM)^2
//!  - 두번째 상황을 피하기 위해서 벽을 부셨는가 여부에 대한 기록이 필요
//! 
//! 따라서 BFS 에서 벽을 부수지 않고 방문한 visited 와 부수고 방문한 wrecked matix 를 생성하여
//! 다음 조건에 따라 해당 칸의 방문 여부로 각각 기록
//!  - 모든 조건은 visited 가 false 일 때 이동 가능
//!  - 한번도 방문하지 않고 이동했다면 visited 와 wrecked 모두 true 뚫은 횟수는 0
//!  - (최초) 벽울 뚫을 때 visited 와 wrecked 모두 true 뚫은 횟수 +1
//!  - 한번 벽을 뚥고 움직일 때, wrecked 만 true
//!  - 그외 경우 skip
//!  -> 위와 같은 설정일 때, 한번 벽을 통과하고 지나간 칸을 한번도 뚫지 않고 방문했을 때 재 방문 가능해짐

use std::io::{stdin, stdout, Read, Write};
use std::error::Error;
use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; 

fn get_input() -> Result<(usize, usize, Vec<Vec<bool>>), Box<dyn Error>> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let mut buf_iter = buf.split_ascii_whitespace();
    
    let n = buf_iter.next().unwrap().parse::<usize>()?;
    let m = buf_iter.next().unwrap().parse::<usize>()?;

    let mut map = vec![vec![true; m]; n];
    for i in 0..n {
        for (j, ch) in buf_iter.next().unwrap().char_indices() {
            if ch == '1' {
                map[i][j] = true;
            } else {
                map[i][j] = false;
            }
        }        
    }

    Ok((n, m, map))
}

fn bfs(
    n: usize, 
    m: usize, 
    map: &Vec<Vec<bool>>
) -> i32 {
    let mut queue = VecDeque::<(usize, usize, i32, i32)>::new(); // y, x, cnt, wrecking
    let mut visited = vec![vec![false; m]; n];
    let mut wrecked = vec![vec![false; m]; n];
    queue.push_back((0, 0, 1, 0));
    while let Some((y, x, cnt, mut wrecking)) = queue.pop_front() {
        // println!("map[{}][{}], cnt:{}, wrecking:{}", y, x, cnt, wrecking);
        // println!("map[7][2]:{} visited[7][2]:{} wrecked[7][2]:{}", map[7][2], visited[7][2], wrecked[7][2]);
        
        if y == n - 1 && x == m - 1 {
            return cnt;
        }
        
        if wrecking == 0  {
            visited[y][x] = true;
        }
        wrecked[y][x] = true; 

        for (dy, dx) in DIRECTIONS.iter() {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
    
            if ny >= 0 && nx >= 0 && ny < n as i32 && nx < m as i32 {
                let ny = ny as usize;
                let nx = nx as usize;
                let mut this_wreck = wrecking;

                if visited[ny][nx] { continue; }
                if wrecking == 1 && !map[ny][nx] && !wrecked[ny][nx] { // 한번 벽을 뚫은 상태에서 움직일때
                    wrecked[ny][nx] = true; 
                } else if wrecking == 0 && map[ny][nx] { // 벽을 (최초) 뚫을때
                    visited[ny][nx] = true;
                    wrecked[ny][nx] = true;
                    this_wreck += 1;
                } else if wrecking == 0 && !map[ny][nx] { // 한번도 벽울 뚫지 않고 갈때
                    visited[ny][nx] = true;
                    wrecked[ny][nx] = true;
                } else {
                    continue;
                }
                queue.push_back((ny, nx, cnt + 1, this_wreck));
            }
        }
        // println!("{:?}", queue);
    }
    return -1;
}


fn main() -> Result<(), Box<dyn Error>>{
    let (n, m, map) = get_input()?;

    let mut output = stdout().lock();
    writeln!(output, "{}", bfs(n, m, &map))?;

    Ok(())
}