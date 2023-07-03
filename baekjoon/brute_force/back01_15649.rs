// https://www.acmicpc.net/problem/15649
// N과 M (1)

use std::io::{stdin, stdout, BufWriter, Write, read_to_string};
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

fn back_tracking(n: usize, m: usize, result: Rc<RefCell<Vec<usize>>>) -> String {
    if result.borrow().len() == m {
        let mut ans = String::new();
        for i in 0..m {
            ans = ans + &result.borrow()[i].to_string() + " ";
        }
        
        return ans + "\n";
    }

    let mut answer = String::new();
    for i in 1..=n {
        if !result.borrow().contains(&i) {
            result.borrow_mut().push(i);
            
           answer = answer + &back_tracking(n, m, result.clone());

            result.borrow_mut().pop();
        }
    }

    return answer
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string(stdin().lock())?;
    let nums: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|s|s.parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|e| e)?;

    let result = Rc::new(RefCell::new(Vec::<usize>::new()));
    
    let answer = back_tracking(nums[0], nums[1], Rc::clone(&result));

    println!("{}", answer);
    Ok(())
}
