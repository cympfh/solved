#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let s = sc.chars();
    let w: Vec<i64> = sc.vec(n);

    let mut uniq: BTreeSet<i64> = w.clone().into_iter().collect();
    uniq.insert(0);
    uniq.insert(1_000_000_100);

    let mut children = vec![];
    let mut adult = vec![];
    for i in 0..n {
        if s[i] == '0' {
            children.push(w[i]);
        } else {
            adult.push(w[i]);
        }
    }
    children.sort();
    adult.sort();
    trace!(&children);
    trace!(&adult);

    let mut ans = 0;
    for &w in uniq.iter() {
        let n1 = count(&children, 0..w);
        let n2 = count(&adult, w..1_000_000_100);
        trace!(w, (n1, n2), n1 + n2);
        ans = max!(ans, n1 + n2);
    }
    put!(ans);
}

// @algorithm/binary_search_count
// @algorithm/binary_search
/// Algorithm - Binary Search (lowerbound)
pub trait Complete: Copy + PartialEq + PartialOrd {
    fn mid(self, other: Self) -> Self;
}
#[macro_export]
macro_rules! complete {
    ( $type:ty, mid($self:ident, $other:ident) = $code:block ) => {
        impl Complete for $type {
            fn mid($self, $other: Self) -> Self { $code }
        }
    };
}
complete! { usize, mid(self, other) = { (self + other) / 2 }}
complete! { u128, mid(self, other) = { (self + other) / 2 }}
complete! { i128, mid(self, other) = { (self + other) / 2 }}
complete! { u64, mid(self, other) = { (self + other) / 2 }}
complete! { i64, mid(self, other) = { (self + other) / 2 }}
complete! { f64, mid(self, other) = { (self + other) / 2.0 }}

/// Find a lowerbound for the condition
/// the condition has monotone: false -> true
pub fn lowerbound<T: Complete>(r: std::ops::Range<T>, cond: &dyn Fn(T) -> bool) -> Option<T> {
    if r.is_empty() {
        return None;
    }
    if cond(r.start) {
        return Some(r.start);
    }
    let mut left = r.start;
    let mut right = r.end;
    let mut ok = false;
    for _ in 0..100 {
        let mid = T::mid(left, right);
        if cond(mid) {
            right = mid;
            ok = true;
        } else {
            left = mid;
        }
    }
    if ok {
        Some(right)
    } else {
        None
    }
}

/// num of elements in the range
pub fn count<X: Complete>(xs: &Vec<X>, range: std::ops::Range<X>) -> usize {
    let n = xs.len();
    if n == 0 {
        return 0;
    }
    let rightout = lowerbound(0..n, &|i| xs[i] >= range.end)
        .map(|i| n - i)
        .unwrap_or(0);
    let rightin = lowerbound(0..n, &|i| xs[i] >= range.start)
        .map(|i| n - i)
        .unwrap_or(rightout);
    rightin - rightout
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
    ($x:expr, $($ys:expr),* $(,)*) => {{
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
    ($x:expr, $($ys:expr),* $(,)*) => {{
        let t = max!($($ys),*);
        if $x > t { $x } else { t }
    }}
}
#[macro_export]
macro_rules! trace {
    (# $a:ident $(,)? $(;)? $($xs:expr),* $(,)? ) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}] {} = {:?}", stringify!($a), stringify!($($xs),*), ($($xs),*))
    };
    ($($xs:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($($xs),*), ($($xs),*))
    };
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
/// Array-indexing by i64.
/// (Vec<Vec<..<T>..>>; i64, i64, ..) => Option<T>
#[macro_export]
macro_rules! at {
    ($s:expr;) => { Some($s) };
    ($s:expr; $idx:expr $(,$args:expr)* $(,)?) => {
        if 0 <= $idx {
            let idx_usize = $idx as usize;
            if idx_usize < $s.len() {
                at!($s[idx_usize]; $($args),*)
            } else {
                None
            }
        } else {
            None
        }
    }
}
// }}}
