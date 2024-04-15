#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let k: u128 = sc.cin();

    // Simple Solution
    {
        let mut k = k;
        for p in 1..=2_000_000 {
            k /= gcd(k, p);
            if k == 1 {
                put!(p);
                return;
            }
        }
        put!(k);
        return;
    }

    // Another Solution
    {
        let mut ans = 1;
        for (p, num) in factorize(k) {
            let n = phiinv(p, num as u128);
            ans = max!(ans, n);
        }
        put!(ans);
    }
}

// @num/prime/factorize
/// Number - Prime Factorization
pub fn factorize(n: u128) -> Vec<(u128, usize)> {
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

pub fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// (n!) が持つ素因数 k の個数
fn phi(n: u128, k: u128) -> u128 {
    let mut r = 0;
    let mut n = n;
    while n > 0 {
        n /= k;
        r += n;
    }
    r
}

/// (n!) が素因数 p を num 個以上持つ最小の n
/// p is prime
fn phiinv(p: u128, num: u128) -> u128 {
    if num == 0 {
        1
    } else if num == 1 {
        p
    } else {
        let mut left = 1;
        let mut right = 1;
        while phi(right, p) < num {
            right *= 2;
        }
        for _ in 0..100 {
            let mid = (left + right) / 2;
            if phi(mid, p) < num {
                left = mid;
            } else {
                right = mid;
            }
        }
        right
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
