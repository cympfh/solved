#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let a: (i128, i128) = sc.pair();
    let b: (i128, i128) = sc.pair();
    let c: (i128, i128) = sc.pair();
    let d: (i128, i128) = sc.pair();

    let a = IntPoint(a.0, a.1);
    let b = IntPoint(b.0, b.1);
    let c = IntPoint(c.0, c.1);
    let d = IntPoint(d.0, d.1);

    if (b - a).cross(c - b) > 0
        && (c - b).cross(d - c) > 0
        && (d - c).cross(a - d) > 0
        && (a - d).cross(b - a) > 0
    {
        put!(#Yes);
    } else {
        put!(#No);
    }
}

// @geometry2d/int/line
// @geometry2d/int/point
/// Geometry2D/Int - Definition of Point
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntPoint(pub i128, pub i128);

impl IntPoint {
    fn cross(&self, other: IntPoint) -> i128 {
        self.0 * other.1 - self.1 * other.0
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

/// Geometry2D/Int - Definition of Line
pub struct IntLine(pub IntPoint, pub IntPoint);

impl IntLine {
    pub fn contains(&self, p: IntPoint) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

#[macro_export]
macro_rules! iline {
    ($x0:expr, $y0:expr; $x1:expr, $y1:expr) => {
        IntLine(IntPoint($x0, $y0), IntPoint($x1, $y1))
    };
    ($a:expr; $b:expr) => {
        IntLine($a, $b)
    };
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
