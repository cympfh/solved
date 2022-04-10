#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let s: Vec<u64> = sc.vec(n);
    let t: Vec<u64> = sc.vec(n);

    let mut memo = vec![Inf; n];

    let mut q = BinaryHeap::new();
    for i in 0..n {
        q.push((Reverse(t[i]), i));
    }
    while let Some((Reverse(t), i)) = q.pop() {
        if memo[i] > Real(t) {
            memo[i] = Real(t);
            q.push((Reverse(t + s[i]), (i + 1) % n));
        }
    }

    for &a in memo.iter() {
        put!(a.unwrap());
    }
}

// @algebra/hyper
// @algebra/group
/// Algebra - Group (+, -, 0)
pub trait Group:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::iter::Sum
{
    fn zero() -> Self;
}
macro_rules! define_group {
    ($t:ty, $x:expr) => {
        impl Group for $t {
            fn zero() -> Self {
                $x
            }
        }
    };
}
define_group!(i32, 0);
define_group!(i64, 0);
define_group!(i128, 0);
define_group!(f32, 0.0);
define_group!(f64, 0.0);

/// Algebra - Hyper Numbers (numbers with infinity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hyper<X> {
    NegInf,
    Real(X),
    Inf,
}
use Hyper::{Inf, NegInf, Real};
impl<X> Hyper<X> {
    pub fn unwrap(self) -> X {
        if let Hyper::Real(x) = self {
            x
        } else {
            panic!()
        }
    }
}
impl<X: Group> std::ops::Add for Hyper<X> {
    type Output = Self;
    fn add(self, rhs: Hyper<X>) -> Hyper<X> {
        match (self, rhs) {
            (Real(x), Real(y)) => Real(x + y),
            (Inf, _) => Inf,
            (_, Inf) => Inf,
            _ => NegInf,
        }
    }
}
impl<X: Clone + Group> std::ops::AddAssign for Hyper<X> {
    fn add_assign(&mut self, rhs: Hyper<X>) {
        *self = (*self).clone() + rhs;
    }
}
impl<X: Group> std::ops::Sub for Hyper<X> {
    type Output = Self;
    fn sub(self, rhs: Hyper<X>) -> Hyper<X> {
        self + (-rhs)
    }
}
impl<X: Clone + Group> std::ops::SubAssign for Hyper<X> {
    fn sub_assign(&mut self, rhs: Hyper<X>) {
        *self = (*self).clone() - rhs;
    }
}
impl<X: Group> std::ops::Neg for Hyper<X> {
    type Output = Self;
    fn neg(self) -> Hyper<X> {
        match self {
            Inf => NegInf,
            NegInf => Inf,
            Real(x) => Real(-x),
        }
    }
}
impl<X: Group> std::iter::Sum for Hyper<X> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Hyper::zero(), std::ops::Add::add)
    }
}
impl<X: Group> Group for Hyper<X> {
    fn zero() -> Self {
        Hyper::Real(X::zero())
    }
}
impl<X: Group> std::ops::Add<X> for Hyper<X> {
    type Output = Self;
    fn add(self, other: X) -> Self {
        match self {
            Inf => Inf,
            NegInf => NegInf,
            Real(x) => Real(x + other),
        }
    }
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;

struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
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
