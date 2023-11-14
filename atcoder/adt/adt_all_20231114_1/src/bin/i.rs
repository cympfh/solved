#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let k: usize = sc.cin();

    let bin1: Vec<(i64, i64)> = (0..n).map(|_| (sc.cin(), sc.cin())).collect();
    let bin2: Vec<(i64, i64)> = (0..m).map(|_| (sc.cin(), sc.cin())).collect();

    let mut left = 0.0; // コレ以上が K 個以上ある
    let mut right = 1.0; // コレ以上が K 個未満ある

    for _ in 0..200 {
        let mid = (left + right) / 2.0;
        let mut num = 0;
        for &(c, d) in bin2.iter() {
            let mut rs: Vec<Float> = bin1
                .iter()
                .map(|&(a, b)| Float((a + c) as f64 / (a + b + c + d) as f64))
                .collect();
            rs.sort();
            // trace!((c, d), &rs, lowerbound(0..n, &|i| rs[i] >= Float(mid)));
            if let Some(i) = lowerbound(0..n, &|i| rs[i] >= Float(mid)) {
                num += n - i;
            }
        }
        if num >= k {
            left = mid;
        } else {
            right = mid;
        }
        // trace!(mid, num);
        // trace!(left, right);
    }
    put!(left * 100.0);
}

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

// @algorithm/binary_search_count
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

// @num/float
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

/// Def of Float Numbers
#[derive(Debug, Clone, Copy)]
pub struct Float(pub f64);
impl Float {
    pub fn abs(&self) -> Self {
        Float(self.0.abs())
    }
    pub fn sin(&self) -> Self {
        Float(self.0.sin())
    }
    pub fn cos(&self) -> Self {
        Float(self.0.cos())
    }
    pub fn tan(&self) -> Self {
        Float(self.0.tan())
    }
}
agroup! {
    Float;
    zero = Float(0.0);
    add(self, other) = { Float(self.0 + other.0) };
    neg(self) = { Float(-self.0) };
}
monoid! {
    Float;
    one = Float(1.0);
    mul(self, other) = { Float(self.0 * other.0) };
}
ring! {
    Float;
    div(self, other) = { Float(self.0 / other.0) };
}
impl std::ops::Rem for Float {
    type Output = Self;
    fn rem(self, modulo: Self) -> Float {
        Float(self.0 % modulo.0)
    }
}
impl std::ops::RemAssign for Float {
    fn rem_assign(&mut self, modulo: Self) {
        self.0 %= modulo.0;
    }
}
impl std::cmp::PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        let allow = 1e-6;
        (self.0 - other.0).abs() < allow
            || (self.0 - other.0).abs() / (self.0 + other.0) * 2.0 < allow
    }
}
impl std::cmp::Eq for Float {}
impl std::cmp::PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
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
