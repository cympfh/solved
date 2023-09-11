#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum E {
    Portion(usize, u64),
    Teki(u64, usize, u64),
}
fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        let pa = sc.usize1();
        let ty: usize = sc.cin();
        let s: u64 = sc.cin();
        let gain: u64 = sc.cin();
        g[pa].push(if ty == 1 {
            E::Teki(s, i, gain)
        } else {
            E::Portion(i, gain)
        });
    }
    trace!(&g);

    let mut q = BinaryHeap::new();
    let mut cur = 1;
    let mut failed = false;
    q.push(Reverse(E::Portion(0, 1)));
    while let Some(Reverse(e)) = q.pop() {
        trace!(cur, &e);
        match e {
            E::Portion(i, gain) => {
                cur *= gain;
                if cur > 2_000_000_000 {
                    cur = 2_000_000_000;
                }
                for &h in g[i].iter() {
                    q.push(Reverse(h));
                }
            }
            E::Teki(s, i, gain) => {
                if cur < s {
                    failed = true;
                    break;
                }
                cur += gain;
                for &h in g[i].iter() {
                    q.push(Reverse(h));
                }
            }
        }
    }
    if failed {
        put!(#No);
    } else {
        put!(#Yes);
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
