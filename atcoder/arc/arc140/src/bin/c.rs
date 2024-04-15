#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let x: usize = sc.cin();

    let mut max = 0;
    let mut ans = vec![];

    for parity in 0..2 {
        let base0 = if n / 2 <= 50 { 1 } else { n / 2 - 50 };
        let base1 = if n / 2 + 50 > n { n } else { n / 2 + 50 };
        for base in base0..=base1 {
            let mut phase = parity == 0;
            // trace!(parity, base);
            let mut r = vec![x];
            let mut left = base;
            let mut right = base + 1;
            loop {
                // trace!(phase, (left, right), &r);
                if phase {
                    while left == x {
                        left -= 1;
                    }
                    if left >= 1 {
                        r.push(left);
                        left -= 1;
                    }
                } else {
                    while right == x {
                        right += 1;
                    }
                    if right <= n {
                        r.push(right);
                        right += 1;
                    }
                }
                phase ^= true;
                if left < 1 && right > n {
                    break;
                }
            }
            let len = f(&r);
            if len > max {
                trace!(&r, len);
                max = len;
                ans = r;
            }
        }
    }

    put!(..ans);
}

// @sequence/lis
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

fn udiff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn f(xs: &Vec<usize>) -> usize {
    let n = xs.len();
    let mut bots = vec![];
    for i in 1..n {
        let x = udiff(xs[i], xs[i - 1]);
        let i = if bots.is_empty() {
            0
        } else {
            binsearch(0..bots.len(), &|i| bots[i] >= x)
        };
        if i == bots.len() {
            bots.push(x);
        } else {
            bots[i] = x;
        }
    }
    bots.len()
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
    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
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
