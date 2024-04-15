#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn rot(deg: usize, x: i64, y: i64) -> (i64, i64) {
    match deg {
        0 => (x, y),
        1 => (-y, x),
        2 => (-x, -y),
        3 => (y, -x),
        _ => (0, 0),
    }
}

fn xy2ij(x: i64, y: i64) -> Option<(usize, usize)> {
    if 0 <= x && x < 4 && 0 <= y && y < 4 {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

fn main() {
    let mut sc = Scanner::default();

    let p1: Vec<Vec<bool>> = (0..4)
        .map(|_| {
            let line = sc.chars();
            (0..4).map(|i| line[i] == '#').collect()
        })
        .collect();
    let p2: Vec<Vec<bool>> = (0..4)
        .map(|_| {
            let line = sc.chars();
            (0..4).map(|i| line[i] == '#').collect()
        })
        .collect();
    let p3: Vec<Vec<bool>> = (0..4)
        .map(|_| {
            let line = sc.chars();
            (0..4).map(|i| line[i] == '#').collect()
        })
        .collect();

    {
        let mut num = 0;
        for i in 0..4 {
            for j in 0..4 {
                if p1[i][j] {
                    num += 1;
                }
                if p2[i][j] {
                    num += 1;
                }
                if p3[i][j] {
                    num += 1;
                }
            }
        }
        if num != 16 {
            put!(#No);
            return;
        }
    }

    let mut b = ndarray![false; 4, 4];

    for dx1 in -4..4 {
        for dy1 in -4..4 {
            for deg1 in 0..4 {
                for dx2 in -4..4 {
                    for dy2 in -4..4 {
                        for deg2 in 0..4 {
                            for dx3 in -4..4 {
                                for dy3 in -4..4 {
                                    for deg3 in 0..4 {
                                        let mut failed = false;
                                        for i in 0..4 {
                                            for j in 0..4 {
                                                b[i][j] = false;
                                            }
                                        }
                                        for i in 0..4 {
                                            for j in 0..4 {
                                                if !p1[i][j] {
                                                    continue;
                                                }
                                                let (x, y) =
                                                    rot(deg1, i as i64 + dx1, j as i64 + dy1);
                                                if let Some((i, j)) = xy2ij(x, y) {
                                                    b[i][j] = true;
                                                } else {
                                                    failed = true;
                                                }
                                            }
                                        }
                                        for i in 0..4 {
                                            for j in 0..4 {
                                                if !p2[i][j] {
                                                    continue;
                                                }
                                                let (x, y) =
                                                    rot(deg2, i as i64 + dx2, j as i64 + dy2);
                                                if let Some((i, j)) = xy2ij(x, y) {
                                                    if b[i][j] {
                                                        failed = true;
                                                    }
                                                    b[i][j] = true;
                                                } else {
                                                    failed = true;
                                                }
                                            }
                                        }
                                        for i in 0..4 {
                                            for j in 0..4 {
                                                if !p3[i][j] {
                                                    continue;
                                                }
                                                let (x, y) =
                                                    rot(deg3, i as i64 + dx3, j as i64 + dy3);
                                                if let Some((i, j)) = xy2ij(x, y) {
                                                    if b[i][j] {
                                                        failed = true;
                                                    }
                                                    b[i][j] = true;
                                                } else {
                                                    failed = true;
                                                }
                                            }
                                        }
                                        if !failed {
                                            put!(#Yes);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    put!(#No);
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
