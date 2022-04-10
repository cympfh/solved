#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

// @num/prime/factorize
/// Number - Prime Factorization
pub fn factorize(n: u64) -> Vec<(u64, usize)> {
    let mut m = n;
    let mut r = vec![];
    for x in 2..=n {
        if m == 1 {
            break;
        }
        if x * x > n {
            r.push((m, 1));
            break;
        }
        if n % x != 0 {
            continue;
        }
        let mut c = 0;
        while m % x == 0 {
            m /= x;
            c += 1;
        }
        if c > 0 {
            r.push((x, c));
        }
    }
    r
}

fn main() {
    let ps = prime_sieve(100_100);
    let ps: Vec<u64> = (2..100_100usize)
        .filter(|&i| ps[i])
        .map(|i| i as u64)
        .collect();

    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let xs: Vec<u64> = sc.vec(n);

    let mut divs = BTreeSet::new();
    for &x in xs.iter() {
        for &(p, _) in factorize(x).iter() {
            divs.insert(p);
        }
    }
    // trace!(&divs);

    let mut memo = vec![true; 100_100];
    for &p in divs.iter() {
        for x in 1..=m {
            let xp = x * p as usize;
            if xp <= m {
                memo[xp] = false;
            }
        }
    }
    let a: Vec<usize> = (1..=m).filter(|&i| memo[i]).collect();
    put!(a.len());
    for &x in a.iter() {
        put!(x);
    }
}

// @num/prime/sieve
/// Prime Numbers - Sieve of Eratosthenes
pub fn prime_sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n];
    s[0] = false;
    s[1] = false;
    for i in 2..n {
        if i * i > n {
            break;
        }
        if s[i] {
            for k in 2..(n + i - 1) / i {
                s[k * i] = false
            }
        }
    }
    s
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
// }}}
