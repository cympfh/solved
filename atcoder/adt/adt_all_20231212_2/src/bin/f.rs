#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn add(a: usize, b: i32) -> i32 {
    a as i32 + b
}
fn limit(x: i32, left: usize, right: usize) -> Option<usize> {
    if left as i32 <= x && x < right as i32 {
        Some(x as usize)
    } else {
        None
    }
}

fn main() {
    let mut sc = Scanner::default();
    let ha: usize = sc.cin();
    let wa: usize = sc.cin();
    let a: Vec<Vec<char>> = (0..ha).map(|_| sc.chars()).collect();
    let hb: usize = sc.cin();
    let wb: usize = sc.cin();
    let b: Vec<Vec<char>> = (0..hb).map(|_| sc.chars()).collect();
    let hc: usize = sc.cin();
    let wc: usize = sc.cin();
    let c: Vec<Vec<char>> = (0..hc).map(|_| sc.chars()).collect();

    for dx_a in -10_i32..=10 {
        for dy_a in -10_i32..=10 {
            for dx_b in -10_i32..=10 {
                for dy_b in -10_i32..=10 {
                    let mut failed = false;
                    let mut d: Vec<Vec<char>> = ndarray!['.'; hc, wc];
                    for x in 0..ha {
                        for y in 0..wa {
                            if a[x][y] == '.' {
                                continue;
                            }
                            if let (Some(x2), Some(y2)) =
                                (limit(add(x, dx_a), 0, hc), limit(add(y, dy_a), 0, wc))
                            {
                                d[x2][y2] = '#';
                            } else {
                                failed = true;
                            }
                        }
                    }
                    for x in 0..hb {
                        for y in 0..wb {
                            if b[x][y] == '.' {
                                continue;
                            }
                            if let (Some(x2), Some(y2)) =
                                (limit(add(x, dx_b), 0, hc), limit(add(y, dy_b), 0, wc))
                            {
                                d[x2][y2] = '#';
                            } else {
                                failed = true;
                            }
                        }
                    }
                    if failed {
                        continue;
                    }
                    if c == d {
                        put!(#Yes);
                        return;
                    }
                }
            }
        }
    }
    put!(#No);
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
    ($x:expr, $($ys:expr),* $(,)*) => {{
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
    ($x:expr, $($ys:expr),* $(,)*) => {{
        let t = max!($($ys),*);
        if $x > t { $x } else { t }
    }}
}
#[macro_export]
macro_rules! trace {
    (# $a:ident $(,)? $(;)? $($xs:expr),* $(,)? ) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}] {} = {:?}", stringify!($a), stringify!($($xs),*), ($($xs),*))
    };
    ($($xs:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($($xs),*), ($($xs),*))
    };
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
/// Array-indexing by i64.
/// (Vec<Vec<..<T>..>>; i64, i64, ..) => Option<T>
#[macro_export]
macro_rules! at {
    ($s:expr;) => { Some($s) };
    ($s:expr; $idx:expr $(,$args:expr)* $(,)?) => {
        if 0 <= $idx {
            let idx_usize = $idx as usize;
            if idx_usize < $s.len() {
                at!($s[idx_usize]; $($args),*)
            } else {
                None
            }
        } else {
            None
        }
    }
}
// }}}
