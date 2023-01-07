#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

const MOD: u64 = 1_000_000_007;
const X: u64 = 511111;
const INV: u64 = 866304977;

fn powmod(x: u64, n: usize, modulo: u64) -> u64 {
    if n == 0 {
        1
    } else {
        let x2 = (x * x) % modulo;
        let a = powmod(x2, n / 2, modulo) % modulo;
        if n % 2 == 0 {
            a
        } else {
            (a * x) % modulo
        }
    }
}

#[derive(Debug, Clone)]
struct StringHash {
    length: usize,
    sethash: u64,
    strhash: u64,
    xpow: u64,
    table: Vec<u64>,
}

impl StringHash {
    fn code(&self) -> (usize, u64) {
        (self.length, self.strhash)
    }
    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
    fn new() -> Self {
        let length = 0;
        let sethash = 0;
        let strhash = 0;
        let xpow = 1;
        let mut rng = XorShift::new();
        let table: Vec<u64> = (0..300).map(|_| rng.gen::<u64>() % (MOD + 100)).collect();
        Self {
            length,
            sethash,
            strhash,
            xpow,
            table,
        }
    }
    fn push_front(&mut self, c: char) {
        let x = c as usize;
        self.sethash ^= self.table[x];
        self.strhash = (self.table[x] + (self.strhash * X) % MOD) % MOD;
        self.xpow = (self.xpow * X) % MOD;
        self.length += 1;
    }
    fn push_back(&mut self, c: char) {
        let x = c as usize;
        self.sethash ^= self.table[x];
        self.strhash = (self.strhash + (self.table[x] * self.xpow) % MOD) % MOD;
        self.length += 1;
        self.xpow = (self.xpow * X) % MOD;
    }

    fn pop_front(&mut self, c: char) {
        let x = c as usize;
        self.sethash ^= self.table[x];
        self.strhash = ((self.strhash + MOD - self.table[x] % MOD) * INV) % MOD;
        self.length -= 1;
        self.xpow = (self.xpow * INV) % MOD;
    }
    fn pop_back(&mut self, c: char) {
        let x = c as usize;
        self.length -= 1;
        self.xpow = (self.xpow * INV) % MOD;
        self.sethash ^= self.table[x];
        self.strhash = (self.strhash + MOD - ((self.table[x] % MOD) * self.xpow % MOD)) % MOD;
    }
    fn concat(&self, other: &Self) -> Self {
        let mut r = self.clone();
        r.length = self.length + other.length;
        r.sethash = self.sethash ^ other.sethash;
        r.strhash = (self.strhash + (other.strhash * self.xpow) % MOD) % MOD;
        r
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let t = sc.chars();

    let mut u1 = StringHash::new();
    let mut u2 = StringHash::new();
    let mut v = StringHash::new();

    for i in 0..n {
        u1.push_back(t[i]);
        v.push_back(t[n * 2 - i - 1]);
    }
    if v.eq(&u1.concat(&u2)) {
        let ans: String = (0..n).map(|i| t[i]).collect();
        put!(ans);
        put!(n);
        return;
    }
    for k in 0..n {
        u1.pop_back(t[n - k - 1]);
        u2.push_front(t[n * 2 - k - 1]);
        v.pop_front(t[n * 2 - k - 1]);
        v.push_back(t[n - k - 1]);
        if v.eq(&u1.concat(&u2)) {
            for i in (n - k - 1..n - k - 1 + n).rev() {
                print!("{}", t[i]);
            }
            println!();
            put!(n - k - 1);
            return;
        }
    }
    put!(-1);
}

// @num/random/xorshift
// @num/random/fromu64
/// Number - Utility - FromU64
pub trait FromU64 {
    fn coerce(x: u64) -> Self;
}
impl FromU64 for u64 {
    fn coerce(x: u64) -> Self {
        x
    }
}
macro_rules! define_fromu64 {
    ($ty:ty) => {
        impl FromU64 for $ty {
            fn coerce(x: u64) -> Self {
                x as $ty
            }
        }
    };
}
define_fromu64!(usize);
define_fromu64!(u32);
define_fromu64!(u128);
define_fromu64!(i32);
define_fromu64!(i64);
define_fromu64!(i128);
impl FromU64 for bool {
    fn coerce(x: u64) -> Self {
        x % 2 == 0
    }
}
impl FromU64 for f32 {
    fn coerce(x: u64) -> Self {
        (x as f32) / (std::u64::MAX as f32)
    }
}
impl FromU64 for f64 {
    fn coerce(x: u64) -> Self {
        (x as f64) / (std::u64::MAX as f64)
    }
}

/// Random Number - Xor-Shift Algorithm
pub struct XorShift(u64);
impl XorShift {
    pub fn new() -> Self {
        XorShift(88_172_645_463_325_252)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x = x ^ (x << 13);
        x = x ^ (x >> 7);
        x = x ^ (x << 17);
        self.0 = x;
        x
    }
    pub fn gen<T: FromU64>(&mut self) -> T {
        T::coerce(self.next())
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
