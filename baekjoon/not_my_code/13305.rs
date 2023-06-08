use std::{
    io::*,
    str::{FromStr, SplitWhitespace},
};

fn main() {
    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut input_str = String::new();
    handle
        .read_to_string(&mut input_str)
        .expect("Failed to read");
    let mut scan = Scanner::new(&input_str);
    let stdout = stdout();
    let out = &mut BufWriter::with_capacity(262144, stdout.lock());

    macro_rules! next {
        () => {
            scan.next()
        };
        ($t:ty) => {
            scan.next::<$t>()
        };
    }

    #[allow(unused)]
    macro_rules! out { ($($arg:tt)*) => { write!(out, $($arg)*).ok(); }; }

    #[allow(unused)]
    macro_rules! outln { ($($arg:tt)*) => { writeln!(out, $($arg)*).ok(); }; }

    let n: usize = next!();
    let roads: Vec<i64> = (0..n - 1).map(|_| next!()).collect();
    let cities: Vec<i64> = (0..n - 1).map(|_| next!()).collect();

    let mut min: i64 = i64::MAX;
    let mut cost: i64 = 0;
    for (&r, &c) in roads.iter().zip(cities.iter()) {
        min = min.min(c);
        cost += min * r;
    }
    outln!("{cost}");
}

struct Scanner<'a, I: Iterator<Item = &'a str>> {
    input: I,
}

impl<'a> Scanner<'a, SplitWhitespace<'a>> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_whitespace(),
        }
    }
}

impl<'a, I: Iterator<Item = &'a str>> Scanner<'a, I> {
    #[inline(always)]
    fn next<T: FromStr>(&mut self) -> T {
        self.input
            .next()
            .expect("Input has been exhausted")
            .parse()
            .ok()
            .expect("Failed to parse")
    }
}