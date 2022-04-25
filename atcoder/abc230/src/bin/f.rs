#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn numof_subseq<T: std::fmt::Debug + Ord + Eq>(xs: &[T], zero: ModInt) -> ModInt {
    trace!(&xs);
    let n = xs.len();
    if xs.is_empty() {
        return zero + 1;
    }
    // dp[i] = i 文字目未満までを決める, 最後の i-1 文字目は使う
    let mut dp = vec![zero; n + 1];
    // dpsum[i] = sum of dp[0] + ... + dp[i] = i 文字目未満まで決めた
    let mut dpsum = vec![zero; n + 1];
    // mp[x] = 最後に x が出現した位置 i
    let mut mp = std::collections::BTreeMap::new();
    dp[0] += 1;
    dpsum[0] += 1;
    for (i, x) in xs.iter().enumerate() {
        dp[i + 1] = dpsum[i];
        if let Some(&prev) = mp.get(x) {
            let df = dpsum[prev];
            dp[i + 1] -= df;
        }
        dpsum[i + 1] = dpsum[i] + dp[i + 1];
        mp.insert(x, i);
        trace!(i + 1, &dp[i + 1], &dpsum[i + 1], &mp);
    }
    trace!(&dp);
    trace!(&dpsum);
    dpsum[n]
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let xs: Vec<i64> = sc.vec(n);
    let mut zs = vec![0];
    for i in 0..n {
        zs.push(zs[i] + xs[i]);
    }
    trace!(&zs);
    if zs.len() < 3 {
        put!(1);
    } else {
        put!(numof_subseq(&zs[1..zs.len() - 1], mint!(0)));
    }
}

// @collections/defaultdict
/// collections - defaultdict
#[derive(Debug, Clone)]
pub struct DefaultDict<K, V>
where
    K: Eq + std::hash::Hash,
{
    data: std::collections::HashMap<K, V>,
    default: V,
}
impl<K: Eq + std::hash::Hash, V> DefaultDict<K, V> {
    pub fn new(default: V) -> DefaultDict<K, V> {
        DefaultDict {
            data: std::collections::HashMap::new(),
            default,
        }
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<K, V> {
        self.data.keys()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.data.iter()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl<K: Eq + std::hash::Hash, V> std::ops::Index<K> for DefaultDict<K, V> {
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        if let Some(val) = self.data.get(&key) {
            val
        } else {
            &self.default
        }
    }
}
impl<K: Eq + std::hash::Hash + Clone, V: Clone> std::ops::IndexMut<K> for DefaultDict<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        let val = self.default.clone();
        self.data.entry(key.clone()).or_insert(val);
        self.data.get_mut(&key).unwrap()
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
