#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let k: usize = sc.cin();
    let mut a: Vec<i64> = sc.vec(n);
    a.push(0);

    let mut d = vec![a[0]];
    for i in 1..n {
        d.push(a[i] - a[i - 1]);
    }
    trace!(&a);
    trace!(&d);

    let mut ts = vec![];
    for j in 0..k {
        let mut t = SegmentTree::new(n + 2);
        for i in 0..n {
            if i % k == j {
                t.update(i, Sum(d[i]));
            }
        }
        ts.push(t);
    }

    let q: usize = sc.cin();
    for _ in 0..q {
        let left: usize = sc.usize1();
        let right: usize = sc.usize1();
        trace!(left, right);
        let tmp = ts[left % k].at(left);
        let tmp2 = ts[(right + 1) % k].at(right + 1);
        ts[left % k].update(left, Sum(a[left]));
        ts[(right + 1) % k].update(right + 1, Sum(-a[right]));
        let mut ok = true;
        for j in 0..k {
            let x = ts[j].product(left..right + 2);
            if x != Sum(0) {
                ok = false;
            }
        }
        ts[left % k].update(left, tmp);
        ts[(right + 1) % k].update(right + 1, tmp2);
        if ok {
            put!(#Yes);
        } else {
            put!(#No);
        }
    }
}

// @sequence/tree/segment_tree
// @algebra/monoid
/// Algebra - Def of Monoid (*, 1)
pub trait Monoid: std::ops::Mul<Output = Self> + std::iter::Product
where
    Self: std::marker::Sized,
{
    fn one() -> Self;
}

#[macro_export]
macro_rules! monoid {
    (
        $type:ty where [ $( $params:tt )* ];
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Mul for $type {
            type Output = Self;
            fn mul($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::MulAssign for $type where Self: Clone {
            fn mul_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() * $y;
            }
        }
        impl<$($params)*> std::iter::Product for $type {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::one(), std::ops::Mul::mul)
            }
        }
        impl<$($params)*> Monoid for $type {
            fn one() -> Self { $one }
        }
    };
    (
        $type:ty;
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        monoid! { $type where []; one = $one; mul($self, $y) = $code; }
    };
}

impl Monoid for i64 {
    fn one() -> Self {
        1
    }
}
impl Monoid for f64 {
    fn one() -> Self {
        1.0
    }
}

/// Sequence - Segment Tree
pub struct SegmentTree<X> {
    length_upper: usize, // power of 2
    size: usize,         // of nodes
    data: Vec<X>,
}
impl<X> std::ops::Index<usize> for SegmentTree<X> {
    type Output = X;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size / 2 + i]
    }
}
impl<X: Copy + Monoid> SegmentTree<X> {
    pub fn new(length: usize) -> Self {
        let mut length_upper = 1;
        while length_upper < length {
            length_upper <<= 1
        }
        let size = length_upper * 2 - 1;
        let data = vec![X::one(); size];
        SegmentTree {
            length_upper,
            size,
            data,
        }
    }
    pub fn from(xs: Vec<X>) -> Self {
        let mut tree = Self::new(xs.len());
        for i in 0..xs.len() {
            tree.data[tree.size / 2 + i] = xs[i];
        }
        for i in (0..tree.size / 2).rev() {
            tree.data[i] = tree.data[2 * i + 1] * tree.data[2 * i + 2];
        }
        tree
    }
    pub fn to_vec(self) -> Vec<X> {
        self.data[self.size / 2..].to_vec()
    }
    pub fn update(&mut self, i: usize, t: X) {
        let mut u = self.size / 2 + i;
        self.data[u] = t;
        while u > 0 {
            u = (u - 1) / 2;
            self.data[u] = self.data[u * 2 + 1] * self.data[u * 2 + 2];
        }
    }
    fn product_sub(
        &self,
        range: std::ops::Range<usize>,
        u: usize,
        focus: std::ops::Range<usize>,
    ) -> X {
        if focus.end <= range.start || range.end <= focus.start {
            X::one()
        } else if range.start <= focus.start && focus.end <= range.end {
            self.data[u]
        } else {
            let mid = (focus.start + focus.end) / 2;
            let a = self.product_sub(range.clone(), u * 2 + 1, focus.start..mid);
            let b = self.product_sub(range.clone(), u * 2 + 2, mid..focus.end);
            a * b
        }
    }
    pub fn at(&self, i: usize) -> X {
        self.product(i..i + 1)
    }
    pub fn product(&self, range: std::ops::Range<usize>) -> X {
        self.product_sub(range, 0, 0..self.length_upper)
    }
}
impl<X: std::fmt::Debug> SegmentTree<X> {
    pub fn debug(&self) {
        #[cfg(debug_assertions)]
        for i in 0..self.size {
            if i > 0 && (i + 1).count_ones() == 1 {
                eprintln!();
            }
            eprint!("{:?} ", &self.data[i]);
        }
        eprintln!();
    }
}

// @algebra/monoid_sum
/// Algebra - Def of Monoid (i64, +)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum(pub i64);
monoid! {
    Sum;
    one = Sum(0);
    mul(self, other) = {
        Self(self.0 + other.0)
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
