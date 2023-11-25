#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let k: i64 = sc.cin();
    let ps: Vec<(i64, i64)> = (0..n).map(|_| (sc.cin(), sc.cin())).collect();

    let mut xs: Vec<i64> = ps.iter().map(|&(x, _)| x).collect();
    let mut ys: Vec<i64> = ps.iter().map(|&(_, y)| y).collect();
    xs.sort();
    ys.sort();
    trace!(&xs);
    trace!(&ys);

    let xbit = BIT::from(&xs);
    let ybit = BIT::from(&ys);

    let mut left = 0_i64;
    let mut right = 3_000_000_000;
    for _ in 0..40 {
        let mid = (left + right) / 2;
        if minops(&xs, mid, &xbit) + minops(&ys, mid, &ybit) <= k {
            right = mid;
        } else {
            left = mid;
        }
    }
    put!(right);
}

// 区間に収めるための最小回数
fn minops(xs: &Vec<i64>, m: i64, xbit: &BIT<i64>) -> i64 {
    let mut mini = ops(xs, 0, m, xbit);
    let mut z0 = *xs.iter().min().unwrap();
    let mut z3 = *xs.iter().max().unwrap();
    for _ in 0..60 {
        let z1 = (z0 + z0 + z3) / 3;
        let z2 = (z0 + z3 + z3) / 3;
        let m1 = ops(xs, z1, m, xbit);
        let m2 = ops(xs, z2, m, xbit);
        if m1 > m2 {
            z0 = z1;
        } else if m1 < m2 {
            z3 = z2;
        } else {
            z0 += 1;
            z3 -= 1;
        }
        mini = min!(mini, m1, m2);
        if z0 >= z3 {
            break;
        }
    }
    mini
}

// 区間 [z, z+m] に収めるための素直な回数
fn ops(xs: &Vec<i64>, z: i64, m: i64, xbit: &BIT<i64>) -> i64 {
    let n = count(&xs, 0..z);
    let mut k = 0;
    k += n as i64 * z - xbit.sum(0..n);
    let n = count(&xs, z + m + 1..3_000_000_000);
    k += xbit.sum(xs.len() - n..xs.len()) - n as i64 * (z + m);
    k
}

// @sequence/tree/bit_cumulative
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

// @sequence/tree/bit
/// Sequence - Binary Indexed Tree (Fenwick Tree) of Additive Group (+, 0)
pub struct BIT<X> {
    size: usize,
    array: Vec<X>,
}
impl<X: Copy + AGroup> BIT<X> {
    pub fn new(size: usize) -> Self {
        BIT {
            size,
            array: vec![X::zero(); size + 1],
        }
    }
    pub fn from(xs: &Vec<X>) -> Self {
        let mut r = BIT::new(xs.len());
        for i in 0..xs.len() {
            r.add(i, xs[i]);
        }
        r
    }
    pub fn add(&mut self, idx: usize, w: X) {
        let mut x = idx + 1;
        while x <= self.size {
            self.array[x] = self.array[x] + w;
            let xi = x as i32;
            x += (xi & -xi) as usize;
        }
    }
    /// sum of [0, idx)
    pub fn sum_up(&self, idx: usize) -> X {
        let mut sum = X::zero();
        let mut x = idx;
        while x > 0 {
            sum = sum + self.array[x];
            let xi = x as i32;
            x -= (xi & -xi) as usize;
        }
        sum
    }
    /// sum of [left, right)
    pub fn sum(&self, range: std::ops::Range<usize>) -> X {
        if range.end <= range.start {
            return X::zero();
        }
        self.sum_up(range.end) - self.sum_up(range.start)
    }
}

/// Sequence - Cumulative Array by BIT (Fenwick Tree)
pub struct CumBIT<X> {
    data: BIT<X>,
}
impl<X: Copy + AGroup> CumBIT<X> {
    pub fn new(size: usize) -> Self {
        let data = BIT::new(size);
        Self { data }
    }
    pub fn add(&mut self, range: std::ops::Range<usize>, x: X) {
        self.data.add(range.start, x);
        self.data.add(range.end, -x);
    }
    pub fn at(&self, idx: usize) -> X {
        self.data.sum_up(idx + 1)
    }
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
