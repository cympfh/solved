#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Mono {
    Wall,
    Key,
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let mut g: i64 = sc.cin();
    let mut ys: Vec<i64> = sc.vec(n);
    let mut zs: Vec<i64> = sc.vec(n);

    if g < 0 {
        g = -g;
        ys = ys.into_iter().map(|y| -y).collect();
        zs = zs.into_iter().map(|y| -y).collect();
    }

    let mut ans = -1;

    use Mono::*;
    for &left_bound in zs.iter() {
        for b in 0..2 {
            let mut ps: BTreeSet<(i64, usize, Mono)> = BTreeSet::new();
            for i in 0..n {
                ps.insert((ys[i], i, Wall));
                ps.insert((zs[i], i, Key));
            }
            let mut x = 0;
            let mut length = 0;
            let mut right = b == 0;
            let mut stress = 0;
            let mut goal = false;
            loop {
                if stress > 2 {
                    break;
                }
                if right {
                    if let Some((y, i, mono)) = ps.range((x, 0, Wall)..).next().cloned() {
                        trace!(y, i, mono);
                        if y > g {
                            trace!("goal");
                            length += g - x;
                            goal = true;
                            break;
                        }
                        match mono {
                            Wall => {
                                right = false;
                                stress += 1;
                            }
                            Key => {
                                trace!("Get key");
                                stress = 0;
                                ps.remove(&(ys[i], i, Wall));
                                ps.remove(&(zs[i], i, Key));
                                length += (x - y).abs();
                                x = y;
                            }
                        }
                    } else {
                        trace!("goal");
                        length += g - x;
                        goal = true;
                        break;
                    }
                } else {
                    if let Some((y, i, mono)) = ps.range(..(x, 0, Wall)).next_back().cloned() {
                        trace!(y, i, mono);
                        match mono {
                            Wall => {
                                right = true;
                                stress += 1;
                            }
                            Key => {
                                trace!("Get key");
                                stress = 0;
                                ps.remove(&(ys[i], i, Wall));
                                ps.remove(&(zs[i], i, Key));
                                length += (x - y).abs();
                                x = y;
                            }
                        }
                        if y <= left_bound {
                            trace!("left bound");
                            right = true;
                        }
                    } else {
                        right = true;
                    }
                }
            }
            if goal {
                trace!(length);
                if ans == -1 || length < ans {
                    ans = length;
                }
            }
        }
    }
    put!(ans);
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
