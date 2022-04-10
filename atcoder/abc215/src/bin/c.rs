#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let s: Vec<char> = sc.chars();
    let k: usize = sc.cin();
    let mut ss: HashSet<String> = Default::default();

    for p in Permutation::new(s.len()) {
        let mut cs = vec![];
        for &i in p.iter() {
            cs.push(s[i]);
        }
        ss.insert(cs.iter().collect());
    }
    let mut ss: Vec<_> = ss.iter().collect();
    ss.sort();
    trace!(&ss);
    put!(ss[k - 1]);
}

// @num/iter/perm
/// Number - Iterator - Factorial Permutation (n!)
pub struct Permutation {
    n: usize,
    idx: usize,
    done: bool,
}
impl Permutation {
    pub fn new(n: usize) -> Permutation {
        Permutation {
            n,
            idx: 0,
            done: false,
        }
    }
    pub fn from(mut perm: Vec<usize>) -> Permutation {
        let n = perm.len();
        let mut idx = 0;
        let mut fact: usize = (1..n).product();
        for i in 0..n {
            if i > 0 {
                fact /= n - i;
            }
            idx += perm[i] * fact;
            for j in i + 1..n {
                if perm[j] > perm[i] {
                    perm[j] -= 1;
                }
            }
        }
        Permutation {
            n,
            idx,
            done: false,
        }
    }
    pub fn to_vec(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.n == 0 {
            self.done = true;
            return Some(vec![]);
        }
        let mut r = vec![0; self.n];
        let mut idx = self.idx;
        for k in 1..self.n {
            r[k] = idx % (k + 1);
            idx /= k + 1;
        }
        if idx > 0 {
            self.done = true;
            return None;
        }
        r.reverse();
        let mut b = vec![true; self.n];
        b[r[0]] = false;
        for k in 1..self.n {
            let mut count = 0;
            for j in 0..self.n {
                if b[j] {
                    if count == r[k] {
                        r[k] = j;
                        b[j] = false;
                        break;
                    }
                    count += 1;
                }
            }
        }
        Some(r)
    }
}
impl Iterator for Permutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        let r = self.to_vec();
        self.idx += 1;
        r
    }
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
// }}}
