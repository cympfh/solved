#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let a: i64 = sc.cin();
    let b: i64 = sc.cin();
    let c: i64 = sc.cin();
    let d: i64 = sc.cin();
    let n: u64 = sc.cin();

    type State = ((i64, i64, i64, i64), (i64, i64, i64, i64), usize);
    let initial_state: State = ((a, b, c, d), (a, 0, 0, 0), 0);
    let step = |((a, b, c, d), (s, t, u, v), clock): State| {
        let clock2 = (clock + 1) % 4;
        if clock == 0 {
            let m = min!(s, b - t);
            ((a, b, c, d), (s - m, t + m, u, v), clock2)
        } else if clock == 1 {
            let m = min!(t, c - u);
            ((a, b, c, d), (s, t - m, u + m, v), clock2)
        } else if clock == 2 {
            let m = min!(u, d - v);
            ((a, b, c, d), (s, t, u - m, v + m), clock2)
        } else {
            let m = min!(v, a - s);
            ((a, b, c, d), (s + m, t, u, v - m), clock2)
        }
    };
    let (lambda, mu) = rho(initial_state, step);
    trace!(lambda, mu);

    let t = if n <= lambda {
        n
    } else {
        lambda + (n - lambda) % mu
    };
    let mut state = initial_state;
    for _ in 0..t {
        state = step(state);
    }
    trace!(state);

    put!(state.1 .0, state.1 .1, state.1 .2, state.1 .3);
}

// @algorithm/rho
/// Returns (lambda, mu), Steps before Loop, Length of Loop
pub fn rho<X: Eq + Copy + std::fmt::Debug, F: Fn(X) -> X>(initial_state: X, step: F) -> (u64, u64) {
    let mut x1 = initial_state;
    let mut x2 = initial_state;
    loop {
        x1 = step(x1);
        x2 = step(x2);
        x2 = step(x2);
        if x1 == x2 {
            break;
        }
    }
    let mut lambda = 0;
    let mut x2 = initial_state;
    while x1 != x2 {
        lambda += 1;
        x1 = step(x1);
        x2 = step(x2);
    }
    let mut mu = 0;
    while mu == 0 || x1 != x2 {
        mu += 1;
        x1 = step(x1);
        x2 = step(x2);
        x2 = step(x2);
    }
    (lambda, mu)
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
