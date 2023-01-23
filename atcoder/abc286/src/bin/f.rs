#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let p = prime_sieve(100);
    let primes: Vec<usize> = (2..100).filter(|&i| p[i]).collect();
    let mut p = vec![];
    {
        let mut sum = 0;
        for i in 0..primes.len() {
            let q = if primes[i] <= 3 {
                primes[i] * primes[i]
            } else {
                primes[i]
            };
            if sum + q > 110 {
                break;
            }
            sum += q;
            p.push(q);
        }
    }
    let mut a = vec![];
    {
        let mut offset = 0;
        for &q in p.iter() {
            a.push(offset + q);
            for i in 1..q {
                a.push(offset + i);
            }
            offset += q;
        }
    }
    put!(a.len());
    put!(..a);
    flush();

    let mut sc = Scanner::default();
    let b: Vec<usize> = sc.vec(a.len());

    let mut rms = vec![];
    {
        let mut offset = 0;
        for &q in p.iter() {
            let r = (offset + q + 1 - b[offset]) % q;
            rms.push((r as i128, q as i128));
            offset += q;
        }
    }
    if let Some((x, _)) = crt(&rms) {
        put!(x);
    } else {
        panic!();
    }
}

pub fn gcd_ex(x: i128, y: i128) -> (i128, i128, i128) {
    if y == 0 {
        (1, 0, x)
    } else {
        let (p, q, g) = gcd_ex(y, x % y);
        (q, p - q * (x / y), g)
    }
}

/// Number Theory - Chinese Remainder Theorem (CRT)
/// Solve of x = r[i] mod m[i] => x = y mod z
/// - Args:
///     - rm: &Vec of pair (r[i], m[i])
/// - Returns Some(y, z)
pub fn crt(rm: &[(i128, i128)]) -> Option<(i128, i128)> {
    let mut r0 = 0;
    let mut m0 = 1;
    for &(r, m) in rm.iter() {
        let (p, _, d) = gcd_ex(m0, m);
        if (r - r0) % d != 0 {
            return None;
        }
        let tmp = (r - r0) / d * p % (m / d);
        r0 += m0 * tmp;
        m0 *= m / d;
    }
    while r0 < 0 {
        r0 += m0
    }
    Some((r0, m0))
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
