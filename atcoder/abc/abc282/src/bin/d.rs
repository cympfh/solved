#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let mut g = vec![vec![]; n];
    let es: Vec<_> = (0..m)
        .map(|_| {
            let u = sc.usize1();
            let v = sc.usize1();
            (u, v)
        })
        .collect();
    for &(u, v) in es.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    trace!(&g);
    let mut failed = false;
    let mut gid = vec![0; n];
    let mut id = 1;
    for u in 0..n {
        if gid[u] > 0 {
            continue;
        }
        trace!(u);
        gid[u] = id;
        let mut s = vec![u];
        while let Some(u) = s.pop() {
            for &v in g[u].iter() {
                let neigh = if gid[u] % 2 == 0 {
                    gid[u] - 1
                } else {
                    gid[u] + 1
                };
                if gid[v] > 0 {
                    if gid[v] != neigh {
                        failed = true;
                        break;
                    }
                    continue;
                }
                gid[v] = neigh;
                s.push(v);
            }
        }
        id += 2;
    }
    trace!(&gid);
    if failed {
        trace!(failed);
        put!(0);
        return;
    }
    let mut size_by_id = vec![0_i128; n + n + 2];
    for u in 0..n {
        size_by_id[gid[u]] += 1;
    }
    trace!(&size_by_id);
    let mut ans = 0_i128;
    for u in 0..n {
        ans += n as i128 - size_by_id[gid[u]] - g[u].len() as i128;
    }
    put!(ans / 2);
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
