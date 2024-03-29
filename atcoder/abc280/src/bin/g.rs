#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let d: i64 = sc.cin();
    let ps: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = sc.cin();
            let y: i64 = sc.cin();
            (x, y)
        })
        .collect();
    // let mut close = ndarray![false; n,n];
    // for i in 0..n {
    //     for j in i..n {
    //         if dist(ps[i].0, ps[i].1, ps[j].0, ps[j].1) <= d {
    //             close[i][j] = true;
    //             close[j][i] = true;
    //         }
    //     }
    // }
    // trace!(&close);

    let mut ans = mint!(0);
    for &(x0, y0) in ps.iter() {
        let mut n1 = 0;
        let mut n2 = 0;
        let mut n3 = 0;
        for &(x, y) in ps.iter() {
            if x <= x0 + d && y >= y0 && x - y >= x0 - y0 {
                n1 += 1;
            }
            if y <= y0 + d && x >= x0 && x - y <= x0 - y0 {
                n2 += 1;
            }
            if x0 <= x && x <= x0 + d && y0 <= y && y <= y0 + d && x - y == x0 - y0 {
                n3 += 1;
            }
        }
        trace!((x0, y0), n1, n2, n3);
        let m1 = mint!(2).pow(n1 - 1);
        let m2 = mint!(2).pow(n2 - 1);
        let m3 = mint!(2).pow(n3 - 1);
        trace!(m1, m2, m3);
        ans += m1 + m2 - m3;
        trace!(ans);
    }
    put!(ans);
}

// @algebra/modint
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

/// Algebra - ModInt (Z/pZ)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModInt(pub i64, pub i64); // (residual, modulo)

pub const MOD_1000000007: i64 = 1_000_000_007;
pub const MOD_998244353: i64 = 998_244_353;
#[macro_export]
macro_rules! mint {
    ($x:expr) => {
        ModInt::new($x, MOD_998244353)
    };
}

impl ModInt {
    pub fn new(residual: i64, modulo: i64) -> ModInt {
        if residual >= modulo {
            ModInt(residual % modulo, modulo)
        } else if residual < 0 {
            ModInt((residual % modulo) + modulo, modulo)
        } else {
            ModInt(residual, modulo)
        }
    }
    pub fn unwrap(self) -> i64 {
        self.0
    }
    pub fn inv(self) -> Self {
        fn exgcd(r0: i64, a0: i64, b0: i64, r: i64, a: i64, b: i64) -> (i64, i64, i64) {
            if r > 0 {
                exgcd(r, a, b, r0 % r, a0 - r0 / r * a, b0 - r0 / r * b)
            } else {
                (a0, b0, r0)
            }
        }
        let (a, _, r) = exgcd(self.0, 1, 0, self.1, 0, 1);
        if r != 1 {
            panic!("{:?} has no inverse!", self);
        }
        ModInt(((a % self.1) + self.1) % self.1, self.1)
    }
    pub fn pow(self, n: i64) -> Self {
        if n < 0 {
            self.pow(-n).inv()
        } else if n == 0 {
            ModInt(1, self.1)
        } else if n == 1 {
            self
        } else {
            let mut x = (self * self).pow(n / 2);
            if n % 2 == 1 {
                x *= self
            }
            x
        }
    }
}
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
agroup! {
    ModInt;
    zero = mint!(0);
    add(self, other) = { ModInt::new(self.0 + other.0, self.1) };
    neg(self) = {
        if self.0 == 0 {
            self
        } else {
            ModInt(self.1 - self.0, self.1)
        }
    };
}
monoid! {
    ModInt;
    one = mint!(1);
    mul(self, other) = { ModInt::new(self.0 * other.0, self.1) };
}
ring! {
    ModInt;
    div(self, other) = { self * other.inv() };
}
impl Field for ModInt {}

impl std::ops::Add<i64> for ModInt {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        ModInt::new(self.0 + other, self.1)
    }
}
impl std::ops::Add<ModInt> for i64 {
    type Output = ModInt;
    fn add(self, other: ModInt) -> ModInt {
        other + self
    }
}
impl std::ops::AddAssign<i64> for ModInt {
    fn add_assign(&mut self, other: i64) {
        self.0 = ModInt::new(self.0 + other, self.1).0;
    }
}
impl std::ops::Sub<i64> for ModInt {
    type Output = Self;
    fn sub(self, other: i64) -> Self {
        ModInt::new(self.0 - other, self.1)
    }
}
impl std::ops::Sub<ModInt> for i64 {
    type Output = ModInt;
    fn sub(self, other: ModInt) -> ModInt {
        ModInt::new(self - other.0, other.1)
    }
}
impl std::ops::SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, other: i64) {
        self.0 = ModInt::new(self.0 - other, self.1).0;
    }
}
impl std::ops::Mul<i64> for ModInt {
    type Output = Self;
    fn mul(self, other: i64) -> Self {
        ModInt::new(self.0 * other, self.1)
    }
}
impl std::ops::Mul<ModInt> for i64 {
    type Output = ModInt;
    fn mul(self, other: ModInt) -> ModInt {
        other * self
    }
}
impl std::ops::MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, other: i64) {
        self.0 = ModInt::new(self.0 * other, self.1).0;
    }
}
impl std::ops::Div<i64> for ModInt {
    type Output = Self;
    fn div(self, other: i64) -> Self {
        self / ModInt::new(other, self.1)
    }
}
impl std::ops::Div<ModInt> for i64 {
    type Output = ModInt;
    fn div(self, other: ModInt) -> ModInt {
        other.inv() * self
    }
}
impl std::ops::DivAssign<i64> for ModInt {
    fn div_assign(&mut self, other: i64) {
        *self /= ModInt(other, self.1);
    }
}

fn dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    // dx = a+c
    // dy = b+c
    let mut left = -1_000_000_000;
    let mut right = 1_000_000_000;
    for _ in 0..500 {
        let c1 = (left + left + right) / 3;
        let c2 = (left + right + right) / 3;
        let d1 = aux(dx, dy, c1);
        let d2 = aux(dx, dy, c2);
        if d1 < d2 {
            right = c2;
        } else {
            left = c1;
        }
    }
    let c = (left + right) / 2;
    (c - 5..c + 5).map(|c| aux(dx, dy, c)).min().unwrap()
}

fn aux(dx: i64, dy: i64, c: i64) -> i64 {
    let a = dx - c;
    let b = dy - c;
    a.abs() + b.abs() + c.abs()
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
