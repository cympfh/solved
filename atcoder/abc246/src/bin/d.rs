#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: u128 = sc.cin();

    let n3 = (0..=n).filter(|x| x * x * x >= n).next().unwrap(); // sqrt3(n)

    let mut ans = n * n;
    for a in 0..=n3 {
        let z = a * a * a + a * a + a;
        if z > ans {
            break;
        }
        let b = binsearch(0..n3, &|b| {
            a * a * a + a * a * b + a * b * b + b * b * b >= n
        });
        let x = a * a * a + a * a * b + a * b * b + b * b * b;
        ans = min!(ans, x);
    }
    put!(ans);
}

// @algorithm/binary_search
/// Algorithm - Binary Search
pub trait Integer
where
    Self: std::marker::Sized,
{
    fn close(range: std::ops::Range<Self>) -> bool;
    fn middle(range: std::ops::Range<Self>) -> Self;
}
macro_rules! define_integer {
    ($type:ty, $range:ident, $close_condition:expr, $middle_point:expr) => {
        impl Integer for $type {
            fn close($range: std::ops::Range<Self>) -> bool {
                $close_condition
            }
            fn middle($range: std::ops::Range<Self>) -> Self {
                $middle_point
            }
        }
    };
}
define_integer!(usize, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(u32, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(u64, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(u128, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(i32, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(i64, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(i128, r, r.start + 1 >= r.end, (r.start + r.end) / 2);
define_integer!(
    f32,
    r,
    r.start + 0.00000001 >= r.end,
    (r.start + r.end) / 2.0
);
define_integer!(
    f64,
    r,
    r.start + 0.00000001 >= r.end,
    (r.start + r.end) / 2.0
);

// the minimum index in range s.t. prop holds
pub fn binsearch<X: Integer + Copy>(range: std::ops::Range<X>, prop: &dyn Fn(X) -> bool) -> X {
    if prop(range.start) {
        range.start
    } else {
        let mut left = range.start;
        let mut right = range.end;
        while !X::close(left..right) {
            let mid = X::middle(left..right);
            if prop(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
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
    pub fn pair<S: FromStr, T: FromStr>(&mut self) -> (S, T) {
        let x = self.cin::<S>();
        let y = self.cin::<T>();
        (x, y)
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
