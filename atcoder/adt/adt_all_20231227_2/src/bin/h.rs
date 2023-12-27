#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn occupied(c: char) -> bool {
    match c {
        '#' | '^' | 'v' | '<' | '>' => true,
        _ => false,
    }
}

fn main() {
    let mut sc = Scanner::default();
    let h: usize = sc.cin();
    let w: usize = sc.cin();

    let raw: Vec<Vec<char>> = (0..h).map(|_| sc.chars()).collect();

    let mut s = (0, 0);
    let mut g = (0, 0);

    let mut f = ndarray!['.'; h, w];
    for i in 0..h {
        for j in 0..w {
            match raw[i][j] {
                'S' => {
                    f[i][j] = '.';
                    s = (i, j);
                }
                'G' => {
                    f[i][j] = '.';
                    g = (i, j);
                }
                '#' => {
                    f[i][j] = '#';
                }
                '^' => {
                    f[i][j] = '#';
                    for i in (0..i).rev() {
                        if occupied(raw[i][j]) {
                            break;
                        }
                        f[i][j] = '#';
                    }
                }
                'v' => {
                    f[i][j] = '#';
                    for i in i + 1..h {
                        if occupied(raw[i][j]) {
                            break;
                        }
                        f[i][j] = '#';
                    }
                }
                '<' => {
                    f[i][j] = '#';
                    for j in (0..j).rev() {
                        if occupied(raw[i][j]) {
                            break;
                        }
                        f[i][j] = '#';
                    }
                }
                '>' => {
                    f[i][j] = '#';
                    for j in j + 1..w {
                        if occupied(raw[i][j]) {
                            break;
                        }
                        f[i][j] = '#';
                    }
                }
                _ => {}
            }
        }
        trace!(&f[i]);
    }

    let neigh = neighbor::Grid4(h, w);
    let mut dp = ndarray![9_000_000_u64; h, w];
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), s.0, s.1));
    while let Some((Reverse(cost), x, y)) = q.pop() {
        if f[x][y] == '#' || dp[x][y] <= cost {
            continue;
        }
        dp[x][y] = cost;
        for (u, v) in neigh.iter(x, y) {
            if f[u][v] == '#' {
                continue;
            }
            if dp[u][v] > cost + 1 {
                q.push((Reverse(cost + 1), u, v));
            }
        }
    }
    for i in 0..h {
        trace!(&dp[i]);
    }
    let ans = dp[g.0][g.1];
    if ans < 9_000_000_u64 {
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
    ($x:expr, $($ys:expr),* $(,)*) => {{
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
    ($x:expr, $($ys:expr),* $(,)*) => {{
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
