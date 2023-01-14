#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let s: Vec<char> = sc.chars();
    let t: Vec<char> = sc.chars();

    let mut code: Vec<char> = vec![];
    for _ in 0..2 {
        for i in 0..n {
            code.push(s[i]);
        }
    }
    for _ in 0..n {
        code.push('a')
    }
    for _ in 0..2 {
        for i in 0..n {
            code.push(t[i]);
        }
    }
    for _ in 0..n {
        code.push('z')
    }
    let sa = suffix_array(&code);
    let mut ans = 0_u128;
    let mut acc = 0_u128;
    for &i in sa.iter() {
        if i < n {
            acc += 1;
        } else if 3 * n <= i && i < 4 * n {
            ans += acc;
        }
    }
    put!(ans);
}

// @string/suffix_array
/// String - Suffix Array (O(n log n log n))
pub fn suffix_array<T: Eq + Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n <= 1 {
        return (0..n).collect();
    }
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<usize> = (0..n).collect();
    {
        let alphabet: BTreeSet<&T> = s.iter().collect();
        let chr: BTreeMap<_, usize> = alphabet
            .iter()
            .enumerate()
            .map(|(idx, c)| (c, idx))
            .collect();
        for i in 0..n {
            rank[i] = chr[&&&s[i]];
        }
    }
    use std::cmp::Ordering;
    fn key(i: usize, k: usize, rank: &Vec<usize>) -> (usize, Option<&usize>) {
        (rank[i], rank.get(i + k))
    }
    fn eq(i: usize, j: usize, k: usize, rank: &Vec<usize>) -> bool {
        key(i, k, rank) == key(j, k, rank)
    }
    let mut k = 1;
    while k < n {
        let mut alt: Vec<usize> = (0..n).collect();
        sa.sort_by_key(|&i| key(i, k, &rank));
        alt[sa[0]] = 0;
        for i in 1..n {
            alt[sa[i]] = alt[sa[i - 1]] + if eq(sa[i], sa[i - 1], k, &rank) { 0 } else { 1 };
        }
        rank = alt;
        k *= 2;
    }
    sa
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
