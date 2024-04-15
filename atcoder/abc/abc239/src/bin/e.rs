#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let q: usize = sc.cin();
    let xs: Vec<u64> = sc.vec(n);

    let mut g = vec![vec![]; n];
    for _ in 1..n {
        let u = sc.usize1();
        let v = sc.usize1();
        g[u].push(v);
        g[v].push(u);
    }

    let mut qs = vec![];
    for _ in 0..q {
        let v = sc.usize1();
        let k: usize = sc.cin();
        qs.push((v, k));
    }

    // g -> tree
    let mut t = vec![vec![]; n];
    {
        let mut s = vec![0];
        let mut visited = vec![false; n];
        while let Some(u) = s.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for &v in g[u].iter() {
                if visited[v] {
                    continue;
                }
                t[u].push(v); // parent -> child
                s.push(v);
            }
        }
    }

    let ord = Topological::sort(&t);
    trace!(&ord);

    let mut memo = vec![vec![]; n];
    for &u in ord.iter().rev() {
        let mut tops = vec![];
        for &v in t[u].iter() {
            for &x in memo[v].iter() {
                tops.push(x);
            }
        }
        tops.push(Reverse(xs[u]));
        tops.sort();
        if tops.len() > 20 {
            memo[u] = tops[0..20].to_vec();
        } else {
            memo[u] = tops.to_vec();
        }
        trace!(u, &memo[u]);
    }

    for &(v, k) in qs.iter() {
        trace!((v, k), memo[v][k - 1]);
        put!(memo[v][k - 1].0);
    }
}

// @graph/directed/topological_sort
/// Graph - Directed - Topological Sort

pub struct Topological;

impl Topological {
    pub fn sort(neigh: &Vec<Vec<usize>>) -> Vec<usize> {
        let n = neigh.len();
        let mut rd = vec![vec![]; n];
        for u in 0..n {
            for &v in neigh[u].iter() {
                rd[v].push(u);
            }
        }
        let mut used = vec![false; n];
        let mut ord = vec![];
        for u in 0..n {
            Self::visit(u, &mut used, &rd, &mut ord);
        }
        ord
    }
    fn visit(u: usize, mut used: &mut Vec<bool>, rd: &Vec<Vec<usize>>, mut ord: &mut Vec<usize>) {
        if used[u] {
            return;
        }
        used[u] = true;
        for &v in rd[u].iter() {
            Self::visit(v, &mut used, &rd, &mut ord);
        }
        ord.push(u);
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
    pub fn pair<S: FromStr, T: FromStr>(&mut self) -> (S, T) {
        let x = self.cin::<S>();
        let y = self.cin::<T>();
        (x, y)
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
