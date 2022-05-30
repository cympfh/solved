#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn repeat(n: u128, p: usize, sup: u128) -> u128 {
    if n > sup {
        return sup + 10;
    }
    if p == 1 {
        n
    } else {
        let n2 = format!("{}{}", n, n).parse::<u128>().unwrap();
        if n2 > sup {
            return sup + 10;
        }
        let z = repeat(n2, p / 2, sup);
        if p % 2 == 0 {
            z
        } else {
            format!("{}{}", z, n).parse::<u128>().unwrap()
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let t: usize = sc.cin();
    let qs: Vec<u128> = sc.vec(t);

    for &q in qs.iter() {
        trace!(q);
        let mut ans = 11;
        for cycle in 2..18 {
            let mut left = 1;
            let mut right = 2_000_000_000;
            if repeat(left, cycle, q) > q {
                continue;
            }
            for _ in 0..2000 {
                let mid = (left + right) / 2;
                if repeat(mid, cycle, q) <= q {
                    left = mid;
                } else {
                    right = mid;
                }
                if left + 1 >= right {
                    break;
                }
            }
            ans = max!(ans, repeat(left, cycle, q));
        }
        put!(ans);
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
    (yes) => {println!("Yes")};
    (no) => {println!("No")};
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
