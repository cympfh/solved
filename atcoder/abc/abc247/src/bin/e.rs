#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();

    let x: i64 = sc.cin();
    let y: i64 = sc.cin();

    let a: Vec<i64> = sc.vec(n);
    let minx = SegmentTree::from(
        a.iter()
            .map(|&a| Some((a, a)))
            .chain(vec![None].iter().cloned())
            .collect(),
    );

    let mut ans = 0;
    for left in 0..n {
        let (mx, mn) = minx.product(left..n).unwrap();
        if mx < x || mn > y {
            break;
        }
        let r1 = binsearch(left + 1..n + 1, &|i| {
            let (mx, mn) = minx.product(left..i).unwrap();
            mx >= x && mn <= y
        });
        let r2 = binsearch(r1..n + 1, &|i| {
            let (mx, mn) = minx.product(left..i).unwrap();
            mx > x || mn < y
        });
        let range = (r1, r2);
        if range.0 < range.1 {
            ans += range.1 - range.0;
        }
    }
    put!(ans);
}

// the minimum index in range s.t. prop holds
pub fn binsearch(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        range.start
    } else {
        let mut left = range.start;
        let mut right = range.end;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if prop(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    }
}

fn mul(a: Option<(i64, i64)>, b: Option<(i64, i64)>) -> Option<(i64, i64)> {
    match (a, b) {
        (None, _) => b,
        (_, None) => a,
        (Some((x, y)), Some((u, v))) => Some((x.max(u), y.min(v))),
    }
}

#[derive(Debug, Clone)]
pub struct SegmentTree {
    length: usize,       // of leaves
    length_upper: usize, // power of 2
    size: usize,         // of nodes
    data: Vec<Option<(i64, i64)>>,
}
impl SegmentTree {
    pub fn new(length: usize) -> Self {
        let mut length_upper = 1;
        while length_upper < length {
            length_upper <<= 1
        }
        let size = length_upper * 2 - 1;
        let data = vec![None; size];
        SegmentTree {
            length,
            length_upper,
            size,
            data,
        }
    }
    pub fn from(xs: Vec<Option<(i64, i64)>>) -> Self {
        let mut tree = Self::new(xs.len());
        for i in 0..xs.len() {
            tree.data[tree.size / 2 + i] = xs[i];
        }
        for i in (0..tree.size / 2).rev() {
            tree.data[i] = mul(tree.data[2 * i + 1], tree.data[2 * i + 2]);
        }
        tree
    }
    fn product_sub(
        &self,
        range: std::ops::Range<usize>,
        u: usize,
        focus: std::ops::Range<usize>,
    ) -> Option<(i64, i64)> {
        if focus.end <= range.start || range.end <= focus.start {
            None
        } else if range.start <= focus.start && focus.end <= range.end {
            self.data[u]
        } else {
            let mid = (focus.start + focus.end) / 2;
            let a = self.product_sub(range.clone(), u * 2 + 1, focus.start..mid);
            let b = self.product_sub(range.clone(), u * 2 + 2, mid..focus.end);
            mul(a, b)
        }
    }
    pub fn product(&self, range: std::ops::Range<usize>) -> Option<(i64, i64)> {
        self.product_sub(range, 0, 0..self.length_upper)
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
