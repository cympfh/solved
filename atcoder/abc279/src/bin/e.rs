#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let a: Vec<usize> = sc.vec(m);

    let ts: Vec<Transposition> = a.into_iter().map(|i| perm!(trans [i - 1, i])).collect();

    let mut head = SymmetricGroup::one(n);
    let mut tail = SymmetricGroup::one(n);
    for i in 1..m {
        tail.swap(&ts[i]);
    }

    put!(tail.apply(head.apply(0)) + 1);
    for k in 1..m {
        head.swap(&ts[k - 1]);
        tail.unswap(&ts[k]);
        put!(tail.apply(head.apply(0)) + 1);
    }
}

// @algebra/symmetric
/// Algebra - Symmetric Group (対称群), transposition (互換)
/// TODO: AtCoder の Rust がバージョンアップしてくれたら const generics で書き換える
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymmetricGroup {
    n: usize,
    p: Vec<usize>,
    pinv: Vec<usize>,
}
impl SymmetricGroup {
    pub fn new(p: Vec<usize>) -> Self {
        let n = p.len();
        let mut pinv = vec![0; n];
        for i in 0..n {
            pinv[p[i]] = i;
        }
        Self { n, p, pinv }
    }
    pub fn one(n: usize) -> Self {
        let p = (0..n).collect();
        SymmetricGroup::new(p)
    }
    pub fn apply(&self, i: usize) -> usize {
        self.p[i]
    }
    /// Inverse, O(N)
    pub fn inv(&self) -> Self {
        Self {
            n: self.n,
            p: self.pinv.clone(),
            pinv: self.p.clone(),
        }
    }
    /// SymmetricGroup * SymmetricGroup, O(N)
    pub fn mul(&self, other: &SymmetricGroup) -> Self {
        let p = (0..self.n).map(|i| other.p[self.p[i]]).collect();
        SymmetricGroup::new(p)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transposition {
    x: usize,
    y: usize,
}
impl Transposition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    /// Inverse, O(1)
    pub fn inv(&self) -> Self {
        self.clone()
    }
    /// Cast to SymmetricGroup
    pub fn into_n(&self, n: usize) -> SymmetricGroup {
        let mut p: Vec<usize> = (0..n).collect();
        p[self.x] = self.y;
        p[self.y] = self.x;
        SymmetricGroup::new(p)
    }
}

impl SymmetricGroup {
    /// self=SymmetricGroup * Transposition, O(1)
    pub fn swap(&mut self, trans: &Transposition) {
        let i = self.pinv[trans.x];
        let j = self.pinv[trans.y];
        self.p.swap(i, j);
        self.pinv.swap(trans.x, trans.y);
    }
    /// Transposition * self=SymmetricGroup, O(1)
    pub fn unswap(&mut self, trans: &Transposition) {
        let i = self.p[trans.x];
        let j = self.p[trans.y];
        self.p.swap(trans.x, trans.y);
        self.pinv.swap(i, j);
    }
}

#[macro_export]
macro_rules! perm {
    (sym [ $( $val:expr ),* $(,)? ]) => {
        SymmetricGroup::new(vec![ $($val),* ])
    };
    (trans [$x:expr, $y:expr $(,)?]) => {
        Transposition::new($x, $y)
    };
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
