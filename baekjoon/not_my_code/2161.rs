// Origin
// baekjoon ID: 	bamgoesn  (https://www.acmicpc.net/source/53048506)

#![no_main]
use std::{collections::VecDeque, io::*, iter::FromIterator};

#[no_mangle]
fn main() -> i32 {
    // FastIO
    use fastio::*;
    let input_str = get_input();
    let mut sc: Splitter<_> = Splitter::new(input_str, |s| s.split_ascii_whitespace());
    let stdout = stdout();
    let wr = &mut BufWriter::new(stdout.lock());

    // FastIO Macros
    macro_rules! out { ($($arg:tt)*) => { write!(wr, $($arg)*).ok(); }; }
    macro_rules! outln { ($($arg:tt)*) => { writeln!(wr, $($arg)*).ok(); }; }

    // Main
    let mut ans = vec![];
    let n: usize = sc.next();
    let mut dq = VecDeque::from_iter(1..=n);

    for _ in 0..n - 1 {
        ans.push(dq.pop_front().unwrap());
        let x = dq.pop_front().unwrap();
        dq.push_back(x);
    }
    ans.push(dq.pop_front().unwrap());

    for v in ans {
        out!("{} ", v);
    }
    outln!();

    wr.flush().unwrap();
    0
}

mod fastio {
    use core::{slice::*, str::*};

    #[link(name = "c")]
    extern "C" {
        fn mmap(addr: usize, len: usize, p: i32, f: i32, fd: i32, o: i64) -> *mut u8;
        fn fstat(fd: i32, stat: *mut usize) -> i32;
    }

    pub fn get_input() -> &'static str {
        let mut stat = [0; 20];
        unsafe { fstat(0, stat.as_mut_ptr()) };
        let buffer = unsafe { mmap(0, stat[6], 1, 2, 0, 0) };
        unsafe { from_utf8_unchecked(from_raw_parts(buffer, stat[6])) }
    }

    pub struct Splitter<I: Iterator> {
        it: I,
    }

    impl<'a, 'b: 'a, T: Iterator> Splitter<T> {
        pub fn new(s: &'b str, split: impl FnOnce(&'a str) -> T) -> Self {
            Self { it: split(s) }
        }
    }

    impl<'a, I: Iterator<Item = &'a str>> Splitter<I> {
        pub fn next<T: FromStr>(&mut self) -> T {
            self.it.next().unwrap().parse().ok().unwrap()
        }
    }
}