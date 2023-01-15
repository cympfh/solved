#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn ord(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

/// left..right に c が登場する最小の区間
fn mostright(c: &BIT<i32>, left: usize, right: usize) -> Option<usize> {
    let m = c.sum(left..right);
    if m == 0 {
        return None;
    }
    if m == (right - left) as i32 {
        return Some(right);
    }
    let mut x0 = left;
    let mut x1 = right;
    for _ in 0..17 {
        let mid = (x0 + x1) / 2;
        if c.sum(left..mid) == m {
            x1 = mid;
        } else {
            x0 = mid;
        }
        if x0 + 1 == x1 {
            break;
        }
    }
    Some(x1)
}

fn check(count: &[BIT<i32>], left: usize, right: usize) -> bool {
    let n = count[0].size;
    if left + 1 == right {
        return true;
    }
    let mut freq = vec![];
    for c in 0..26 {
        if let Some(i) = mostright(&count[c], left, right) {
            freq.push((c, i));
        }
    }

    // sorted?
    for i in 1..freq.len() {
        if freq[i - 1].1 > freq[i].1 {
            return false;
        }
        let c = freq[i].0;
        if count[c].sum(left..freq[i - 1].1) > 0 {
            return false;
        }
    }
    // inner chars
    if freq.len() >= 2 {
        let minchar = freq[0].0;
        let maxchar = freq[freq.len() - 1].0;
        for c in minchar + 1..maxchar {
            if count[c].sum(left..right) != count[c].sum(0..n) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let mut s: Vec<char> = sc.chars();

    let mut count = vec![BIT::new(n + 1); 26];
    for i in 0..n {
        count[ord(s[i])].add(i, 1_i32);
    }

    let q: usize = sc.cin();
    for _ in 0..q {
        let ty: usize = sc.cin();
        if ty == 1 {
            let x = sc.usize1();
            let c = sc.chars()[0];
            count[ord(s[x])].add(x, -1);
            count[ord(c)].add(x, 1);
            s[x] = c;
        } else {
            let left = sc.usize1();
            let right: usize = sc.cin();
            if check(&count, left, right) {
                put!(#Yes);
            } else {
                put!(#No);
            }
        }
    }
}

// @sequence/tree/bit
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

impl AGroup for i32 {
    fn zero() -> Self {
        0
    }
}
impl AGroup for i64 {
    fn zero() -> Self {
        0
    }
}

/// Sequence - Binary Indexed Tree (Fenwick Tree) of Additive Group (+, 0)
#[derive(Debug, Clone)]
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
    (# $a:ident) => {
        #[cfg(debug_assertions)]
        eprintln!("{}", stringify!($a))
    };
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
