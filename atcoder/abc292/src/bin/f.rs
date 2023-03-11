#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

const TAU: f64 = 3.1415926536 * 2.0;
const TAU3: f64 = TAU / 3.0;

fn solve(a: f64, b: f64, arg: f64) -> f64 {
    let (w, h) = size(arg, 1.0);
    let r = min!(a / w, b / h);
    let ans = r * (3.0_f64).sqrt();
    ans
}

fn points(arg: f64, r: f64) -> Vec<(f64, f64)> {
    (0..3)
        .map(|i| {
            let theta = arg + TAU3 * i as f64;
            let x = r * theta.cos();
            let y = r * theta.sin();
            (x, y)
        })
        .collect()
}

fn size(arg: f64, r: f64) -> (f64, f64) {
    let ps = points(arg, r);
    let mut xmin = 2000.0;
    let mut xmax = -2000.0;
    let mut ymin = 2000.0;
    let mut ymax = -2000.0;
    for p in ps {
        xmin = min!(xmin, p.0);
        xmax = max!(xmax, p.0);
        ymin = min!(ymin, p.1);
        ymax = max!(ymax, p.1);
    }
    let width = xmax - xmin;
    let height = ymax - ymin;
    (width, height)
}

fn main() {
    let mut sc = Scanner::default();
    let a: f64 = sc.cin();
    let b: f64 = sc.cin();

    let mut left = -0.1;
    let mut right = TAU3 + 0.1;

    for _ in 0..100 {
        let t1 = (left + left + right) / 3.0;
        let t2 = (left + right + right) / 3.0;
        let z1 = solve(a, b, t1);
        let z2 = solve(a, b, t2);
        if z1 < z2 {
            left = t1;
        } else {
            right = t2;
        }
    }

    put!(solve(a, b, left));
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
