#![allow(unused_imports, unused_macros, dead_code)]
use std::collections::btree_set::Union;
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let s = IntPoint::new(sc.cin(), sc.cin());
    let t = IntPoint::new(sc.cin(), sc.cin());
    let cs: Vec<IntCircle> = (0..n)
        .map(|_| {
            let x: i128 = sc.cin();
            let y: i128 = sc.cin();
            let r: i128 = sc.cin();
            IntCircle::new(IntPoint::new(x, y), r)
        })
        .collect();
    let mut si = n + 2;
    let mut ti = n + 2;
    for i in 0..n {
        if cs[i].include(s) == IntCircleInlude::On {
            si = i;
        }
        if cs[i].include(t) == IntCircleInlude::On {
            ti = i;
        }
    }
    assert!(si < n);
    assert!(ti < n);
    let mut uf = UnionFind::new(n);
    use IntCircleIntersection::*;
    for i in 0..n {
        for j in i + 1..n {
            match cs[i].intersection(&cs[j]) {
                Equal | Intersect(1) | Intersect(2) => {
                    uf.merge(i, j);
                }
                _ => {}
            }
        }
    }
    if uf.is_same(si, ti) {
        put!(#Yes);
    } else {
        put!(#No);
    }
}

// @set/union_find
/// Set - Union-Find
#[derive(Debug, Clone)]
pub struct UnionFind {
    data: Vec<UF>,
}

#[derive(Debug, Clone)]
enum UF {
    Root(usize),
    Child(usize),
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            data: vec![UF::Root(1); n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        match self.data[x] {
            UF::Root(_) => x,
            UF::Child(parent) => {
                let root = self.root(parent);
                self.data[x] = UF::Child(root);
                root
            }
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        match self.data[r] {
            UF::Root(size) => size,
            UF::Child(_) => 0,
        }
    }
    pub fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            let size_x = self.size(root_x);
            let size_y = self.size(root_y);
            let (i, j) = if size_x > size_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };
            self.data[i] = UF::Root(size_x + size_y);
            self.data[j] = UF::Child(i);
        }
    }
}

// @geometry2d/int/point
/// Geometry2D/Int - Definition of Point
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntPoint(pub i128, pub i128);

impl IntPoint {
    pub fn new(x: i128, y: i128) -> Self {
        Self(x, y)
    }
    pub fn quadrance(&self) -> i128 {
        self.0.pow(2) + self.1.pow(2)
    }
}

impl std::ops::Add<IntPoint> for IntPoint {
    type Output = Self;
    fn add(self, other: IntPoint) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Sub<IntPoint> for IntPoint {
    type Output = Self;
    fn sub(self, other: IntPoint) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl std::ops::Mul<i128> for IntPoint {
    type Output = Self;
    fn mul(self, k: i128) -> Self {
        Self(self.0 * k, self.1 * k)
    }
}
impl std::ops::Div<i128> for IntPoint {
    type Output = Self;
    fn div(self, k: i128) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

/// Geometry2D/Int - Definition of Circle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntCircle {
    pub center: IntPoint,
    pub radius: i128,
}
impl IntCircle {
    pub fn new(center: IntPoint, radius: i128) -> Self {
        assert!(radius >= 0);
        Self { center, radius }
    }
}

/// IntCircle vs IntPoint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntCircleInlude {
    Inner,
    Outer,
    On,
}

impl IntCircle {
    pub fn include(&self, p: IntPoint) -> IntCircleInlude {
        let d2 = (self.center - p).quadrance();
        let r2 = self.radius.pow(2);
        if d2 == r2 {
            IntCircleInlude::On
        } else if d2 > r2 {
            IntCircleInlude::Outer
        } else {
            IntCircleInlude::Inner
        }
    }
}

/// IntCircle vs IntCircle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntCircleIntersection {
    Equal,
    Sub,              // self is contained by the other
    Sup,              // self contains the other
    Intersect(usize), // intersection with `n` points
}

impl IntCircle {
    pub fn intersection(&self, other: &IntCircle) -> IntCircleIntersection {
        use IntCircleIntersection::*;
        let d2 = (self.center - other.center).quadrance();
        let r2 = (self.radius + other.radius).pow(2);
        let l2 = (self.radius - other.radius).pow(2);
        if self == other {
            Equal
        } else if d2 == r2 {
            Intersect(1)
        } else if d2 < l2 && self.radius < other.radius {
            Sub
        } else if d2 < l2 && self.radius > other.radius {
            Sup
        } else if d2 < r2 {
            Intersect(2)
        } else {
            Intersect(0)
        }
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
