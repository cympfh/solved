#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

const INF: i32 = 377_377;

#[derive(Debug, Clone, Copy)]
enum Top {
    Val(i32),
    Infty,
}
use Top::*;

impl Top {
    fn add(&self, other: Top) -> Top {
        match (self, other) {
            (Val(x), Val(y)) => Val(x + y),
            _ => Infty,
        }
    }
    fn min(&self, other: Top) -> Top {
        match (*self, other) {
            (Val(x), Val(y)) => Val(x.min(y)),
            (Val(x), Infty) => Val(x),
            (Infty, Val(x)) => Val(x),
            _ => Infty,
        }
    }
}

#[derive(Debug, Clone)]
struct Tel {
    data: Vec<Vec<Top>>,
}
impl Tel {
    fn new(m: usize, u: usize, neigh: &Vec<usize>) -> Self {
        let mut data = ndarray![Infty; m, m];
        for &v in neigh.iter() {
            let j = u - v - 1;
            data[0][j] = Val(1);
        }
        for i in 1..m {
            data[i][i - 1] = Val(0);
        }
        Self { data }
    }
    fn unit(m: usize) -> Self {
        let mut data = ndarray![Infty; m, m];
        for i in 0..m {
            data[i][i] = Val(0);
        }
        Self { data }
    }
    fn uplift(m: usize) -> Self {
        let mut data = ndarray![Infty; m, m];
        for i in 1..m {
            data[i][i - 1] = Val(0);
        }
        Self { data }
    }
    /// (self; other) == (other * self)
    fn then(&self, other: &Tel) -> Tel {
        other.mul(self)
    }
    /// self * other
    fn mul(&self, other: &Tel) -> Tel {
        let m = self.data.len();
        let mut data = ndarray![Infty; m, m];
        for i in 0..m {
            for j in 0..m {
                let mut v = Infty;
                for k in 0..m {
                    v = v.min(self.data[i][k].add(other.data[k][j]));
                }
                data[i][j] = v;
            }
        }
        Tel { data }
    }
    /// self * (0, infty)
    fn solve(&self) -> i32 {
        if let Top::Val(ans) = self.data[0][0] {
            ans
        } else {
            -1
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let mut g = vec![vec![]; n];
    let mut grev = vec![vec![]; n];
    for u in 0..n {
        let s = sc.chars();
        for j in 0..m {
            let v = u + j + 1;
            if s[j] == '1' {
                g[u].push(v);
                grev[v].push(u);
            }
        }
    }

    let mut tels = vec![];
    tels.push(Tel::unit(m));
    for i in 1..n {
        tels.push(Tel::new(m, i, &grev[i]));
    }

    let mut forwards = vec![];
    {
        let mut tel = Tel::unit(m);
        forwards.push(tel.clone());
        for i in 1..n {
            tel = tel.then(&tels[i]);
            forwards.push(tel.clone());
        }
    }

    let mut backwards = vec![];
    {
        let mut tel = Tel::unit(m);
        backwards.push(tel.clone());
        for i in (1..n).rev() {
            tel = tel.mul(&tels[i]);
            backwards.push(tel.clone());
        }
    }

    let uplift = Tel::uplift(m);

    for i in 0..n - 2 {
        let tel = forwards[i].then(&uplift).then(&backwards[n - i - 2]);
        put!(tel.solve());
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
