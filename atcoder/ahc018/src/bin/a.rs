#![allow(unused_imports, unused_macros, dead_code)]
use std::ops::Range;
use std::process::exit;
use std::{cmp::*, collections::*};

#[derive(Debug, Clone)]
struct Game {
    n: usize,
    c: i128,
    waters: Vec<(usize, usize)>,
    homes: Vec<(usize, usize)>,
    broken: Vec<Vec<bool>>,
}
impl Game {
    fn new(n: usize, c: i128, waters: Vec<(usize, usize)>, homes: Vec<(usize, usize)>) -> Self {
        let broken = ndarray![false; n, n];
        Self {
            n,
            c,
            waters,
            homes,
            broken,
        }
    }
    // あるセルを掘る
    fn dig(&mut self, i: usize, j: usize, power: i128) {
        if self.broken[i][j] {
            return;
        }
        println!("{} {} {}", i, j, power);
        flush();
        let mut sc = Scanner::default();
        let res: i32 = sc.cin();
        match res {
            0 => self.broken[i][j] = false,
            1 => self.broken[i][j] = true,
            2 => exit(0),
            _ => panic!("Invalid dig({}, {}, {}) => {}", i, j, power, res),
        }
    }
    // 矩形区間を同じパワーで全部掘る
    fn dig_rect(&mut self, i_range: Range<usize>, j_range: Range<usize>, power: i128) {
        for i in i_range {
            for j in j_range.clone() {
                self.dig(i, j, power);
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let w: usize = sc.cin();
    let k: usize = sc.cin();
    let c: i128 = sc.cin();

    // 水源
    let waters: Vec<(usize, usize)> = (0..w)
        .map(|_| {
            let x: usize = sc.cin();
            let y: usize = sc.cin();
            (x, y)
        })
        .collect();

    // 家
    let homes: Vec<(usize, usize)> = (0..k)
        .map(|_| {
            let x: usize = sc.cin();
            let y: usize = sc.cin();
            (x, y)
        })
        .collect();

    let mut game = Game::new(n, c, waters, homes);
    let mut power = c;
    loop {
        game.dig_rect(0..n, 0..n, power);
        power *= 2;
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
