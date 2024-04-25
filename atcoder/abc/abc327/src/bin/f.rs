#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let w: usize = sc.cin();
    let h: usize = sc.cin();

    let ps: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let x: usize = sc.cin();
            let y: usize = sc.cin();
            (x, y)
        })
        .collect();

    use MaxInt::*;
    let zeros = vec![Val(0); 210_000];
    let mut st: LazySegmentTree<MaxInt<i64>, Add> = LazySegmentTree::from(zeros);

    let mut events = vec![];
    for &(x, y) in ps.iter() {
        events.push((x, y, true));
        events.push((x + w, y, false));
    }
    events.sort();

    let mut ans = 0;
    let mut lastx = 0;
    for (x, y, bl) in events {
        trace!(lastx, (x, y, bl));
        if x > lastx {
            if let Val(m) = st.product(0..210_000) {
                ans = max!(ans, m);
            }
        }
        if bl {
            st.update(y..y + h, Add(1));
        } else {
            st.update(y..y + h, Add(-1));
        }
        lastx = x;
    }
    if let Val(m) = st.product(0..210_000) {
        ans = max!(ans, m);
    }
    put!(ans);
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

#[derive(Debug, Clone, Copy)]
struct Add(i64);
monoid! {
    Add;
    one = Add(0);
    mul(self, other) = {
        Add(self.0 + other.0)
    }
}
impl Act<MaxInt<i64>> for Add {
    fn act(&self, other: MaxInt<i64>) -> MaxInt<i64> {
        match other {
            MaxInt::Minimal => MaxInt::Minimal,
            MaxInt::Val(x) => MaxInt::Val(x + self.0),
        }
    }
}

// @sequence/tree/lazy_segment_tree
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
impl Monoid for i128 {
    fn one() -> Self {
        1
    }
}
impl Monoid for f64 {
    fn one() -> Self {
        1.0
    }
}

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
