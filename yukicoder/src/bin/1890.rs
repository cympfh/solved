#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

/// 最初の m 項の和
fn sum(m: usize, xs: &Vec<u64>) -> u64 {
    let mut m = m as u64;
    let mut sum = 0;
    for &x in xs.iter() {
        if m > x {
            m -= x;
            sum += (1 + x) * x / 2;
        } else {
            sum += (1 + m) * m / 2;

            break;
        }
    }
    sum
}

trait CompleteIdx: Copy {
    fn mid(self, other: Self) -> Self;
}
macro_rules! completeidx {
    ( $type:ty, mid($self:ident, $other:ident) = $code:block ) => {
        impl CompleteIdx for $type {
            fn mid($self, $other: Self) -> Self { $code }
        }
    };
}
completeidx! { usize, mid(self, other) = { (self + other) / 2 }}
completeidx! { u128, mid(self, other) = { (self + other) / 2 }}
completeidx! { u64, mid(self, other) = { (self + other) / 2 }}
completeidx! { f64, mid(self, other) = { (self + other) / 2.0 }}

fn lowerbound<T: CompleteIdx>(r: std::ops::Range<T>, cond: &dyn Fn(T) -> bool) -> Option<T> {
    if !cond(r.end) {
        return None;
    }
    if cond(r.start) {
        return Some(r.start);
    }
    let mut left = r.start;
    let mut right = r.end;
    for _ in 0..100 {
        let mid = T::mid(left, right);
        if cond(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    Some(right)
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let q: usize = sc.cin();
    let xs: Vec<u64> = sc.vec(n);
    for _ in 0..q {
        let s: u64 = sc.cin();
        let right = 200_00000;
        let left = 0;
        if let Some(m) = lowerbound(left..right, &|i| sum(i, &xs) >= s) {
            put!(m);
        } else {
            put!(-1);
        }
    }
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
