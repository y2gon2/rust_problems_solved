use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(stdout().lock());
    let mut buf = String::new();
    
    stdin().lock().read_to_string(&mut buf)?;
    let mut nums: Vec<i32> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()?;

    let n = nums.remove(0) as usize;

    let mut freq_map = HashMap::new();
    for &num in &nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut result: usize = 0;

    for i in 0..n {
        for j in i+1..n {
            let target = nums[i] + nums[j];
            if let Some(&count) = freq_map.get(&target) {
                if (target == nums[i] && count > 1) || (target == nums[j] && count > 1) || (target != nums[i] && target != nums[j]) {
                    result += 1;
                    break;
                }
            }
        }
    }

    writeln!(output, "{}", result)?;

    Ok(())
}