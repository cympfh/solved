#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let s = sc.chars();
    let xs: Vec<ModInt> = s
        .into_iter()
        .map(|c| {
            let x = (c as u8) - ('0' as u8);
            mint!(x as i128)
        })
        .collect();
    trace!(&xs);
    let n = xs.len();

    let cum = Cumsum1d::new(&xs);

    let mut base = mint!(1);
    let mut ans = mint!(0);
    for i in 0..n - 1 {
        let s1 = mint!(2).pow((n - 2 - i) as i128);
        let s2 = mint!(2).pow((n - 1 - i) as i128);
        trace!(s1, (n - 2 - i));
        trace!(s2, (n - 1 - i));
        trace!(base, cum.sum(0..n - 1 - i), xs[n - 1 - i]);
        ans += base * (cum.sum(0..n - 1 - i) * s1 + xs[n - 1 - i] * s2);
        base *= 10;
    }
    ans += xs[0] * base;
    put!(ans);
}

// @sequence/cumsum1d
/// Sequence - Cumulative Summation 1D of Additive Group (+, 0)
#[derive(Debug)]
pub struct Cumsum1d<T>(Vec<T>);
impl<T: Copy + AGroup> Cumsum1d<T> {
    pub fn new(xs: &Vec<T>) -> Self {
        let mut ac = T::zero();
        let mut arr = vec![T::zero(); xs.len()];
        for i in 0..arr.len() {
            ac = ac + xs[i];
            arr[i] = ac;
        }
        Self(arr)
    }
    /// sum of [0, idx)
    fn sum_up(&self, idx: usize) -> T {
        if idx > 0 {
            self.0[idx - 1]
        } else {
            T::zero()
        }
    }
    /// sum(i..j) = sum of [i, j)
    pub fn sum(&self, range: std::ops::Range<usize>) -> T {
        if range.start >= range.end {
            T::zero()
        } else {
            self.sum_up(range.end) - self.sum_up(range.start)
        }
    }
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
pub struct ModInt(pub i128, pub i128); // (residual, modulo)

pub const MOD_1000000007: i128 = 1_000_000_007;
pub const MOD_998244353: i128 = 998_244_353;
#[macro_export]
macro_rules! mint {
    ($x:expr) => {
        ModInt::new($x, MOD_998244353)
    };
}

impl ModInt {
    pub fn new(residual: i128, modulo: i128) -> ModInt {
        if residual >= modulo {
            ModInt(residual % modulo, modulo)
        } else if residual < 0 {
            ModInt((residual % modulo) + modulo, modulo)
        } else {
            ModInt(residual, modulo)
        }
    }
    pub fn unwrap(self) -> i128 {
        self.0
    }
    pub fn inv(self) -> Self {
        fn exgcd(r0: i128, a0: i128, b0: i128, r: i128, a: i128, b: i128) -> (i128, i128, i128) {
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
    pub fn pow(self, n: i128) -> Self {
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

impl std::ops::Add<i128> for ModInt {
    type Output = Self;
    fn add(self, other: i128) -> Self {
        ModInt::new(self.0 + other, self.1)
    }
}
impl std::ops::Add<ModInt> for i128 {
    type Output = ModInt;
    fn add(self, other: ModInt) -> ModInt {
        other + self
    }
}
impl std::ops::AddAssign<i128> for ModInt {
    fn add_assign(&mut self, other: i128) {
        self.0 = ModInt::new(self.0 + other, self.1).0;
    }
}
impl std::ops::Sub<i128> for ModInt {
    type Output = Self;
    fn sub(self, other: i128) -> Self {
        ModInt::new(self.0 - other, self.1)
    }
}
impl std::ops::Sub<ModInt> for i128 {
    type Output = ModInt;
    fn sub(self, other: ModInt) -> ModInt {
        ModInt::new(self - other.0, other.1)
    }
}
impl std::ops::SubAssign<i128> for ModInt {
    fn sub_assign(&mut self, other: i128) {
        self.0 = ModInt::new(self.0 - other, self.1).0;
    }
}
impl std::ops::Mul<i128> for ModInt {
    type Output = Self;
    fn mul(self, other: i128) -> Self {
        ModInt::new(self.0 * other, self.1)
    }
}
impl std::ops::Mul<ModInt> for i128 {
    type Output = ModInt;
    fn mul(self, other: ModInt) -> ModInt {
        other * self
    }
}
impl std::ops::MulAssign<i128> for ModInt {
    fn mul_assign(&mut self, other: i128) {
        self.0 = ModInt::new(self.0 * other, self.1).0;
    }
}
impl std::ops::Div<i128> for ModInt {
    type Output = Self;
    fn div(self, other: i128) -> Self {
        self / ModInt::new(other, self.1)
    }
}
impl std::ops::Div<ModInt> for i128 {
    type Output = ModInt;
    fn div(self, other: ModInt) -> ModInt {
        other.inv() * self
    }
}
impl std::ops::DivAssign<i128> for ModInt {
    fn div_assign(&mut self, other: i128) {
        *self /= ModInt(other, self.1);
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
