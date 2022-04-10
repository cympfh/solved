#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let q: usize = sc.cin();
    let mut s: Vec<char> = sc.chars();

    let mut bit = BIT::new(n + 1);
    for i in 0..n {
        if s[i] == '(' {
            bit.add(i, 1);
        } else {
            bit.add(i, -1);
        }
    }

    let mut v = vec![];
    {
        let mut last: i64 = 0;
        for i in 0..n {
            if s[i] == '(' {
                last += 1;
            } else {
                last -= 1;
            }
            v.push(MinInt::Val(last));
        }
    }
    let mut rmq = RangedRMinQ::from(v);

    for _ in 0..q {
        let ty: usize = sc.cin();
        let u = sc.cin::<usize>() - 1;
        let v = sc.cin::<usize>() - 1;
        if ty == 1 {
            if s[u] == '(' && s[v] == ')' {
                bit.add(u, -2);
                bit.add(v, 2);
                s[u] = ')';
                s[v] = '(';
                rmq.update(u..v, Plus(-2));
            } else if s[u] == ')' && s[v] == '(' {
                bit.add(u, 2);
                bit.add(v, -2);
                s[u] = '(';
                s[v] = ')';
                rmq.update(u..v, Plus(2));
            }
        } else {
            if bit.sum(u..v + 1) == 0 && rmq.product(u..v).unwrap() >= bit.sum_up(u) {
                put!("Yes");
            } else {
                put!("No");
            }
        }
    }
}

// @sequence/tree/ranged_rmq
// @algebra/act_assign
// @algebra/act
/// Algebra - Act
pub trait Act<X> {
    fn act(&self, x: X) -> X;
}

// @algebra/monoid
/// Algebra - Def of Monoid (*, 1)
pub trait Monoid: std::ops::Mul<Output = Self>
where
    Self: std::marker::Sized,
{
    fn unit() -> Self;
}

#[macro_export]
macro_rules! monoid {
    (
        [ $( $params:tt )* ]
        for $type:ty;
        unit = $unit:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Mul for $type {
            type Output = Self;
            fn mul($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> Monoid for $type {
            fn unit() -> Self { $unit }
        }
    };
    (
        for $type:ty;
        unit = $unit:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        monoid! { [] for $type; unit = $unit; mul($self, $y) = $code; }
    };
}

/// Algebra - Plus Monoidal Act
#[derive(Debug, Clone, Copy)]
struct Plus(i64);

impl std::ops::Mul for Plus {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Plus(self.0 + other.0)
    }
}
impl Act<MinInt<i64>> for Plus {
    fn act(&self, other: MinInt<i64>) -> MinInt<i64> {
        match other {
            MinInt::Val(z) => MinInt::Val(z + self.0),
            _ => MinInt::Maximal,
        }
    }
}
impl Monoid for Plus {
    fn unit() -> Self {
        Plus(0)
    }
}

// @algebra/monoid_minmax
/// Algebra - Monoid - MinInt, MaxInt
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
    [X:Ord] for MaxInt<X>;
    unit = MaxInt::Minimal;
    mul(self, other) = {
        if self > other {
            self
        } else {
            other
        }
    }
}

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
    [X:Ord] for MinInt<X>;
    unit = MinInt::Maximal;
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
        let data = vec![X::unit(); size];
        let act = vec![M::unit(); size];
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
        self.act[idx] = M::unit();
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
            X::unit()
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

/// Sequence - Lazy Segment Tree - Ranged RMQ

pub type RangedRMaxQ<X> = LazySegmentTree<MaxInt<X>, Plus>;
pub type RangedRMinQ<X> = LazySegmentTree<MinInt<X>, Plus>;

// @sequence/tree/bit
// @algebra/group
/// Algebra - Group (+, -, 0)
pub trait Group:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::iter::Sum
{
    fn zero() -> Self;
}
macro_rules! define_group {
    ($t:ty, $x:expr) => {
        impl Group for $t {
            fn zero() -> Self {
                $x
            }
        }
    };
}
define_group!(i32, 0);
define_group!(i64, 0);
define_group!(i128, 0);
define_group!(f32, 0.0);
define_group!(f64, 0.0);

/// Sequence - Binary Indexed Tree (Fenwick Tree)

pub struct BIT<X> {
    size: usize,
    array: Vec<X>,
}
impl<X: Copy + Group> BIT<X> {
    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            array: vec![X::zero(); n + 1],
        }
    }
    pub fn add(&mut self, idx: usize, w: X) {
        let mut x = idx + 1;
        while x <= self.size {
            self.array[x] = self.array[x] + w;
            let xi = x as i32;
            x += (xi & -xi) as usize;
        }
    }
    /// sum of [0, idx)
    fn sum_up(&self, idx: usize) -> X {
        let mut sum = X::zero();
        let mut x = idx;
        while x > 0 {
            sum = sum + self.array[x];
            let xi = x as i32;
            x -= (xi & -xi) as usize;
        }
        sum
    }
    /// sum of [left, right)
    pub fn sum(&self, range: std::ops::Range<usize>) -> X {
        if range.end <= range.start {
            return X::zero();
        }
        self.sum_up(range.end) - self.sum_up(range.start)
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
// }}}
