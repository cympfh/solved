#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let ps: Vec<_> = (0..n)
        .map(|_| {
            let x: i128 = sc.cin();
            let y: i128 = sc.cin();
            IntPoint(x, y)
        })
        .collect();
    let mut cx = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                let p0 = ps[i];
                let p1 = ps[j];
                let p2 = ps[k];
                let u = p1 - p0;
                let v = p2 - p0;
                let det = u.0 * v.1 - u.1 * v.0;
                if det != 0 {
                    cx += 1
                }
            }
        }
    }
    put!(cx);
}

// @geometry2d/int/point
/// Geometry2D/Int - Definition of Point
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntPoint(pub i128, pub i128);

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
