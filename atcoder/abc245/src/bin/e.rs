#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum E {
    C,
    B,
}
use E::*;

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let chocos: Vec<(usize, usize)> = {
        let h: Vec<usize> = sc.vec(n);
        let w: Vec<usize> = sc.vec(n);
        h.into_iter().zip(w.into_iter()).collect()
    };
    let boxes: Vec<(usize, usize)> = {
        let h: Vec<usize> = sc.vec(m);
        let w: Vec<usize> = sc.vec(m);
        h.into_iter().zip(w.into_iter()).collect()
    };
    let mut es = vec![];
    for (h, w) in chocos {
        es.push((h, w, C));
    }
    for (h, w) in boxes {
        es.push((h, w, B));
    }
    es.sort();
    es.reverse();
    trace!(&es);

    let mut wset = BTreeMultiSet::new();

    let mut ans = "Yes";
    for e in es {
        match e {
            (_, w, B) => {
                wset.insert(w);
            }
            (_, w, C) => {
                if let Some(&w) = wset.min(w..) {
                    wset.remove(w);
                } else {
                    ans = "No";
                    break;
                }
            }
        }
    }
    put!(ans);
}

pub struct BTreeMultiSet<T>(BTreeMap<T, usize>);
impl<T: Sized + Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn insert(&mut self, item: T) {
        self.0.entry(item).and_modify(|e| *e += 1).or_insert(1);
    }
    pub fn remove(&mut self, item: T) {
        if let Some(&c) = self.0.get(&item) {
            if c <= 1 {
                self.0.remove(&item);
            } else {
                self.0.entry(item).and_modify(|e| *e -= 1);
            }
        }
    }
    pub fn range<R: std::ops::RangeBounds<T>>(
        &self,
        range: R,
    ) -> std::collections::btree_map::Range<T, usize> {
        self.0.range(range)
    }
    pub fn min<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.0.range(range).next().map(|(t, _)| t)
    }
    pub fn max<R: std::ops::RangeBounds<T>>(&self, range: R) -> Option<&T> {
        self.0.range(range).next_back().map(|(t, _)| t)
    }
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
