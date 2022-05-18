#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    for _ in 0..n {
        let n: usize = sc.cin();
        let s = sc.chars();
        let rl = runlength(&s);
        trace!(&rl);
        let mut stack = vec![];
        for &(chr, num) in rl.iter().rev() {
            if let Some((chr0, num0)) = stack.pop() {
                if chr == chr0 {
                    stack.push((chr, num + num0));
                } else if chr == 'A' {
                    if num < num0 {
                        stack.push(('B', num0 - num));
                    } else {
                        stack.push(('B', 1));
                        stack.push(('A', 1 + num - num0));
                    }
                } else {
                    stack.push((chr0, num0));
                    stack.push((chr, num));
                }
            } else {
                stack.push((chr, num));
            }
        }
        trace!(&stack);
        for &(chr, num) in stack.iter().rev() {
            for _ in 0..num {
                print!("{}", chr);
            }
        }
        println!();
    }
}

// @string/runlength
/// String Compression - Run-Length
pub fn runlength<A: Copy + PartialEq>(xs: &Vec<A>) -> Vec<(A, usize)> {
    let m = xs.len();
    if m == 0 {
        return vec![];
    }
    let mut count = 1;
    let mut result = vec![];
    for i in 0..m {
        if i > 0 {
            if xs[i - 1] == xs[i] {
                count += 1;
            } else {
                count = 1;
            }
        }
        if i == m - 1 || xs[i] != xs[i + 1] {
            result.push((xs[i], count));
        }
    }
    result
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
