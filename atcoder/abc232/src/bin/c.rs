#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.usize1();
        let v = sc.usize1();
        g[u].push(v);
        g[v].push(u);
    }
    for i in 0..n {
        g[i].sort();
    }
    trace!(&g);
    let mut h = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.usize1();
        let v = sc.usize1();
        h[u].push(v);
        h[v].push(u);
    }
    let mut ans = "No";
    for p in Permutation::new(n) {
        let h2 = perm(&h, p);
        trace!(&h2);
        if eq(&g, &h2) {
            ans = "Yes";
        }
    }
    put!(ans);
}

fn perm(g: &Vec<Vec<usize>>, p: Vec<usize>) -> Vec<Vec<usize>> {
    let n = g.len();
    let mut h = vec![vec![]; n];
    // i in g == p[i] in h
    for i in 0..n {
        for &v in g[i].iter() {
            h[p[i]].push(p[v]);
        }
    }
    for i in 0..n {
        h[i].sort();
    }
    h
}

fn eq(g: &Vec<Vec<usize>>, h: &Vec<Vec<usize>>) -> bool {
    g == h
}

// @num/iter/perm
/// Number - Iterator - Factorial Permutation (n!)
pub struct Permutation {
    n: usize,
    idx: usize,
    done: bool,
}
impl Permutation {
    pub fn new(n: usize) -> Permutation {
        Permutation {
            n,
            idx: 0,
            done: false,
        }
    }
    pub fn from(mut perm: Vec<usize>) -> Permutation {
        let n = perm.len();
        let mut idx = 0;
        let mut fact: usize = (1..n).product();
        for i in 0..n {
            if i > 0 {
                fact /= n - i;
            }
            idx += perm[i] * fact;
            for j in i + 1..n {
                if perm[j] > perm[i] {
                    perm[j] -= 1;
                }
            }
        }
        Permutation {
            n,
            idx,
            done: false,
        }
    }
    pub fn to_vec(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.n == 0 {
            self.done = true;
            return Some(vec![]);
        }
        let mut r = vec![0; self.n];
        let mut idx = self.idx;
        for k in 1..self.n {
            r[k] = idx % (k + 1);
            idx /= k + 1;
        }
        if idx > 0 {
            self.done = true;
            return None;
        }
        r.reverse();
        let mut b = vec![true; self.n];
        b[r[0]] = false;
        for k in 1..self.n {
            let mut count = 0;
            for j in 0..self.n {
                if b[j] {
                    if count == r[k] {
                        r[k] = j;
                        b[j] = false;
                        break;
                    }
                    count += 1;
                }
            }
        }
        Some(r)
    }
}
impl Iterator for Permutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let r = self.to_vec();
        self.idx += 1;
        r
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
