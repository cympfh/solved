#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let x: Vec<u64> = sc.vec(n);
    let y: Vec<u64> = sc.vec(n);
    let rs: Vec<_> = (0..m)
        .map(|_| {
            let a = sc.usize1();
            let b = sc.usize1();
            let z: u64 = sc.cin();
            (a, b, z)
        })
        .collect();

    let airport = n;
    let port = n + 1;
    trace!(airport, port);

    // with airport & port
    let mut edges = vec![];
    for i in 0..n {
        edges.push((i, airport, x[i]));
        edges.push((i, port, y[i]));
    }
    for &(a, b, z) in rs.iter() {
        edges.push((a, b, z));
    }
    edges.sort_by_key(|&item| item.2);
    let mut uf = UnionFind::new(n + 2);
    let mut cost = 0;
    for &(a, b, z) in edges.iter() {
        if uf.is_same(a, b) {
            continue;
        };
        uf.merge(a, b);
        cost += z;
    }
    trace!("airport&port", cost);

    let mut ans = cost;

    // with airport
    let mut edges = vec![];
    for i in 0..n {
        edges.push((i, airport, x[i]));
    }
    for &(a, b, z) in rs.iter() {
        edges.push((a, b, z));
    }
    edges.sort_by_key(|&item| item.2);
    let mut uf = UnionFind::new(n + 1);
    let mut cost = 0;
    for &(a, b, z) in edges.iter() {
        if uf.is_same(a, b) {
            continue;
        };
        uf.merge(a, b);
        cost += z;
    }
    trace!("airport", cost);
    ans = min!(ans, cost);

    // with port
    let mut edges = vec![];
    for i in 0..n {
        edges.push((i, airport, y[i]));
    }
    for &(a, b, z) in rs.iter() {
        edges.push((a, b, z));
    }
    edges.sort_by_key(|&item| item.2);
    let mut uf = UnionFind::new(n + 1);
    let mut cost = 0;
    for &(a, b, z) in edges.iter() {
        if uf.is_same(a, b) {
            continue;
        };
        uf.merge(a, b);
        cost += z;
    }
    trace!("port", cost);
    ans = min!(ans, cost);

    // wo airport,port
    let mut edges = vec![];
    for &(a, b, z) in rs.iter() {
        edges.push((a, b, z));
    }
    edges.sort_by_key(|&item| item.2);
    let mut uf = UnionFind::new(n);
    let mut cost = 0;
    for &(a, b, z) in edges.iter() {
        if uf.is_same(a, b) {
            continue;
        };
        uf.merge(a, b);
        cost += z;
    }
    {
        let mut ok = true;
        for i in 1..n {
            if !uf.is_same(0, i) {
                ok = false;
            }
        }
        if ok {
            trace!("wo", cost);
            ans = min!(ans, cost);
        }
    }

    put!(ans);
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

use btree_set::Union;
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
