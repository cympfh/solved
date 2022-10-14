#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let s: usize = sc.cin();
    let cards: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let a: usize = sc.cin();
            let b: usize = sc.cin();
            (a, b)
        })
        .collect();

    // dp[i][t] = cards[0..i] を使って t を作れる?
    //            0 ... 作れない
    //            1 ... 表面で作れる
    //            2 ... 裏面で作れる
    let mut dp = ndarray![0; n+1,s+1];
    dp[0][0] = 1;
    for i in 0..n {
        for t in 0..s {
            if dp[i][t] == 0 {
                continue;
            }
            if t + cards[i].0 <= s {
                dp[i + 1][t + cards[i].0] = 1;
            }
            if t + cards[i].1 <= s {
                dp[i + 1][t + cards[i].1] = 2;
            }
        }
    }
    trace!(dp[n][s]);
    if dp[n][s] == 0 {
        put!(#No);
        return;
    }
    put!(#Yes);
    let mut sum = s;
    let mut ans = vec![];
    for i in (0..n).rev() {
        ans.push(if dp[i + 1][sum] == 1 { 'H' } else { 'T' });
        sum -= if dp[i + 1][sum] == 1 {
            cards[i].0
        } else {
            cards[i].1
        };
    }
    trace!(&ans);
    put!(ans.into_iter().rev().collect::<String>());
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
