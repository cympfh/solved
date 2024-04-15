#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn solve(s: &Vec<char>) -> Option<usize> {
    let mut head = 0;
    let mut tail = 0;
    let t = {
        let mut s = s.clone();
        while s.len() >= 1 && s[s.len() - 1] == '0' {
            s.pop();
            tail += 1;
        }
        s.reverse();
        while s.len() > 0 && s[s.len() - 1] == '0' {
            s.pop();
            head += 1;
        }
        s.reverse();
        s
    };
    let ones = s.iter().filter(|&&c| c == '1').count();
    trace!(&t, head, tail, ones);
    if ones == 0 {
        // 0000000
        Some(0)
    } else if ones % 2 == 1 {
        None
    } else if ones < t.len() {
        // 1111011111
        Some(ones / 2)
    } else if t.len() == 2 && head >= 2 {
        Some(ones / 2 + 1)
    } else if t.len() == 2 && tail >= 2 {
        Some(ones / 2 + 1)
    } else if t.len() == 2 && head > 0 && tail > 0 {
        Some(ones / 2 + 2)
    } else if t.len() <= 2 {
        None
    } else {
        Some(ones / 2)
    }
}

fn main() {
    let mut sc = Scanner::default();
    let t: usize = sc.cin();
    for _ in 0..t {
        let _n: usize = sc.cin();
        let s: Vec<char> = sc.chars();
        trace!(&s);

        if let Some(ans) = solve(&s) {
            put!(ans);
        } else {
            put!(-1);
        }
        flush();
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
