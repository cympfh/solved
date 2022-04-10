#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn solve2(x: u64, y: u64, a: u64, b: u64) -> bool {
    let r = div(a, x) * x;
    if r + b <= x * y {
        return true;
    }
    let r = div(a, y) * y;
    if r + b <= x * y {
        return true;
    }
    false
}

fn solve(x: u64, y: u64, a: u64, b: u64, c: u64) -> bool {
    if x * y < a + b + c {
        return false;
    }
    sub(x, y, a, b, c)
        || sub(y, x, a, b, c)
        || sub(x, y, b, a, c)
        || sub(y, x, b, a, c)
        || sub(x, y, c, a, b)
        || sub(y, x, c, a, b)
}

fn div(x: u64, y: u64) -> u64 {
    (x + y - 1) / y
}

fn sub(x: u64, y: u64, a: u64, b: u64, c: u64) -> bool {
    let ha = div(a, x);
    let hb = div(b, x);
    let hc = div(c, x);
    if ha + hb + hc <= y {
        trace!("STUCK", (x, y), (a, b, c), (ha, hb, hc));
        return true;
    }

    if y > ha {
        let h = y - ha;
        let wb = div(b, h);
        let wc = div(c, h);
        if wb + wc <= x {
            trace!("T", (x, y), (a, b, c), (h, wb, wc));
            return true;
        }
    }

    false
}

fn main() {
    assert!(div(5, 3) == 2);
    assert!(div(6, 3) == 2);
    assert!(div(7, 3) == 3);
    assert!(div(8, 3) == 3);
    assert!(div(9, 3) == 3);
    let mut sc = Scanner::new();
    let x: u64 = sc.cin();
    let y: u64 = sc.cin();
    let a: u64 = sc.cin();
    let b: u64 = sc.cin();
    let c: u64 = sc.cin();
    if solve(x, y, a, b, c) {
        put!("Yes");
    } else {
        put!("No");
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
