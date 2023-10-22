#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let a: i64 = sc.cin();
    let b: i64 = sc.cin();
    let c: i64 = sc.cin();
    let d: Vec<Vec<i64>> = (0..n).map(|_| sc.vec(n)).collect();

    use Hyper::*;

    // 行き, 車
    let mut g: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            g[i].push((j, d[i][j] * a));
        }
    }
    // 帰り, 電車
    let mut h: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            h[i].push((j, d[j][i] * b + c));
        }
    }

    let dg = dijkstra(0, &g);
    let dh = dijkstra(n - 1, &h);
    trace!(&dg);
    trace!(&dh);
    let mut ans = Inf;
    for i in 0..n {
        match (dg[i], dh[i]) {
            (Real(x), Real(y)) => ans = min!(ans, Real(x + y)),
            _ => {}
        }
    }
    if let Real(ans) = ans {
        put!(ans);
    }
}

// @graph/shortest/dijkstra
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

// @algebra/hyper
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

/// Graph - Dijkstra
pub fn dijkstra<Cost: Copy + AGroup + Ord>(
    s: usize,
    neigh: &Vec<Vec<(usize, Cost)>>,
) -> Vec<Hyper<Cost>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = neigh.len();
    let mut d: Vec<Hyper<Cost>> = vec![Hyper::Inf; n];
    let mut q = BinaryHeap::new();
    d[s] = Hyper::Real(Cost::zero());
    q.push((Reverse(d[s]), s));
    while let Some((_, u)) = q.pop() {
        for &(v, cost) in neigh[u].iter() {
            if d[v] > d[u] + cost {
                d[v] = d[u] + cost;
                q.push((Reverse(d[v]), v));
            }
        }
    }
    d
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
