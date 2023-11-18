#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let row = sc.chars();
    let col = sc.chars();

    fn validation(line: &Vec<char>, first: char) -> bool {
        line.iter().filter(|&&c| c == 'A').count() == 1
            && line.iter().filter(|&&c| c == 'B').count() == 1
            && line.iter().filter(|&&c| c == 'C').count() == 1
            && line.iter().filter(|&&c| c != '.').next() == Some(&first)
    }

    // 行ごとにありえる組み合わせを列挙する
    let mut rowss: Vec<Vec<Vec<char>>> = vec![vec![]; n];
    for i in 0..n {
        trace!(i);
        for p in PowerPermutation::new(4, n) {
            let r: Vec<char> = p
                .iter()
                .map(|i| match i {
                    0 => '.',
                    1 => 'A',
                    2 => 'B',
                    _ => 'C',
                })
                .collect();
            if !validation(&r, row[i]) {
                continue;
            }
            rowss[i].push(r);
        }
    }

    let size: Vec<usize> = (0..n).map(|i| rowss[i].len()).collect();
    for rs in CartesianPermutation::new(size) {
        let mut ok = true;
        // i-th row <- rowss[i][rs[i]]
        for j in 0..n {
            let line: Vec<char> = (0..n).map(|i| rowss[i][rs[i]][j]).collect();
            if !validation(&line, col[j]) {
                ok = false;
            }
        }
        if ok {
            put!(#Yes);
            for i in 0..n {
                for j in 0..n {
                    print!("{}", rowss[i][rs[i]][j]);
                }
                println!();
            }
            return;
        }
    }

    put!(#No);
}

/// Iterator - Cartesian (zip) Product Permutation (n x m)
pub struct CartesianPermutation {
    dim: usize,
    size: Vec<usize>,
    data: Vec<usize>,
    done: bool,
}
#[macro_export]
macro_rules! zip {
    ($($xs:expr),* $(,)?) => { CartesianPermutation::new(vec![$($xs),*]) }
}
impl CartesianPermutation {
    pub fn new(size: Vec<usize>) -> Self {
        let dim = size.len();
        let done = size.iter().any(|&s| s == 0);
        Self {
            dim,
            size,
            data: vec![0; dim],
            done,
        }
    }
}
impl Iterator for CartesianPermutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        let ret = self.data.clone();
        self.data[0] += 1;
        for i in 0..self.dim {
            if self.data[i] >= self.size[i] {
                self.data[i] -= self.size[i];
                if i < self.dim - 1 {
                    self.data[i + 1] += 1;
                } else {
                    self.done = true;
                }
            }
        }
        Some(ret)
    }
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
