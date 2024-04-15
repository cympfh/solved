#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

const MX: usize = 1_000_000;

fn main() {
    let mut sc = Scanner::default();
    let sx: i64 = sc.cin();
    let sy: i64 = sc.cin();
    let tx: i64 = sc.cin();
    let ty: i64 = sc.cin();
    let a: i64 = sc.cin();
    let b: i64 = sc.cin();
    let c: i64 = sc.cin();
    let d: i64 = sc.cin();
    trace!((sx, sy), (tx, ty));
    trace!(a, b, c, d);

    if (sx, sy) == (tx, ty) {
        trace!("Equal case");
        put!(#Yes);
        return;
    }

    if (sx + tx) % 2 == 1 || (sy + ty) % 2 == 1 {
        trace!("Odd case");
        put!(#No);
        return;
    }

    // 奇数
    let dx = (tx + sx) / 2;
    let dy = (ty + sy) / 2;
    if let (Some(vx), Some(vy)) = (omake(dx, a, b), omake(dy, c, d)) {
        let mut vx = vx;
        let mut vy = vy;
        trace!(vx.len(), vy.len());
        while vx.len() < vy.len() {
            vx.push(a);
            vx.push(a);
        }
        while vx.len() > vy.len() {
            vy.push(c);
            vy.push(c);
        }
        trace!(vx.len(), vy.len());
        assert!(vx.len() <= MX);
        put!(#Yes);
        let mut cur = (sx, sy); // simulation
        for (x, y) in vx.into_iter().zip(vy.into_iter()).rev() {
            put!(x, y);
            cur = (2 * x - cur.0, 2 * y - cur.1);
        }
        assert!(cur == (tx, ty));
        trace!(&cur);
        return;
    }

    // 偶数で作る
    let dx = (tx - sx) / 2;
    let dy = (ty - sy) / 2;
    if let (Some(vx), Some(vy)) = (make(dx, a, b), make(dy, c, d)) {
        let mut vx = vx;
        let mut vy = vy;
        while vx.len() < vy.len() {
            vx.push(a);
            vx.push(a);
        }
        while vx.len() > vy.len() {
            vy.push(c);
            vy.push(c);
        }
        if vx.len() > MX {
            put!(#No);
            return;
        }
        trace!(vx.len(), vy.len());
        assert!(vx.len() <= MX);
        put!(#Yes);
        let mut cur = (sx, sy); // simulation
        for (x, y) in vx.into_iter().zip(vy.into_iter()).rev() {
            put!(x, y);
            cur = (2 * x - cur.0, 2 * y - cur.1);
        }
        assert!(cur == (tx, ty));
        trace!(&cur);
        return;
    }
    put!(#No);
}

// 適用順序は逆
fn make(dx: i64, a: i64, b: i64) -> Option<Vec<i64>> {
    if dx == 0 {
        return Some(vec![]);
    }
    if dx < 0 {
        if let Some(r) = make(-dx, a, b) {
            return Some(r.into_iter().rev().collect());
        } else {
            return None;
        }
    }
    if dx > 0 && a == b {
        return None;
    }
    let mut r = vec![];
    let mut dx = dx;
    while dx > b - a {
        r.push(b);
        r.push(a);
        dx -= b - a;
    }
    if dx > 0 {
        r.push(a + dx);
        r.push(a);
    }
    Some(r)
}

// 適用順序は逆
fn omake(dx: i64, a: i64, b: i64) -> Option<Vec<i64>> {
    for z in a..=b {
        if let Some(vx) = make(dx - z, a, b) {
            trace!((dx, a, b), (z, &vx));
            let mut vx = vx;
            vx.push(z);
            return Some(vx);
        }
    }
    return None;
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
