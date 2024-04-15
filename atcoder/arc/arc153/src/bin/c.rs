#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let a: Vec<i64> = sc.vec(n);
    let mut acc = a.clone();
    for i in (0..n - 1).rev() {
        acc[i] += acc[i + 1];
    }
    trace!(&acc);
    let mut d = vec![1; n];
    d[0] = 0;
    let mut sum = (0..n).map(|i| acc[i] * d[i]).sum::<i64>();
    if sum != 0 {
        if acc[0] != 0 {
            let k = sum / acc[0];
            d[0] -= k;
            sum -= k * acc[0];
        }
    }
    for i in 1..n {
        if sum == 0 {
            break;
        }
        if acc[i] == 0 {
            continue;
        }
        if sum < 0 && acc[i] > 0 {
            let k = -sum / acc[i];
            d[i] += k;
            sum += k * acc[i];
        } else if sum > 0 && acc[i] < 0 {
            let k = sum / (-acc[i]);
            d[i] += k;
            sum += k * acc[i];
        }
        assert!(d[i] > 0);
    }
    if sum != 0 {
        if acc[0] != 0 {
            let k = sum / acc[0];
            d[0] -= k;
            sum -= k * acc[0];
        }
    }
    trace!(sum);
    trace!(&d);
    if sum == 0 {
        let mut xs = vec![0; n];
        xs[0] = d[0];
        for i in 1..n {
            xs[i] = xs[i - 1] + d[i];
        }
        put!(#Yes);
        put!(..xs);
    } else {
        put!(#No);
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
