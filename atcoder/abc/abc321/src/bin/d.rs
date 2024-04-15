#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let p: i64 = sc.cin();
    let mut xs: Vec<i64> = sc.vec(n);
    let mut ys: Vec<i64> = sc.vec(m);
    xs.sort();
    ys.sort();
    let ys_cum = Cumsum1d::new(&ys);

    let mut ans = 0;
    for &x in xs.iter() {
        if x < p {
            let num = count(&ys, 0..p - x);
            ans += x * num as i64 + ys_cum.sum(0..num);
            ans += p * (m - num) as i64;
        } else {
            ans += p * m as i64;
        }
    }

    put!(ans);
}

// @algorithm/binary_search_count
// @algorithm/binary_search
/// Algorithm - Binary Search (lowerbound)
pub trait CompleteIdx: Copy {
    fn mid(self, other: Self) -> Self;
}
#[macro_export]
macro_rules! completeidx {
    ( $type:ty, mid($self:ident, $other:ident) = $code:block ) => {
        impl CompleteIdx for $type {
            fn mid($self, $other: Self) -> Self { $code }
        }
    };
}
completeidx! { usize, mid(self, other) = { (self + other) / 2 }}
completeidx! { u128, mid(self, other) = { (self + other) / 2 }}
completeidx! { i128, mid(self, other) = { (self + other) / 2 }}
completeidx! { u64, mid(self, other) = { (self + other) / 2 }}
completeidx! { i64, mid(self, other) = { (self + other) / 2 }}
completeidx! { f64, mid(self, other) = { (self + other) / 2.0 }}

pub fn lowerbound<T: CompleteIdx>(r: std::ops::Range<T>, cond: &dyn Fn(T) -> bool) -> Option<T> {
    if cond(r.start) {
        return Some(r.start);
    }
    // TODO(from 1.47.0)
    // if r.is_empty() { return None }
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
pub fn count<X: CompleteIdx + Ord>(xs: &Vec<X>, range: std::ops::Range<X>) -> usize {
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

// @sequence/cumsum1d
// @algebra/group_additive
/// Algebra - AGroup (Additive Group) (+, -, 0)
pub trait AGroup:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::iter::Sum
where
    Self: std::marker::Sized,
{
    fn zero() -> Self;
}

#[macro_export]
macro_rules! agroup {
    (
        $type:ty where [ $( $params:tt )* ] ;
        zero = $zero:expr ;
        add($self:ident, $y:ident) = $code:block ;
        neg($self_neg:ident) = $code_neg:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Add for $type {
            type Output = Self;
            fn add($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::Neg for $type {
            type Output = Self;
            fn neg($self_neg) -> Self { $code_neg }
        }
        impl<$($params)*> std::ops::Sub for $type {
            type Output = Self;
            fn sub($self, other: Self) -> Self { ($self) + (-other) }
        }
        impl<$($params)*> std::ops::AddAssign for $type where Self: Clone {
            fn add_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() + $y;
            }
        }
        impl<$($params)*> std::ops::SubAssign for $type where Self: Clone {
            fn sub_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() - $y;
            }
        }
        impl<$($params)*> std::iter::Sum for $type {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), std::ops::Add::add)
            }
        }
        impl<$($params)*> AGroup for $type {
            fn zero() -> Self { $zero }
        }
    };
    (
        $type:ty ;
        zero = $zero:expr ;
        add($self:ident, $y:ident) = $code:block ;
        neg($self_neg:ident) = $code_neg:block
        $(;)*
    ) => {
        agroup! { $type where []; zero = $zero; add($self, $y) = $code; neg($self_neg) = $code_neg; }
    };
}

impl AGroup for i64 {
    fn zero() -> Self {
        0
    }
}
impl AGroup for i128 {
    fn zero() -> Self {
        0
    }
}
impl AGroup for f64 {
    fn zero() -> Self {
        0.0
    }
}

/// Sequence - Cumulative Summation 1D of Additive Group (+, 0)
#[derive(Debug)]
pub struct Cumsum1d<T>(Vec<T>);
impl<T: Copy + AGroup> Cumsum1d<T> {
    pub fn new(xs: &Vec<T>) -> Self {
        let mut ac = T::zero();
        let mut arr = vec![T::zero(); xs.len()];
        for i in 0..arr.len() {
            ac = ac + xs[i];
            arr[i] = ac;
        }
        Self(arr)
    }
    /// sum of [0, idx)
    fn sum_up(&self, idx: usize) -> T {
        if idx > 0 {
            self.0[idx - 1]
        } else {
            T::zero()
        }
    }
    /// sum(i..j) = sum of [i, j)
    pub fn sum(&self, range: std::ops::Range<usize>) -> T {
        if range.start >= range.end {
            T::zero()
        } else {
            self.sum_up(range.end) - self.sum_up(range.start)
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
