#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.cin();
    let w: usize = sc.cin();
    let a: usize = sc.cin();
    let _b: usize = sc.cin();
    let mut ans: u64 = 0;

    // 1x2 を置くとこ決める
    for locs in Combination::new(h * w, a) {
        // 縦横決める: 0=縦, 1=横
        for dirs in PowerPermutation::new(2, a) {
            // 置けるかシミュレーション
            let mut tatami = vec![vec![false; w]; h];
            let mut ok = true;
            for (&loc, &dir) in locs.iter().zip(dirs.iter()) {
                let i = loc / w;
                let j = loc % w;
                if dir == 0 {
                    if tatami[i][j] == false && i + 1 < h && tatami[i + 1][j] == false {
                        tatami[i][j] = true;
                        tatami[i + 1][j] = true;
                    } else {
                        ok = false;
                        break;
                    }
                } else {
                    if tatami[i][j] == false && j + 1 < w && tatami[i][j + 1] == false {
                        tatami[i][j] = true;
                        tatami[i][j + 1] = true;
                    } else {
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                ans += 1;
            }
        }
    }

    put!(ans);
}

// @num/iter/power
/// Number - Iterator - Power Permutation (pow(n, m))
pub struct PowerPermutation {
    n: usize,
    m: usize,
    ar: Vec<usize>,
    done: bool,
}
impl PowerPermutation {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            ar: vec![0; m],
            done: false,
        }
    }
}
impl Iterator for PowerPermutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.m == 0 {
            self.done = true;
            return Some(self.ar.clone());
        }
        if self.ar[self.m - 1] >= self.n {
            return None;
        }
        let r = self.ar.clone();
        self.ar[0] += 1;
        for i in 0..self.m - 1 {
            if self.ar[i] == self.n {
                self.ar[i] = 0;
                self.ar[i + 1] += 1;
            } else {
                break;
            }
        }
        Some(r)
    }
}

// @num/iter/combination
/// Number - Iterator - Combination (Binom[n;m])
pub struct Combination {
    n: usize,
    m: usize,
    ar: Vec<usize>,
}
impl Combination {
    pub fn new(n: usize, m: usize) -> Combination {
        let mut ar = vec![0; m];
        for i in 0..m {
            ar[i] = m - i - 1
        }
        Combination { n: n, m: m, ar: ar }
    }
}
impl Iterator for Combination {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.m == 0 {
            if self.n == 0 {
                return None;
            } else {
                self.n = 0;
                return Some(vec![]);
            }
        }
        if self.ar[self.m - 1] > self.n - self.m {
            return None;
        }
        let r = self.ar.clone();
        self.ar[0] += 1;
        let mut c = 0;
        for i in 0..self.m - 1 {
            if self.ar[i] >= self.n - i {
                self.ar[i + 1] += 1;
                c = i + 1;
            } else {
                break;
            }
        }
        for i in (0..c).rev() {
            self.ar[i] = self.ar[i + 1] + 1;
        }
        return Some(r);
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
    (yes) => {println!("Yes")}; (no) => {println!("No")};
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
    ($x:expr;) => {
        $x
    };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

// }}}
