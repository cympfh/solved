#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let mut c = vec![0; 9];
    for i in 0..9 {
        c[i] = sc.cin();
    }
    let mut n = 0_i128;
    let mut m = 0_i128;
    for ps in Permutation::new(9) {
        let mut failed = false;
        for k in 0..3 {
            failed |= fa(
                c[0 + k * 3],
                c[1 + k * 3],
                c[2 + k * 3],
                ps[0 + k * 3],
                ps[1 + k * 3],
                ps[2 + k * 3],
            );
            failed |= fa(
                c[0 + k],
                c[3 + k],
                c[6 + k],
                ps[0 + k],
                ps[3 + k],
                ps[6 + k],
            );
        }
        failed |= fa(c[0], c[4], c[8], ps[0], ps[4], ps[8]);
        failed |= fa(c[2], c[4], c[6], ps[2], ps[4], ps[6]);
        if !failed {
            n += 1;
        }
        m += 1;
    }
    put!(n as f64 / m as f64);
}

fn fa(a: i32, b: i32, c: i32, index_a: usize, index_b: usize, index_c: usize) -> bool {
    let mut x = [(index_a, a), (index_b, b), (index_c, c)];
    x.sort();
    x[0].1 == x[1].1 && x[1].1 != x[2].1
}

// @num/iter/perm
/// Number - Iterator - Factorial Permutation (n!)
pub struct Permutation {
    n: usize,
    idx: usize,
    done: bool,
}
impl Permutation {
    pub fn new(n: usize) -> Permutation {
        Permutation {
            n,
            idx: 0,
            done: false,
        }
    }
    pub fn from(mut perm: Vec<usize>) -> Permutation {
        let n = perm.len();
        let mut idx = 0;
        let mut fact: usize = (1..n).product();
        for i in 0..n {
            if i > 0 {
                fact /= n - i;
            }
            idx += perm[i] * fact;
            for j in i + 1..n {
                if perm[j] > perm[i] {
                    perm[j] -= 1;
                }
            }
        }
        Permutation {
            n,
            idx,
            done: false,
        }
    }
    pub fn to_vec(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.n == 0 {
            self.done = true;
            return Some(vec![]);
        }
        let mut r = vec![0; self.n];
        let mut idx = self.idx;
        for k in 1..self.n {
            r[k] = idx % (k + 1);
            idx /= k + 1;
        }
        if idx > 0 {
            self.done = true;
            return None;
        }
        r.reverse();
        let mut b = vec![true; self.n];
        b[r[0]] = false;
        for k in 1..self.n {
            let mut count = 0;
            for j in 0..self.n {
                if b[j] {
                    if count == r[k] {
                        r[k] = j;
                        b[j] = false;
                        break;
                    }
                    count += 1;
                }
            }
        }
        Some(r)
    }
}
impl Iterator for Permutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let r = self.to_vec();
        self.idx += 1;
        r
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
