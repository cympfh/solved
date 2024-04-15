#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

// @num/prime/factorize
/// Number - Prime Factorization
pub fn factorize(n: u64) -> Vec<(u64, usize)> {
    let mut m = n;
    let mut r = vec![];
    for x in 2..=n {
        if m == 1 {
            break;
        }
        if x * x > n {
            r.push((m, 1));
            break;
        }
        if n % x != 0 {
            continue;
        }
        let mut c = 0;
        while m % x == 0 {
            m /= x;
            c += 1;
        }
        if c > 0 {
            r.push((x, c));
        }
    }
    r
}

fn main() {
    let mut sc = Scanner::new();
    let n: u64 = sc.cin();
    trace!(n);
    let mut ans = 0;
    for a in 1..=n {
        let mut b = 1;
        for (p, c) in factorize(a) {
            if c % 2 == 1 {
                b *= p;
            }
        }
        let mut left = 1;
        let mut right = n + 1;
        for _ in 0..100 {
            let mid = (left + right) / 2;
            if b * mid * mid <= n {
                left = mid;
            } else {
                right = mid;
            }
        }
        trace!(a, b, (left, right));
        ans += left;
    }
    put!(ans);
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn pair<S: FromStr, T: FromStr>(&mut self) -> (S, T) {
        let x = self.cin::<S>();
        let y = self.cin::<T>();
        (x, y)
    }
}
fn flush() {
    std::io::stdout().flush().unwrap();
}
#[macro_export]
macro_rules! min {
    (.. $x:expr) => {{
        let mut it = $x.iter();
        it.next().map(|z| it.fold(z, |x, y| min!(x, y)))
    }};
    ($x:expr) => ($x);
    ($x:expr, $($ys:expr),*) => {{
        let t = min!($($ys),*);
        if $x < t { $x } else { t }
    }}
}
#[macro_export]
macro_rules! max {
    (.. $x:expr) => {{
        let mut it = $x.iter();
        it.next().map(|z| it.fold(z, |x, y| max!(x, y)))
    }};
    ($x:expr) => ($x);
    ($x:expr, $($ys:expr),*) => {{
        let t = max!($($ys),*);
        if $x > t { $x } else { t }
    }}
}
#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}
#[macro_export]
macro_rules! put {
    (yes) => {println!("Yes")}; (no) => {println!("No")};
    (# $a:ident) => {println!("{}", stringify!($a))};
    (.. $x:expr) => {{
        let mut it = $x.iter();
        if let Some(x) = it.next() { print!("{}", x); }
        for x in it { print!(" {}", x); }
        println!("");
    }};
    ($x:expr) => { println!("{}", $x) };
    ($x:expr, $($xs:expr),*) => { print!("{} ", $x); put!($($xs),*) }
}
#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => {
        $x
    };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

// }}}
