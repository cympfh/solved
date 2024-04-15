#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn count_rect(x: usize, y: usize, n: usize, sum_p: usize, pacc: &Vec<Vec<usize>>) -> usize {
    let mut r = 0;
    r += sum_p * (x / n) * (y / n);
    r += pacc[x % n][n] * (y / n);
    r += pacc[n][y % n] * (x / n);
    r += pacc[x % n][y % n];
    r
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let q: usize = sc.cin();
    let p: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            let line = sc.chars();
            line.into_iter()
                .map(|c| if c == 'B' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let sum_p = {
        let mut r = 0;
        for i in 0..n {
            for j in 0..n {
                r += p[i][j];
            }
        }
        r
    };

    // 横方向の累積和
    let mut p_yoko: Vec<Vec<usize>> = ndarray![0; n, n+1];
    for i in 0..n {
        for j in 0..n {
            p_yoko[i][j + 1] = p_yoko[i][j] + p[i][j];
        }
    }

    // 次に縦方向に累積
    let mut pacc: Vec<Vec<usize>> = ndarray![0; n+1, n+1];
    for i in 0..n {
        for j in 0..=n {
            pacc[i + 1][j] = pacc[i][j] + p_yoko[i][j];
        }
    }

    for _ in 0..q {
        let a: usize = sc.cin();
        let b: usize = sc.cin();
        let c: usize = sc.cin();
        let d: usize = sc.cin();
        let ans = count_rect(a, b, n, sum_p, &pacc) + count_rect(c + 1, d + 1, n, sum_p, &pacc)
            - count_rect(a, d + 1, n, sum_p, &pacc)
            - count_rect(c + 1, b, n, sum_p, &pacc);
        put!(ans);
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
    (# $a:ident $(,)? $(;)? $($xs:expr),* $(,)? ) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}] {} = {:?}", stringify!($a), stringify!($($xs),*), ($($xs),*))
    };
    ($($xs:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($($xs),*), ($($xs),*))
    };
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
/// Array-indexing by i64.
/// (Vec<Vec<..<T>..>>; i64, i64, ..) => Option<T>
#[macro_export]
macro_rules! at {
    ($s:expr;) => { Some($s) };
    ($s:expr; $idx:expr $(,$args:expr)* $(,)?) => {
        if 0 <= $idx {
            let idx_usize = $idx as usize;
            if idx_usize < $s.len() {
                at!($s[idx_usize]; $($args),*)
            } else {
                None
            }
        } else {
            None
        }
    }
}
// }}}
