// let v: Vec<usize> = buf
//     .split_ascii_whitespace()
//     .map(|c| c.parse::<usize>())
//     .collect::<Result<_, _>>()?;

// let (n, l, w, h) = match v.as_slice() {
//     [n, l, w, h] => (*n, *l, *w, *h),
//     _ => return Err("Invalid input".into()),
// };