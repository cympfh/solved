#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let h: usize = sc.cin();
    let w: usize = sc.cin();
    let a: Vec<Vec<u64>> = (0..h).map(|_| sc.vec(w)).collect();

    let mut dfs = DFS::default();
    let mut viewed = vec![];
    dfs.push(&(0, 0));
    let mut ans = 0;
    while let Some(ord) = dfs.pop() {
        match ord {
            DfsOrd::Pre(u) => {
                viewed.push(a[u.0][u.1]);
                if u == (h - 1, w - 1) {
                    trace!(&viewed);
                    if unique(&viewed) {
                        ans += 1;
                    }
                }
                if u.0 < h - 1 {
                    dfs.push(&(u.0 + 1, u.1));
                }
                if u.1 < w - 1 {
                    dfs.push(&(u.0, u.1 + 1));
                }
            }
            DfsOrd::Post(_) => {
                let _ = viewed.pop();
            }
        }
    }
    put!(ans);
}

fn unique(xs: &Vec<u64>) -> bool {
    let ys: BTreeSet<_> = xs.iter().cloned().collect();
    xs.len() == ys.len()
}

// @algorithm/dfs
#[derive(Debug, Default)]
pub struct DFS<X> {
    pub stack: Vec<DfsOrd<X>>,
}
impl<X: Clone> DFS<X> {
    pub fn push(&mut self, x: &X) {
        self.stack.push(DfsOrd::Post(x.clone()));
        self.stack.push(DfsOrd::Pre(x.clone()));
    }
    pub fn pop(&mut self) -> Option<DfsOrd<X>> {
        self.stack.pop()
    }
}
#[derive(Debug)]
pub enum DfsOrd<X> {
    Pre(X),
    Post(X),
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
