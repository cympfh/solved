#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let h: usize = sc.cin();
    let w: usize = sc.cin();
    let a: Vec<Vec<usize>> = (0..h).map(|_| sc.vec(w)).collect();

    // dp[i][j][k] = i-th 行目まで決めて (i-1) 行目まで孤立が無い,
    //               j = i-th 行目をフリップした ? 1 : 0,
    //               k = (i-1)-th 行目をフリップした ? 1 : 0,
    //               のときの最小フリップ数
    const INF: usize = 377_000;
    let mut dp = ndarray![INF; h, 2, 2];
    dp[0][0][0] = 0;
    dp[0][0][1] = 1;
    dp[0][1][0] = 1;
    dp[0][1][1] = 2;

    // i 行目に孤立がないことのチェック
    fn ok(a: &Vec<Vec<usize>>, i: usize, j: usize, k: usize, ell: usize) -> bool {
        let h = a.len();
        let w = a[0].len();
        let neigh = neighbor::Grid4(h, w);
        for w in 0..w {
            let mut ok = false;
            let x = (a[i][w] + k) % 2;
            for (i2, j2) in neigh.iter(i, w) {
                let y = match i2 {
                    _ if i2 == i + 1 => (a[i2][j2] + j) % 2,
                    _ if i2 == i => (a[i2][j2] + k) % 2,
                    _ if i2 == i - 1 => (a[i2][j2] + ell) % 2,
                    _ => panic!(),
                };
                if x == y {
                    ok = true;
                    break;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }

    for i in 1..h {
        // flip i
        for j in 0..2 {
            // flip (i-1)
            for k in 0..2 {
                // flip (i-2)
                for ell in 0..2 {
                    if ok(&a, i - 1, j, k, ell) {
                        dp[i][j][k] = min!(dp[i][j][k], dp[i - 1][k][ell] + j);
                    }
                }
            }
        }
    }

    let mut ans = INF;
    for j in 0..2 {
        for k in 0..2 {
            if ok(&a, h - 1, 0, j, k) {
                ans = min!(ans, dp[h - 1][j][k]);
            }
        }
    }
    if ans < INF {
        put!(ans);
    } else {
        put!(-1);
    }
}

// @misc/neighbor
/// Misc - Neighbor
pub mod neighbor {
    pub struct Grid4(pub usize, pub usize);
    impl Grid4 {
        pub fn iter(&self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            let mut v = vec![];
            for s in 0..3 {
                for t in 0..3 {
                    if (s + t) % 2 == 1
                        && (1..self.0 + 1).contains(&(i + s))
                        && (1..self.1 + 1).contains(&(j + t))
                    {
                        v.push((i + s - 1, j + t - 1));
                    }
                }
            }
            VecIter(v)
        }
    }
    pub struct Grid8(pub usize, pub usize);
    impl Grid8 {
        pub fn iter<'a>(&'a self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            let mut v = vec![];
            for s in 0..3 {
                for t in 0..3 {
                    if (s * t) != 1
                        && (1..self.0 + 1).contains(&(i + s))
                        && (1..self.1 + 1).contains(&(j + t))
                    {
                        v.push((i + s - 1, j + t - 1));
                    }
                }
            }
            VecIter(v)
        }
    }
    pub struct VecIter<T>(Vec<T>);
    impl<T: Copy> Iterator for VecIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
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
