#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy)]
enum Query {
    Add(usize, usize, i64),
    Replace(usize, i64),
    Print(usize, usize),
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let q: usize = sc.cin();
    let qs: Vec<Query> = (0..q)
        .map(|_| {
            let ty: usize = sc.cin();
            if ty == 1 {
                let left: usize = sc.cin();
                let right: usize = sc.cin();
                let x: i64 = sc.cin();
                Query::Add(left, right, x)
            } else if ty == 2 {
                let i: usize = sc.cin();
                let x: i64 = sc.cin();
                Query::Replace(i, x)
            } else {
                let i: usize = sc.cin();
                let j: usize = sc.cin();
                Query::Print(i, j)
            }
        })
        .collect();
    trace!(&qs);

    let mut e: Vec<Vec<usize>> = vec![vec![]; q + 3];
    {
        let mut time = q;
        let mut prints: Vec<Vec<usize>> = vec![vec![]; n + 3];
        for i in (0..q).rev() {
            time -= 1;
            match qs[i] {
                Query::Print(i, j) => {
                    prints[i].push(j);
                }
                Query::Replace(i, _) => {
                    for &j in prints[i].iter() {
                        e[time].push(j);
                    }
                    prints[i].clear();
                }
                _ => {}
            }
        }
    }

    let mut st: LazySegmentTree<i64, Add<i64>> = LazySegmentTree::from(vec![0; m + 3]);
    let mut hose = DefaultDict::new(0);
    let mut time = 0;
    for q in qs.iter() {
        match *q {
            Query::Add(left, right, x) => {
                st.update(left..right + 1, Add(x));
            }
            Query::Replace(i, x) => {
                for &j in e[time].iter() {
                    let z = st.product(j..j + 1);
                    hose[(i, j)] = x - z;
                }
            }
            Query::Print(i, j) => {
                let z = st.product(j..j + 1);
                let ans = hose[(i, j)] + z;
                trace!(z, hose[(i, j)], ans);
                put!(ans);
            }
        }
        time += 1;
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

// @algebra/act_add
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

/// Algebra - Addition Monoidal Act
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Add<X: AGroup>(pub X);
monoid! {
    Add<X> where [X:AGroup];
    one = Add(X::zero());
    mul(self, other) = {
        Add(self.0 + other.0)
    }
}
impl<X: AGroup + Copy> Act<X> for Add<X> {
    fn act(&self, other: X) -> X {
        self.0 + other
    }
}

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
    pub fn pair<S: FromStr, T: FromStr>(&mut self) -> (S, T) {
        let x = self.cin::<S>();
        let y = self.cin::<T>();
        (x, y)
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
