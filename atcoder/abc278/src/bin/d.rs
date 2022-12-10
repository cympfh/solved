#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let a: Vec<i64> = sc.vec(n);
    let q: usize = sc.cin();

    let mut st = RangedAssignRMaxQ::from(a.into_iter().map(|val| MaxInt::Val(val)).collect());

    for _ in 0..q {
        let ty: usize = sc.cin();
        match ty {
            1 => {
                let x: i64 = sc.cin();
                st.update(0..n, Assign::Some(MaxInt::Val(x)));
            }
            2 => {
                let i = sc.usize1();
                let x: i64 = sc.cin();
                let x = st.index(i).unwrap() + x;
                st.update(i..i + 1, Assign::Some(MaxInt::Val(x)));
            }
            _ => {
                let i = sc.usize1();
                let x = st.index(i).unwrap();
                put!(x);
            }
        }
    }
}

// @sequence/tree/ranged_assign_rmq
// @algebra/act_assign
// @algebra/act
/// Algebra - Act
pub trait Act<X> {
    fn act(&self, x: X) -> X;
}

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

/// Algebra - Assign Monoidal Act
#[derive(Debug, Clone, Copy)]
pub enum Assign<X> {
    Some(X),
    None,
}
monoid! {
    Assign<X> where [X];
    one = Assign::None;
    mul(self, other) = {
        match (self, &other) {
            (x, Assign::None) => x,
            _ => other,
        }
    }
}
impl<X: Copy> Act<X> for Assign<X> {
    fn act(&self, other: X) -> X {
        match *self {
            Assign::None => other,
            Assign::Some(x) => x,
        }
    }
}

// @algebra/monoid_max
/// Algebra - Monoid - MaxInt
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MaxInt<X> {
    Minimal,
    Val(X),
}
impl<X> MaxInt<X> {
    pub fn unwrap(self) -> X {
        match self {
            Self::Val(x) => x,
            _ => panic!(),
        }
    }
}
monoid! {
    MaxInt<X> where [X:Ord];
    one = MaxInt::Minimal;
    mul(self, other) = {
        if self > other {
            self
        } else {
            other
        }
    }
}

// @algebra/monoid_min
/// Algebra - Monoid - MinInt
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MinInt<X> {
    Val(X),
    Maximal,
}
impl<X> MinInt<X> {
    pub fn unwrap(self) -> X {
        match self {
            Self::Val(x) => x,
            _ => panic!(),
        }
    }
}
monoid! {
    MinInt<X> where [X:Ord];
    one = MinInt::Maximal;
    mul(self, other) = {
        if self < other {
            self
        } else {
            other
        }
    }
}

// @sequence/tree/lazy_segment_tree
/// Sequence - Lazy Segment Tree
#[derive(Debug, Clone)]
pub struct LazySegmentTree<X, M> {
    length: usize,       // of leaves
    length_upper: usize, // power of 2
    size: usize,         // of nodes
    data: Vec<X>,
    act: Vec<M>,
}
impl<X: Copy + Monoid, M: Copy + Monoid + Act<X>> LazySegmentTree<X, M> {
    pub fn new(length: usize) -> Self {
        let mut length_upper = 1;
        while length_upper < length {
            length_upper *= 2;
        }
        let size = length_upper * 2 - 1;
        let data = vec![X::one(); size];
        let act = vec![M::one(); size];
        LazySegmentTree {
            length,
            length_upper,
            size,
            data,
            act,
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
    fn propagation(&mut self, idx: usize) {
        if idx < self.size / 2 {
            self.act[idx * 2 + 1] = self.act[idx * 2 + 1] * self.act[idx];
            self.act[idx * 2 + 2] = self.act[idx * 2 + 2] * self.act[idx];
        }
        self.data[idx] = self.act[idx].act(self.data[idx]);
        self.act[idx] = M::one();
    }
    fn update_sub(
        &mut self,
        range: std::ops::Range<usize>,
        m: M,
        idx: usize,
        focus: std::ops::Range<usize>,
    ) {
        self.propagation(idx);
        if focus.end <= range.start || range.end <= focus.start {
            return;
        }
        if range.start <= focus.start && focus.end <= range.end {
            self.act[idx] = self.act[idx] * m;
            self.propagation(idx);
        } else if idx < self.data.len() / 2 {
            let mid = (focus.start + focus.end) / 2;
            self.update_sub(range.clone(), m, idx * 2 + 1, focus.start..mid);
            self.update_sub(range.clone(), m, idx * 2 + 2, mid..focus.end);
            self.data[idx] = self.data[idx * 2 + 1] * self.data[idx * 2 + 2];
        }
    }
    pub fn update(&mut self, range: std::ops::Range<usize>, m: M) {
        self.update_sub(range, m, 0, 0..self.length_upper);
    }
    fn product_sub(
        &mut self,
        range: std::ops::Range<usize>,
        idx: usize,
        focus: std::ops::Range<usize>,
    ) -> X {
        self.propagation(idx);
        if focus.end <= range.start || range.end <= focus.start {
            X::one()
        } else if range.start <= focus.start && focus.end <= range.end {
            self.data[idx]
        } else {
            let mid = (focus.start + focus.end) / 2;
            let a = self.product_sub(range.clone(), idx * 2 + 1, focus.start..mid);
            let b = self.product_sub(range.clone(), idx * 2 + 2, mid..focus.end);
            a * b
        }
    }
    pub fn product(&mut self, range: std::ops::Range<usize>) -> X {
        self.product_sub(range, 0, 0..self.length_upper)
    }
    pub fn index(&mut self, i: usize) -> X {
        self.product(i..i + 1)
    }
    pub fn to_vec(&mut self) -> Vec<X> {
        (0..self.length).map(|i| self.index(i)).collect()
    }
}
impl<X: std::fmt::Debug, M: std::fmt::Debug> LazySegmentTree<X, M> {
    pub fn debug(&self) {
        #[cfg(debug_assertions)]
        for i in 0..self.size {
            if i > 0 && (i + 1).count_ones() == 1 {
                eprintln!();
            }
            eprint!("{:?} / {:?}; ", &self.data[i], &self.act[i]);
        }
        eprintln!();
    }
}

/// Sequence - Lazy Segment Tree - Ranged Assign/RMQ
pub type RangedAssignRMaxQ<X> = LazySegmentTree<MaxInt<X>, Assign<MaxInt<X>>>;
pub type RangedAssignRMinQ<X> = LazySegmentTree<MinInt<X>, Assign<MinInt<X>>>;

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
