#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let mut ms = vec![];
    ms.push((0, 0, 0));
    for _ in 0..n {
        let t: i64 = sc.cin();
        let x: i64 = sc.cin();
        let size: u64 = sc.cin();
        ms.push((t, x, size));
    }

    let mut able = vec![false; n + 1];
    able[0] = true;
    let mut dp = vec![0; n + 1]; // dp[i] = The sum of sizes, on catching i-th
    let mut dpmax = vec![0; n + 2]; // dpmax[i] = max(dp[0..i]);

    for i in 1..=n {
        let (ti, xi, sizei) = ms[i];
        trace!(i, ms[i]);
        for k in 1..6 {
            if i < k {
                break;
            }
            let (tj, xj, _) = ms[i - k];
            if able[i - k] && xi - ti + tj <= xj && xj <= xi + ti - tj {
                trace!(i - k, i);
                dp[i] = max!(dp[i], dp[i - k] + sizei);
                able[i] = true;
            }
        }
        if i >= 5 {
            dp[i] = max!(dp[i], dpmax[i - 5] + sizei);
            able[i] = true;
        }
        dpmax[i + 1] = max!(dpmax[i], dp[i]);
        trace!(i, dp[i], dpmax[i + 1]);
    }
    trace!(&dp);
    trace!(&dpmax);
    put!(dpmax[n + 1]);
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
