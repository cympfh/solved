#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

struct Operation {
    maxnum: usize,
    num: usize,
    g: Vec<Vec<char>>,
    painted: Vec<Vec<bool>>,
    commands: Vec<String>,
}

impl Operation {
    fn new(maxnum: usize, g: Vec<Vec<char>>) -> Self {
        let h = g.len();
        let w = g[0].len();
        let painted: Vec<Vec<bool>> = ndarray![false; h, w];
        Self {
            maxnum,
            num: 0,
            g,
            painted,
            commands: vec![],
        }
    }
    fn show(&self) {
        put!(self.commands.len());
        for c in self.commands.iter() {
            put!(c);
        }
    }
    fn drop(&mut self, x: usize, y: usize, r: usize, color: (u8, u8, u8)) {
        if self.num >= self.maxnum {
            return;
        }
        self.num += 1;
        let cmd = format!("drop {} {} {} {} {} {}", y, x, r, color.0, color.1, color.2);
        self.commands.push(cmd);

        // Update painted
        {
            let h = self.g.len();
            let w = self.g[0].len();
            let mut pnew = ndarray![false; h, w];
            let r2 = (r * r) as f64;
            for i in 0..h {
                for j in 0..w {
                    let dist = l2((i, j), (x, y)) as f64;
                    if dist <= r2 + 1e-6 {
                        pnew[i][j] = true;
                    } else {
                        let k = (1.0 - r2 / dist).sqrt();
                        let i2 = (k * (i as f64) + (1.0 - k) * (x as f64)).round() as usize;
                        let j2 = (k * (j as f64) + (1.0 - k) * (y as f64)).round() as usize;
                        pnew[i][j] = self.painted[i2][j2];
                    }
                }
            }
            for i in 0..h {
                for j in 0..w {
                    self.painted[i][j] = pnew[i][j];
                }
            }
        }
    }
    /// the circle has only '#', and they are not painted.
    fn check(&self, i: usize, j: usize, r: usize) -> bool {
        if self.g[i][j] != '#' {
            return false;
        }
        let h = self.g.len();
        let w = self.g[0].len();
        let left = if i >= r { i - r } else { 0 };
        let right = (i + r).min(w - 1);
        let top = if j >= r { j - r } else { 0 };
        let bot = (j + r).min(h - 1);
        let r2 = r * r;
        for ii in left..=right {
            for jj in top..=bot {
                if l2((i, j), (ii, jj)) > r2 {
                    continue;
                }
                if self.g[ii][jj] != '#' {
                    return false;
                }
                if self.painted[ii][jj] {
                    return false;
                }
            }
        }
        true
    }
}

fn diff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}
fn l2((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let dx = diff(x1, x2);
    let dy = diff(y1, y2);
    dx * dx + dy * dy
}

fn main() {
    let mut sc = Scanner::default();
    let w: usize = sc.cin();
    let h: usize = sc.cin();
    let n: usize = sc.cin();
    let g: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();

    let mut op = Operation::new(n, g); // n

    for r in (2..=30).rev() {
        for i in 0..h {
            for j in 0..w {
                if op.check(i, j, r) {
                    let rdrop = match r {
                        _ if r > 20 => r,
                        _ if r > 14 => r - 3,
                        _ if r > 9 => r - 5,
                        _ if r > 4 => r + 1,
                        _ if r == 4 => 5,
                        _ if r == 3 => 5,
                        _ if r == 2 => 4,
                        _ if r == 1 => 2,
                        _ => r + 1,
                    };
                    op.drop(i, j, rdrop, (255, 255, 255));
                }
            }
        }
    }

    let mut score = 0;
    for i in 0..h {
        for j in 0..w {
            let a = op.g[i][j] == '#';
            let b = op.painted[i][j];
            if a != b {
                score += 1;
            }
        }
    }
    eprintln!(">>> score = {}", score);
    op.show();
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
