#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let fus: Vec<_> = (0..n)
        .map(|_| {
            let x: i64 = sc.cin();
            let y: i64 = sc.cin();
            (x, y)
        })
        .collect();
    let mut args: Vec<_> = fus
        .iter()
        .map(|&(x, y)| {
            let sub = Ratio::new(y - 1, x);
            let sup = Ratio::new(y, x - 1);
            (sub, sup)
        })
        .collect();
    trace!(&args);
    args.sort_by_key(|&(z0, z1)| (z1, z0));
    trace!(&args);

    let mut last = Ratio::zero();
    let mut ans = 0_usize;
    for (z0, z1) in args {
        if last <= z0 {
            ans += 1;
            last = z1;
        }
    }
    put!(ans);
}

// @algebra/ratio
// @algebra/field
// @algebra/ring
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

/// Algebra - Field ((+, 0), (*, 1), /)
pub trait Field: Ring + std::ops::Div {}

impl Field for i64 {}
impl Field for f64 {}

/// Algebra - Ratio Numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio(i64, i64); // Normalized (numerator / denominator)

impl Ratio {
    pub fn new(a: i64, b: i64) -> Self {
        Ratio(a, b).normalize()
    }
    fn normalize(&mut self) -> Self {
        let g = Self::gcd(self.0.abs(), self.1.abs());
        self.0 /= g;
        self.1 /= g;
        if self.1 < 0 {
            self.0 *= -1;
            self.1 *= -1;
        }
        *self
    }
    pub fn from(x: i64) -> Self {
        Ratio(x, 1)
    }
    pub fn inv(&self) -> Self {
        if self.0 > 0 {
            Ratio(self.1, self.0)
        } else if self.0 < 0 {
            Ratio(-self.1, -self.0)
        } else {
            Ratio(1, 0) //Infinity
        }
    }
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }
}
agroup! {
    Ratio;
    zero = Ratio(0, 1);
    add(self, other) = {
        let num = Self::lcm(self.1, other.1);
        Ratio::new(self.0 * num / self.1 + other.0 * num / other.1, num)
    };
    neg(self) = {
        Ratio(-self.0, self.1)
    };
}
monoid! {
    Ratio;
    one = Ratio(1, 1);
    mul(self, other) = { Self::new(self.0 * other.0, self.1 * other.1) }
}
impl Ring for Ratio {}

impl std::ops::Add<i64> for Ratio {
    type Output = Self;
    fn add(self, z: i64) -> Self {
        Ratio::new(self.0 + self.1 * z, self.1)
    }
}
impl std::ops::AddAssign<i64> for Ratio {
    fn add_assign(&mut self, z: i64) {
        self.0 += self.1 * z;
        self.normalize();
    }
}
impl std::ops::Sub<i64> for Ratio {
    type Output = Self;
    fn sub(self, z: i64) -> Self {
        self + (-z)
    }
}
impl std::ops::SubAssign<i64> for Ratio {
    fn sub_assign(&mut self, z: i64) {
        *self += -z;
    }
}
impl std::ops::Mul<i64> for Ratio {
    type Output = Self;
    fn mul(self, z: i64) -> Self {
        Self::new(self.0 * z, self.1)
    }
}
impl std::ops::MulAssign<i64> for Ratio {
    fn mul_assign(&mut self, z: i64) {
        self.0 *= z;
        self.normalize();
    }
}
impl std::ops::Div for Ratio {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.0 * other.1, self.1 * other.0)
    }
}
impl Field for Ratio {}
impl std::ops::Div<i64> for Ratio {
    type Output = Self;
    fn div(self, other: i64) -> Self {
        Self::new(self.0, self.1 * other)
    }
}
impl std::ops::DivAssign<i64> for Ratio {
    fn div_assign(&mut self, z: i64) {
        self.1 *= z;
        self.normalize();
    }
}
impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.0 * other.1;
        let right = other.0 * self.1;
        left.partial_cmp(&right)
    }
}
impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[macro_export]
macro_rules! r {
    ($x:expr, $y:expr) => {
        Ratio::new($x, $y)
    };
}

// @algebra/total
/// Algebra - Totalize PartialOrd Things
/// Thanks to: https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Total<T>(T);
impl<T> Total<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}
impl<T: PartialEq> Eq for Total<T> {}
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, rhs: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&rhs.0).unwrap()
    }
}

pub type Float = Total<f64>;
pub fn float(x: f64) -> Float {
    Total(x)
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;

struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }
    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
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
