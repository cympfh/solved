#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let mut d = ndarray![0; n, n];
    for i in 0..n {
        for j in i + 1..n {
            let x: i64 = sc.cin();
            d[i][j] = x;
            d[j][i] = x;
        }
    }
    fn dfs(n: usize, visited: BitSet, d: &Vec<Vec<i64>>) -> i64 {
        let mut p = None;
        let mut qs = vec![];
        for i in 0..n {
            if !visited.contains(i) {
                if p == None {
                    p = Some(i);
                } else {
                    qs.push(i);
                }
            }
        }
        match (p, qs) {
            (Some(p), qs) if !qs.is_empty() => {
                let mut mx = 0;
                for &q in qs.iter() {
                    let mut v = visited.clone();
                    v.insert(p);
                    v.insert(q);
                    mx = max!(mx, dfs(n, v, &d) + d[p][q]);
                }
                mx
            }
            _ => 0,
        }
    }
    let mut ans = dfs(n, BitSet::new(), &d);
    if n % 2 == 1 {
        for ig in 0..n {
            ans = max!(ans, dfs(n, BitSet::from(1 << ig), &d));
        }
    }
    put!(ans);
}

// @set/bitset
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BitSet(pub u128);

impl BitSet {
    pub fn new() -> Self {
        BitSet(0)
    }
    pub fn from(data: u128) -> Self {
        BitSet(data)
    }
    pub fn id(&self) -> usize {
        self.0 as usize
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn contains(&self, i: usize) -> bool {
        (self.0 & (1 << i)) != 0
    }
    pub fn insert(&mut self, i: usize) {
        self.0 |= 1 << i;
    }
    /// subset <: self
    pub fn is_supset_of(&self, subset: &BitSet) -> bool {
        subset.0 & !self.0 == 0
    }
    /// self <: subset
    pub fn is_subset_of(&self, supset: &BitSet) -> bool {
        self.0 & !supset.0 == 0
    }
    pub fn iter(&self) -> BitSetIter {
        BitSetIter(*self, 0)
    }
    pub fn to_vec(&self) -> Vec<usize> {
        self.iter().collect()
    }
}

impl std::ops::BitAnd<usize> for BitSet {
    type Output = bool;
    fn bitand(self, i: usize) -> bool {
        self.contains(i)
    }
}

impl std::ops::BitOr<usize> for BitSet {
    type Output = Self;
    fn bitor(self, i: usize) -> Self {
        BitSet(self.0 | (1 << i))
    }
}

impl std::ops::BitOrAssign<usize> for BitSet {
    fn bitor_assign(&mut self, i: usize) {
        self.insert(i);
    }
}

pub struct BitSetIter(BitSet, usize);
impl std::iter::Iterator for BitSetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let data = (self.0).0;
        let mut cur = self.1;
        while cur < 128 {
            if (data >> cur) & 1 == 1 {
                self.1 = cur + 1;
                return Some(cur);
            }
            cur += 1;
        }
        self.1 = 129;
        None
    }
}

/// Iter of all subsets for {0, 1, .., (n-1)}
pub struct Subsets(u128, u128);
impl Subsets {
    pub fn new(n: usize) -> Self {
        Self(1 << n, 0)
    }
}
impl std::iter::Iterator for Subsets {
    type Item = BitSet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < self.0 {
            self.1 += 1;
            Some(BitSet::from(self.1 - 1))
        } else {
            None
        }
    }
}

/// Iter of all subsets for all subsets for {0, 1, .., (n-1)}
pub struct SubSubsets(u128, u128, u128);
impl SubSubsets {
    pub fn new(n: usize) -> Self {
        Self(1 << n, 0, 0)
    }
}
impl std::iter::Iterator for SubSubsets {
    type Item = (BitSet, BitSet);
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0 {
            None
        } else if self.2 == 0 {
            self.1 += 1;
            self.2 = self.1;
            Some((BitSet::from(self.1 - 1), BitSet::from(0)))
        } else {
            let set = BitSet::from(self.2);
            self.2 = (self.2 - 1) & self.1;
            Some((BitSet::from(self.1), set))
        }
    }
}

#[macro_export]
macro_rules! bitset {
    ($($elems:expr),* $(,)*) => {{
        #[allow(unused_mut)]
        let mut bs = BitSet::new();
        $(
            bs |= $elems;
        )*
        bs
    }}
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
