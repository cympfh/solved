#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let a: Vec<usize> = sc.vec(n);

    if n < 4 {
        put!(solve_naiive(&a));
        return;
    }

    let x = solve_front(&a);
    let z = solve_rear(&a);
    trace!(x, z);
    let ans = x + z;
    put!(ans);
}

fn solve_naiive(a: &Vec<usize>) -> u128 {
    let n = a.len();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut left = i;
            let mut right = j;
            while left < right {
                if a[left] != a[right] {
                    ans += 1
                }
                left += 1;
                right -= 1;
            }
        }
    }
    ans
}

// a.len()/2 前半だけ解く
fn solve_front(a: &Vec<usize>) -> u128 {
    trace!("solve_front", &a);
    let n = a.len();
    let mut chars = BTreeMultiSet::new();
    for &c in a.iter() {
        chars.insert(c);
    }
    chars.remove(a[0]);
    let mut koken = vec![0; 200_200];
    let mut koken_sum = 0;
    let mut ans = 0_u128;
    for i in 0..(n + 1) / 2 {
        let c = a[i];
        let m = chars.len() - chars.get(&c).unwrap_or(0);
        ans += m as u128 * (i + 1) as u128;
        ans += koken_sum - koken[c];
        trace!((i, c), (m * (i + 1)), (koken_sum - koken[c]));
        chars.remove(a[i + 1]);
        chars.remove(a[a.len() - 1 - i]);
        let last = a[a.len() - 1 - i];
        koken[last] += (i + 1) as u128;
        koken_sum += (i + 1) as u128;
    }
    ans
}

// a.len()/2 後半だけ解く
fn solve_rear(a: &Vec<usize>) -> u128 {
    trace!("solve_rear", &a);
    let n = a.len();
    let mut koken = vec![0; 200_200];
    let mut koken_sum = 0;
    let mut ans = 0_u128;
    for i in 0..n / 2 {
        let c = a[n - 1 - i];
        ans += koken_sum - koken[c];
        trace!((i, c), (koken_sum - koken[c]));
        let last = a[a.len() - 1 - i];
        koken[last] += (i + 1) as u128;
        koken_sum += (i + 1) as u128;
    }
    ans
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
