#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let xs: Vec<u64> = sc.vec(n);

    let m = xs.iter().filter(|&&x| x > 0).count();
    if m <= 2 {
        put!("Yes");
        return;
    }

    let mini = *xs.iter().filter(|&&x| x > 0).min().unwrap();
    let maxi = *xs.iter().filter(|&&x| x > 0).max().unwrap();
    trace!(mini, maxi);

    if mini == maxi {
        put!("Yes");
        return;
    }

    let mut zs = vec![];
    for &x in xs.iter().filter(|&&x| x >= mini) {
        let z = x - mini;
        zs.push(z);
    }
    trace!(&zs);

    let mut g = zs[0];
    for i in 1..zs.len() {
        g = gcd(g, zs[i]);
    }
    trace!(g);

    let mut ans = "Yes";
    let mut used = vec![false; n];
    for &z in zs.iter() {
        let idx = (z / g) as usize;
        if idx < n && !used[idx] {
            used[idx] = true;
        } else {
            ans = "No";
            break;
        }
    }
    put!(ans);
}

// @num/gcd
// @num/base
pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
pub trait Num:
    Copy
    + Eq
    + Ord
    + Zero
    + One
    + std::marker::Sized
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::ops::Sub<Output = Self>
    + std::ops::SubAssign
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
{
}
pub trait Int: Num {}
pub trait Nat: Num {}

macro_rules! define_zero_one {
    ($ty:ty, $zero:expr, $one:expr) => {
        impl Zero for $ty {
            fn zero() -> Self {
                $zero
            }
        }
        impl One for $ty {
            fn one() -> Self {
                $one
            }
        }
    };
}

define_zero_one!(usize, 0, 1);
define_zero_one!(u32, 0, 1);
define_zero_one!(u64, 0, 1);
define_zero_one!(u128, 0, 1);
define_zero_one!(i32, 0, 1);
define_zero_one!(i64, 0, 1);
define_zero_one!(i128, 0, 1);
define_zero_one!(f32, 0.0, 1.0);
define_zero_one!(f64, 0.0, 1.0);

impl Num for usize {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}
impl Nat for usize {}
impl Nat for u32 {}
impl Nat for u64 {}
impl Nat for u128 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}

/// Number - GCD on Natural Numbers
pub fn gcd<N: Nat>(a: N, b: N) -> N {
    if b == N::zero() {
        a
    } else {
        gcd(b, a % b)
    }
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
