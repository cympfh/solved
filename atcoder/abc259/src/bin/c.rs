#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

// @string/runlength
/// String Compression - Run-Length
pub fn runlength<A: Copy + PartialEq>(xs: &Vec<A>) -> Vec<(A, usize)> {
    let m = xs.len();
    if m == 0 {
        return vec![];
    }
    let mut count = 1;
    let mut result = vec![];
    for i in 0..m {
        if i > 0 {
            if xs[i - 1] == xs[i] {
                count += 1;
            } else {
                count = 1;
            }
        }
        if i == m - 1 || xs[i] != xs[i + 1] {
            result.push((xs[i], count));
        }
    }
    result
}

fn main() {
    let mut sc = Scanner::new();
    let s = sc.chars();
    let t = sc.chars();
    let s = runlength(&s);
    let t = runlength(&t);
    let mut ans = "No";
    if s.len() == t.len() {
        if (0..s.len())
            .all(|i| s[i] == t[i] || (s[i].0 == t[i].0 && 2 <= s[i].1 && s[i].1 <= t[i].1))
        {
            ans = "Yes";
        }
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
