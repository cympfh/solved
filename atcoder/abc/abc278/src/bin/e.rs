#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.cin();
    let w: usize = sc.cin();
    let _n: usize = sc.cin();
    let dh: usize = sc.cin();
    let dw: usize = sc.cin();
    let a: Vec<Vec<usize>> = (0..h).map(|_| sc.vec(w)).collect();

    let mut count = BTreeMultiSet::new();
    for line in a.iter() {
        for &x in line.iter() {
            count.insert(x);
        }
    }
    trace!(count.len());
    let original = count.clone();

    for i in 0..=h - dh {
        count = original.clone();
        for ii in 0..dh {
            for jj in 0..dw {
                count.remove(a[i + ii][jj]);
            }
        }
        print!("{}", count.len());
        for j in 0..w - dw {
            for ii in 0..dh {
                count.insert(a[i + ii][j]);
            }
            for ii in 0..dh {
                count.remove(a[i + ii][j + dw]);
            }
            print!(" {}", count.len());
        }
        println!();
    }
}

// @collections/btreemultiset
/// collections - BTree MultiSet
#[derive(Debug, Clone)]
pub struct BTreeMultiSet<T>(std::collections::BTreeMap<T, usize>);
impl<T: Sized + Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn insert(&mut self, item: T) {
        self.0.entry(item).and_modify(|e| *e += 1).or_insert(1);
    }
    pub fn get(&self, item: &T) -> Option<usize> {
        self.0.get(item).cloned()
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
    (yes) => {println!("Yes")}; (no) => {println!("No")};
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
    ($x:expr;) => {
        $x
    };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

// }}}
