#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    L,
    R,
    U,
    D,
}
impl Dir {
    fn make(&self, op: bool) -> (Dir, char) {
        use Dir::*;
        match self {
            L => {
                if op {
                    (U, 'R')
                } else {
                    (D, 'L')
                }
            }
            R => {
                if op {
                    (U, 'L')
                } else {
                    (D, 'R')
                }
            }
            U => {
                if op {
                    (R, 'R')
                } else {
                    (L, 'L')
                }
            }
            D => {
                if op {
                    (R, 'L')
                } else {
                    (L, 'R')
                }
            }
        }
    }
}

fn solve(xs: &Vec<i64>, goal: i64) -> Option<Vec<bool>> {
    let n = xs.len();
    trace!(#solve, n, &xs);
    let m = n / 2;
    // 前半の全列挙
    let mut middles = BTreeSet::new();
    for iset in 0..1 << m {
        let mut sum = 0;
        for i in 0..m {
            if (iset >> i) & 1 > 0 {
                sum += xs[i];
            } else {
                sum -= xs[i];
            }
        }
        middles.insert(sum);
    }
    let mut accept = None;
    // 後半の全探索
    let mut path_later = 0;
    for iset in 0..1 << (n - m) {
        let mut sum = 0;
        for i in 0..(n - m) {
            if (iset >> i) & 1 > 0 {
                sum += xs[m + i];
            } else {
                sum -= xs[m + i];
            }
        }
        if middles.contains(&(goal - sum)) {
            accept = Some(goal - sum);
            path_later = iset;
            break;
        }
    }
    if let Some(mid) = accept {
        // もっかい前半を探す
        for iset in 0..1 << m {
            let mut sum = 0;
            for i in 0..m {
                if (iset >> i) & 1 > 0 {
                    sum += xs[i];
                } else {
                    sum -= xs[i];
                }
            }
            if sum == mid {
                let mut r = vec![];
                for i in 0..m {
                    r.push((iset >> i) & 1 > 0);
                }
                for i in 0..(n - m) {
                    r.push((path_later >> i) & 1 > 0);
                }
                return Some(r);
            }
        }
        None
    } else {
        None
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let x: i64 = sc.cin();
    let y: i64 = sc.cin();
    let a: Vec<i64> = sc.vec(n);

    let mut tate = vec![];
    let mut yoko = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            tate.push(a[i]);
        } else {
            yoko.push(a[i]);
        }
    }

    match (solve(&tate, y), solve(&yoko, x)) {
        (Some(z1), Some(z2)) => {
            trace!(&z1);
            trace!(&z2);
            put!(#Yes);
            let ops: Vec<bool> = (0..n)
                .map(|i| if i % 2 == 0 { z1[i / 2] } else { z2[i / 2] })
                .collect();
            trace!(&ops);
            use Dir::*;
            let mut cur = R;
            for i in 0..n {
                let (next, dirchar) = cur.make(ops[i]);
                print!("{}", dirchar);
                cur = next;
            }
            println!();
        }
        _ => {
            put!(#No);
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
