#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let ss: Vec<Vec<char>> = (0..n).map(|_| sc.chars()).collect();
    let mut code = vec![];
    let mut indices = vec![];
    for s in ss.iter() {
        indices.push(code.len());
        code.push('@');
        for &c in s.iter() {
            code.push(c);
        }
    }
    trace!(&code);
    trace!(indices);
    let (sa, rank) = suffix_array(&code);
    trace!(&sa);
    trace!(&rank);

    for i in 0..n {
        let r = rank[indices[i]];
        let mut ans = 0;
        if r >= 1 {
            ans = max!(ans, lcp(&code, indices[i], sa[r - 1]));
        }
        // if r >= 2 { ans = max!(ans, lcp(&code, indices[i], sa[r - 2])); }
        if r + 1 < code.len() {
            ans = max!(ans, lcp(&code, indices[i], sa[r + 1]));
        }
        // if r + 2 < code.len() { ans = max!(ans, lcp(&code, indices[i], sa[r + 2])); }
        put!(ans);
    }
}

fn lcp(code: &Vec<char>, x: usize, y: usize) -> usize {
    if code[x] != '@' || code[y] != '@' {
        return 0;
    }
    let n = code.len();
    for i in 1.. {
        if x + i >= n
            || y + i >= n
            || code[x + i] != code[y + i]
            || code[x + i] == '@'
            || code[y + i] == '@'
        {
            return i - 1;
        }
    }
    return 0;
}

// @string/suffix_array_search
// @string/suffix_array
/// String - Suffix Array (Manber&Myers, O(n (log n)^2))
pub fn suffix_array<T: Eq + Ord>(s: &[T]) -> (Vec<usize>, Vec<usize>) {
    use std::collections::{BTreeMap, BTreeSet};
    let n = s.len();
    if n <= 1 {
        let a = (0..n).collect();
        let b = (0..n).collect();
        return (a, b);
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
    (sa, rank)
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
