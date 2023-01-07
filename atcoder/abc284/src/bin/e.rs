#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.usize1();
        let v = sc.usize1();
        g[u].push(v);
        g[v].push(u);
    }
    let mut dfs = DFS::default();
    let mut visited = HashSet::new();
    dfs.push(&0);
    let mut ans = 0;
    while let Some(next) = dfs.pop() {
        match next {
            DfsOrd::Pre(u) => {
                visited.insert(u);
                ans += 1;
                if ans >= 1_000_000 {
                    put!(1_000_000);
                    return;
                }
                for &v in g[u].iter() {
                    if visited.contains(&v) {
                        continue;
                    }
                    dfs.push(&v);
                }
            }
            DfsOrd::Post(u) => {
                visited.remove(&u);
            }
        }
    }
    put!(ans);
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
