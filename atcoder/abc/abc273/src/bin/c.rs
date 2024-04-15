#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

// @algorithm/binary_search
/// Algorithm - Binary Search (lowerbound)
pub trait CompleteIdx: Copy {
    fn mid(self, other: Self) -> Self;
}
#[macro_export]
macro_rules! completeidx {
    ( $type:ty, mid($self:ident, $other:ident) = $code:block ) => {
        impl CompleteIdx for $type {
            fn mid($self, $other: Self) -> Self { $code }
        }
    };
}
completeidx! { usize, mid(self, other) = { (self + other) / 2 }}
completeidx! { u128, mid(self, other) = { (self + other) / 2 }}
completeidx! { i128, mid(self, other) = { (self + other) / 2 }}
completeidx! { u64, mid(self, other) = { (self + other) / 2 }}
completeidx! { i64, mid(self, other) = { (self + other) / 2 }}
completeidx! { f64, mid(self, other) = { (self + other) / 2.0 }}

pub fn lowerbound<T: CompleteIdx>(r: std::ops::Range<T>, cond: &dyn Fn(T) -> bool) -> Option<T> {
    if cond(r.start) {
        return Some(r.start);
    }
    // TODO(from 1.47.0)
    // if r.is_empty() { return None }
    let mut left = r.start;
    let mut right = r.end;
    let mut ok = false;
    for _ in 0..100 {
        let mid = T::mid(left, right);
        if cond(mid) {
            right = mid;
            ok = true;
        } else {
            left = mid;
        }
    }
    if ok {
        Some(right)
    } else {
        None
    }
}

// x 以上の個数
fn count(x: i64, xs: &Vec<i64>) -> usize {
    let n = xs.len();
    if let Some(i) = lowerbound(0..n, &|i| xs[i] > x) {
        n - i
    } else {
        0
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let xs: Vec<i64> = sc.vec(n);
    let mut xset: Vec<i64> = xs
        .iter()
        .cloned()
        .collect::<BTreeSet<i64>>()
        .iter()
        .cloned()
        .collect();
    xset.sort();
    let mut ans = vec![0; n + 1];
    for &x in xs.iter() {
        trace!(x, &xs, count(x, &xs));
        ans[count(x, &xset)] += 1;
    }
    for a in &ans[..n] {
        put!(a);
    }
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;
#[derive(Default)]
pub struct Scanner {
    buffer: VecDeque<String>,
}
impl Scanner {
    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line);
            self.buffer = line.split_whitespace().map(|w| String::from(w)).collect();
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
        (self.cin::<S>(), self.cin::<T>())
    }
}
fn flush() {
    io::stdout().flush().unwrap();
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
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

// }}}
