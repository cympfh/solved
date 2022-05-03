#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn battle(x: char, y: char) -> (i32, i32) {
    match (x, y) {
        ('G', 'C') => (1, 0),
        ('G', 'P') => (0, 1),
        ('C', 'G') => (0, 1),
        ('C', 'P') => (1, 0),
        ('P', 'G') => (1, 0),
        ('P', 'C') => (0, 1),
        _ => (0, 0),
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let a: Vec<Vec<char>> = (0..2 * n).map(|_| sc.chars()).collect();
    let mut ws = vec![0_i32; 2 * n];
    for j in 0..m {
        let mut is: Vec<(usize, i32)> = ws.iter().cloned().enumerate().collect();
        is.sort_by_key(|&(id, cx)| (Reverse(cx), id));
        for i in 0..n {
            let id1 = is[i * 2].0;
            let id2 = is[i * 2 + 1].0;
            let (r1, r2) = battle(a[id1][j], a[id2][j]);
            ws[id1] += r1;
            ws[id2] += r2;
        }
    }
    let mut is: Vec<(usize, i32)> = ws.iter().cloned().enumerate().collect();
    is.sort_by_key(|&(id, cx)| (Reverse(cx), id));
    let ans: Vec<usize> = is.into_iter().map(|(id, _)| id + 1).collect();
    put!(..ans);
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;

struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }
    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
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
