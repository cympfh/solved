#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let k: usize = sc.cin();
    let a: Vec<i64> = sc.vec(n);

    let mut e = E::new(k);
    for i in 0..m {
        e.push(a[i]);
    }
    trace!(&e);
    put!(e.sumk());
    for i in m..n {
        e.push(a[i]);
        e.pop(a[i - m]);
        trace!(&e);
        put!(e.sumk());
    }
}

#[derive(Debug, Clone)]
struct E {
    topk: BTreeMultiSet<i64>,
    rest: BTreeMultiSet<i64>,
    k: usize,
    sumk: i64,
}

impl E {
    fn new(k: usize) -> Self {
        let topk = BTreeMultiSet::new();
        let rest = BTreeMultiSet::new();
        E {
            topk,
            rest,
            k,
            sumk: 0,
        }
    }
    fn pop(&mut self, x: i64) {
        let y = *self.rest.min(0..).unwrap();
        if x < y {
            self.topk.remove(x);
            self.topk.insert(y);
            self.rest.remove(y);
            self.sumk = self.sumk - x + y;
        } else {
            self.rest.remove(x);
        }
    }
    fn push(&mut self, x: i64) {
        if self.topk.len() < self.k {
            self.topk.insert(x);
            self.sumk += x;
        } else {
            let y = *self.topk.max(0..).unwrap();
            if x < y {
                self.topk.insert(x);
                self.topk.remove(y);
                self.rest.insert(y);
                self.sumk = self.sumk + x - y;
            } else {
                self.rest.insert(x);
            }
        }
    }
    fn sumk(&self) -> i64 {
        self.sumk
    }
}

// @collections/btreemultiset
/// collections - BTree MultiSet
#[derive(Debug, Clone)]
pub struct BTreeMultiSet<T> {
    data: std::collections::BTreeMap<T, usize>,
    size: usize,
}
impl<T: Sized + Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        let data = std::collections::BTreeMap::new();
        let size = 0;
        Self { data, size }
    }
    pub fn insert(&mut self, item: T) {
        self.data.entry(item).and_modify(|e| *e += 1).or_insert(1);
        self.size += 1;
    }
    pub fn get(&self, item: &T) -> Option<usize> {
        self.data.get(item).cloned()
    }
    pub fn remove(&mut self, item: T) {
        if let Some(&c) = self.data.get(&item) {
            if c <= 1 {
                self.data.remove(&item);
            } else {
                self.data.entry(item).and_modify(|e| *e -= 1);
            }
            self.size -= 1;
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn range<R: std::ops::RangeBounds<T>>(
        &self,
        range: R,
    ) -> std::collections::btree_map::Range<T, usize> {
        self.data.range(range)
    }
    pub fn min<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.data.range(range).next().map(|(t, _)| t)
    }
    pub fn max<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.data.range(range).next_back().map(|(t, _)| t)
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
