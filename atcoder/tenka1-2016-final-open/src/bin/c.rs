#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Default)]
pub struct TrieMap<K: Ord, V> {
    value: Option<V>,
    children: std::collections::BTreeMap<K, TrieMap<K, V>>,
}

impl<K: Copy + Default + Ord, V: Copy + Default> TrieMap<K, V> {
    pub fn add(&mut self, keys: &[K], value: V) {
        if keys.is_empty() {
            self.value = Some(value);
        } else {
            let head = keys[0];
            if !self.children.contains_key(&head) {
                self.children.insert(head, TrieMap::default());
            }
            let child = self.children.get_mut(&head).unwrap();
            child.add(&keys[1..], value);
        }
    }
    pub fn prefix_search(&self, s: &[K]) -> Vec<V> {
        let mut values = vec![];
        if let Some(value) = self.value {
            values.push(value);
        }
        if s.len() > 0 {
            if let Some(child) = self.children.get(&s[0]) {
                let mut vs = child.prefix_search(&s[1..]);
                values.append(&mut vs);
            }
        }
        values
    }
}

impl<K: std::fmt::Debug + Ord, V: std::fmt::Debug> TrieMap<K, V> {
    pub fn debug(&self) {
        #[cfg(debug_assertions)]
        self.debug_indent(0);
    }
    fn debug_indent(&self, indent: usize) {
        eprintln!("{:?}", self.value);
        for (key, child) in self.children.iter() {
            for _ in 0..indent {
                eprint!(" ");
            }
            eprint!(" +- {:?}:", key);
            child.debug_indent(indent + 2);
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let s: Vec<char> = sc.chars();
    let m: usize = sc.cin();
    let mut map: TrieMap<char, usize> = TrieMap::default();
    let ps: Vec<Vec<char>> = (0..m).map(|_| sc.chars()).collect();
    for i in 0..m {
        map.add(&ps[i], i);
    }
    // map.debug();

    let ws: Vec<i64> = sc.vec(m);
    let mut dp = vec![0; s.len() + 1];

    for i in 0..s.len() {
        let indices = map.prefix_search(&s[i..]);
        for j in indices {
            dp[i + ps[j].len()] = max!(dp[i + ps[j].len()], dp[i] + ws[j]);
        }
        dp[i + 1] = max!(dp[i + 1], dp[i]);
    }
    put!(dp[s.len()]);
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
    (yes) => {println!("Yes")}; (no) => {println!("No")};
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
    ($x:expr;) => {
        $x
    };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

// }}}
