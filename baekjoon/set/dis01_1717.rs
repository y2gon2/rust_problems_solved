//! https://www.acmicpc.net/problem/1717
//! 집합의 표현
//! 
//! 풀이 과정
//! 분리집합(disjoint set) 에 대한 첫 문제였기 때문에 알고리즘의 기본 개념을 우선 학습하고 풀이 진행
//! 각각의 원소들을 주어진 조건에 따라 원소들간에 연결관계를 가지게 됨으로써,
//! 원소들은 어떤 원소와은 연합 (union) 을 생성하게 되며 부분 연합 중 하나의 원소가 다른 연합과의 
//! 연결 관계를 얻게 되면서 두 연합은 하나의 연합으로 연결되게 된다. 
//! 이를 parent - child 관계로 edge 를 얻게 되면서 하나의 원소가 다른 원소와 root 가 같다면
//! 같은 연합 내 존재함을 확인할 수 있다. 
//! 
//! 해당 node 들이 index 정보만 가진 경우, 별도의 노드를 생성할 필요 없이 array 또는 vec 의 index 만으로
//! 표현가능해지며, 각 원소의 vale 는 자신의 parent element index 값을 포함 하게 된다. 
//! 
//! 내가 속한 연합의 root 를 찾고 싶다면 내 부모의 부모(부모 index 에 저장된 value) 을 계속 쫓아가면
//! root 값을 찾을 수 있다. 
//! 
//! root 는 본인 index 를 value 로 가지고 있다. 
//! 
//! 위 과정의 풀이는 쉽게 인지하였으나, 단순히 위 개념만으로 매번 부모찾기를 똑같이 진행하면 시간 초과가 발생한다.
//!
//! 시간초과 상황을 피하기 위해 한번 root 찾기를 진행했다면, 나의 부모를 해당 root 로 변경해야
//! 반복적으로 탐색을 최소화 할 수 있다. 

use std::io::{stdin, Read};
use std::fmt::Write;
use std::error::Error;

pub struct Disjoint {
    parent: Vec<usize>,
}

impl Disjoint {
    pub fn new(n : usize) -> Self {
        Self {
            parent: (0..=n).collect(),
        }
    }

    pub fn get_root(&mut self, num: usize) -> usize {
        let mut p = num;

        while self.parent[p] != p {
            p = self.parent[p];
        }

        self.parent[num] = p;  // root 를 찾았다면 내 부모를 root 로 변경해줘야 시간 초과를 피할 수 있다.
        return p
    }

    pub fn union(&mut self, num1: usize, num2: usize) {
        let root1 = self.get_root(num1);
        let root2 = self.get_root(num2);

        if root1 < root2 { 
            self.parent[root2] = root1; 
        } else {
            self.parent[root1] = root2;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut buf = String::new();

    let _ = stdin().read_to_string(&mut buf);
    let mut input = buf.split_ascii_whitespace();
    let mut get_n = || input
        .next()
        .unwrap()
        .parse::<usize>();

    let (n, m) = (get_n()?, get_n()?);
    let mut disjiont = Disjoint::new(n);

    for _ in 0..m {
        let (op, n1, n2) = (get_n()?, get_n()?, get_n()?);
        
        match op {
            0 => disjiont.union(n1, n2),
            1 => {
                match disjiont.get_root(n1) == disjiont.get_root(n2) {
                    true => writeln!(output, "YES")?,
                    false => writeln!(output, "NO")?,
                }
            },
            _ => return Err("inavailable operating number".into()),
        }
    }

    print!("{output}");
    Ok(())
}