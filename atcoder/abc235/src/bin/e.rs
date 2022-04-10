#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum E {
    RealEdge(usize, usize, u64),
    QueryEdge(usize, usize, u64, usize),
}
use E::*;

impl E {
    fn val(&self) -> u64 {
        match self {
            RealEdge(_, _, x) => *x,
            QueryEdge(_, _, x, _) => *x,
        }
    }
}
impl std::cmp::PartialOrd for E {
    fn partial_cmp(&self, other: &E) -> Option<std::cmp::Ordering> {
        self.val().partial_cmp(&other.val())
    }
}
impl std::cmp::Ord for E {
    fn cmp(&self, other: &E) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let q: usize = sc.cin();

    let es: Vec<(usize, usize, u64)> = (0..m)
        .map(|_| {
            let a: usize = sc.usize1();
            let b: usize = sc.usize1();
            let c: u64 = sc.cin();
            (a, b, c)
        })
        .collect();

    let qs: Vec<(usize, usize, u64, usize)> = (0..q)
        .map(|i| {
            let a: usize = sc.usize1();
            let b: usize = sc.usize1();
            let c: u64 = sc.cin();
            (a, b, c, i)
        })
        .collect();

    let mut edges = vec![];
    for &(a, b, c) in es.iter() {
        edges.push(RealEdge(a, b, c));
    }
    for &(a, b, c, i) in qs.iter() {
        edges.push(QueryEdge(a, b, c, i));
    }
    edges.sort();
    trace!(&edges);

    let mut ans = vec![false; q];

    let mut uf = UnionFind::new(n);
    for &edge in edges.iter() {
        match edge {
            RealEdge(a, b, _) => {
                uf.merge(a, b);
            }
            QueryEdge(a, b, _, i) => {
                if !uf.is_same(a, b) {
                    ans[i] = true;
                }
            }
        }
    }

    for &a in ans.iter() {
        put!(if a { "Yes" } else { "No" });
    }
}

// @set/union_find
/// Set - Union-Find
#[derive(Debug, Clone)]
pub struct UnionFind {
    data: Vec<UF>,
}

#[derive(Debug, Clone)]
enum UF {
    Root(usize),
    Child(usize),
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            data: vec![UF::Root(1); n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        match self.data[x] {
            UF::Root(_) => x,
            UF::Child(parent) => {
                let root = self.root(parent);
                self.data[x] = UF::Child(root);
                root
            }
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        match self.data[r] {
            UF::Root(size) => size,
            UF::Child(_) => 0,
        }
    }
    pub fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            let size_x = self.size(root_x);
            let size_y = self.size(root_y);
            let (i, j) = if size_x > size_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };
            self.data[i] = UF::Root(size_x + size_y);
            self.data[j] = UF::Child(i);
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
