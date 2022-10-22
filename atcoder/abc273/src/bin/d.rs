#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let h: i64 = sc.cin();
    let w: i64 = sc.cin();
    let (mut x, mut y): (i64, i64) = sc.pair();
    let n: usize = sc.cin();
    let mut vs = DefaultDict::new(BTreeSet::new());
    let mut hs = DefaultDict::new(BTreeSet::new());
    for _ in 0..n {
        let i: i64 = sc.cin();
        let j: i64 = sc.cin();
        vs[i].insert(j);
        hs[j].insert(i);
    }
    let q: usize = sc.cin();
    for _ in 0..q {
        let d: String = sc.cin();
        let len: i64 = sc.cin();
        trace!(&d, len, (x, y));
        match d.as_str() {
            "L" => {
                if let Some(&wy) = vs[x].range(0..y).next_back() {
                    let wy = wy + 1;
                    if y - wy >= len {
                        y -= len;
                    } else {
                        y = wy;
                    }
                } else {
                    y = max!(1, min!(w, y - len));
                }
            }
            "R" => {
                if let Some(&wy) = vs[x].range(y + 1..).next() {
                    let wy = wy - 1;
                    if wy - y >= len {
                        y += len;
                    } else {
                        y = wy;
                    }
                } else {
                    y = max!(1, min!(w, y + len));
                }
            }
            "U" => {
                if let Some(&wx) = hs[y].range(0..x).next_back() {
                    let wx = wx + 1;
                    if x - wx >= len {
                        x -= len;
                    } else {
                        x = wx;
                    }
                } else {
                    x = max!(1, min!(h, x - len));
                }
            }
            "D" => {
                if let Some(&wx) = hs[y].range(x + 1..).next() {
                    let wx = wx - 1;
                    if wx - x >= len {
                        x += len;
                    } else {
                        x = wx;
                    }
                } else {
                    x = max!(1, min!(h, x + len));
                }
            }
            _ => {
                panic!();
            }
        }
        trace!(x, y);
        put!(x, y);
    }
}

// @collections/defaultdict
/// collections - defaultdict
#[derive(Debug, Clone)]
pub struct DefaultDict<K, V>
where
    K: Eq + std::hash::Hash,
{
    data: std::collections::HashMap<K, V>,
    default: V,
}
impl<K: Eq + std::hash::Hash, V> DefaultDict<K, V> {
    pub fn new(default: V) -> DefaultDict<K, V> {
        DefaultDict {
            data: std::collections::HashMap::new(),
            default,
        }
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<K, V> {
        self.data.keys()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.data.iter()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl<K: Eq + std::hash::Hash, V> std::ops::Index<K> for DefaultDict<K, V> {
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        if let Some(val) = self.data.get(&key) {
            val
        } else {
            &self.default
        }
    }
}
impl<K: Eq + std::hash::Hash + Clone, V: Clone> std::ops::IndexMut<K> for DefaultDict<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        let val = self.default.clone();
        self.data.entry(key.clone()).or_insert(val);
        self.data.get_mut(&key).unwrap()
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
