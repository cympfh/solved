#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let p: Vec<usize> = sc.vec(n);
    use Hyper::*;
    let mut d = vec![Inf; n];
    {
        let mut rmq = RMaxQ::from(vec![MaxInt::Minimal; 200_200]);
        for i in 0..n {
            let x = p[i] as i64 + i as i64;
            if let MaxInt::Val(max) = rmq.product(0..p[i]) {
                d[i] = min!(d[i], Real(x - max));
            }
            rmq.update(p[i], MaxInt::Val(x));
        }
        trace!(&d);
    }
    {
        let mut rmq = RMinQ::from(vec![MinInt::Maximal; 200_200]);
        for i in 0..n {
            let x = p[i] as i64 - i as i64;
            if let MinInt::Val(min) = rmq.product(p[i]..200_200) {
                d[i] = min!(d[i], Real(-x + min));
            }
            rmq.update(p[i], MinInt::Val(x));
        }
        trace!(&d);
    }
    {
        trace!(":)");
        let mut rmq = RMaxQ::from(vec![MaxInt::Minimal; 200_200]);
        for i in (0..n).rev() {
            let x = p[i] as i64 - i as i64;
            if let MaxInt::Val(max) = rmq.product(0..p[i]) {
                d[i] = min!(d[i], Real(x - max));
            }
            rmq.update(p[i], MaxInt::Val(x));
        }
        trace!(&d);
    }
    {
        let mut rmq = RMinQ::from(vec![MinInt::Maximal; 200_200]);
        for i in (0..n).rev() {
            let x = p[i] as i64 + i as i64;
            if let MinInt::Val(min) = rmq.product(p[i]..200_200) {
                d[i] = min!(d[i], Real(-x + min));
            }
            rmq.update(p[i], MinInt::Val(x));
        }
        trace!(&d);
    }
    let ans: Vec<i64> = d.into_iter().map(|x| x.unwrap()).collect();
    put!(..ans);
}

// @sequence/tree/rmq
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

// @sequence/tree/segment_tree
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

/// Sequence - Range Maximum/Minimum Query
pub type RMaxQ<X> = SegmentTree<MaxInt<X>>;
pub type RMinQ<X> = SegmentTree<MinInt<X>>;

// @algebra/hyper
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
impl AGroup for f64 {
    fn zero() -> Self {
        0.0
    }
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

// @algebra/ring
/// Algebra - Ring ((+, 0), (*, 1))
pub trait Ring: AGroup + Monoid {}

#[macro_export]
macro_rules! ring {
    (
        $type:ty where [ $( $params:tt )* ];
        div($self:ident, $other:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Div for $type {
            type Output = Self;
            fn div($self, $other: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::DivAssign for $type where Self: Clone {
            fn div_assign(&mut $self, $other: Self) { *$self = (*$self).clone() / $other; }
        }
        impl Ring for $type {}
    };
    (
        $type:ty;
        div($self:ident, $other:ident) = $code:block
        $(;)*
    ) => {
        ring! { $type where []; div($self, $other) = $code; }
    };
}

impl Ring for i64 {}
impl Ring for f64 {}

/// Algebra - Hyper Numbers (numbers with infinity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hyper<X> {
    NegInf,
    Real(X),
    Inf,
}
use Hyper::{Inf, NegInf, Real};
impl<X> Hyper<X> {
    pub fn unwrap(self) -> X {
        if let Hyper::Real(x) = self {
            x
        } else {
            panic!("Could not unwrap Hyper")
        }
    }
}
agroup! {
    Hyper<X> where [X: AGroup];
    zero = Real(X::zero());
    add(self, other) = {
        match (self, other) {
            (Real(x), Real(y)) => Real(x + y),
            (Inf, _) => Inf,
            (_, Inf) => Inf,
            _ => NegInf,
        }
    };
    neg(self) = {
        match self {
            Inf => NegInf,
            NegInf => Inf,
            Real(x) => Real(-x),
        }
    };
}
monoid! {
    Hyper<X> where [X: Monoid];
    one = Real(X::one());
    mul(self, other) = {
        match (self, other) {
            (Real(x), Real(y)) => Real(x * y),
            (Inf, Inf) | (NegInf, NegInf) => Inf,
            _ => NegInf,
        }
    };
}
impl<X: AGroup + Monoid> Ring for Hyper<X> {}
impl<X: std::ops::Add<Output = X>> std::ops::Add<X> for Hyper<X> {
    type Output = Self;
    fn add(self, y: X) -> Hyper<X> {
        match (self, y) {
            (Real(x), y) => Real(x + y),
            (Inf, _) => Inf,
            _ => NegInf,
        }
    }
}
impl<X: Clone + AGroup> std::ops::AddAssign<X> for Hyper<X> {
    fn add_assign(&mut self, y: X) {
        *self = (*self).clone() + Real(y);
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
