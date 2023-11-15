#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let q: usize = sc.cin();

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let u = sc.usize1();
        let v = sc.usize1();
        let c: i64 = sc.cin();
        g[u].push((v, c));
        g[v].push((u, -c));
    }

    let mut cmp = vec![0; n];
    {
        let mut cmp_cursor = 0;
        for root in 0..n {
            if cmp[root] > 0 {
                continue;
            }
            cmp_cursor += 1;
            let mut stack = vec![root];
            while let Some(u) = stack.pop() {
                if cmp[u] > 0 {
                    continue;
                }
                cmp[u] = cmp_cursor;
                for &(v, _) in g[u].iter() {
                    if cmp[v] > 0 {
                        continue;
                    }
                    stack.push(v);
                }
            }
        }
    }
    trace!(&cmp);

    let mut height = vec![None; n];
    let mut infmark = vec![false; n];

    for root in 0..n {
        if height[root] != None {
            continue;
        }
        let mut detect_inf = false;
        height[root] = Some(0_i64);
        let mut q = BinaryHeap::new();
        q.push((Some(0), root));
        'dijkstra: while let Some((Some(h), u)) = q.pop() {
            // trace!(h, u);
            for &(v, c) in g[u].iter() {
                let hv = Some(h + c);
                // trace!(hv, v);
                if height[v] == None {
                    height[v] = Some(h + c);
                    q.push((height[v], v));
                } else if height[v] != hv {
                    // INF!!
                    detect_inf = true;
                    break 'dijkstra;
                }
            }
        }
        if detect_inf {
            let mut stack = vec![root];
            while let Some(u) = stack.pop() {
                if infmark[u] {
                    continue;
                }
                infmark[u] = true;
                for &(v, _) in g[u].iter() {
                    if !infmark[v] {
                        stack.push(v);
                    }
                }
            }
        }
    }
    trace!(&height);
    trace!(&infmark);
    for _ in 0..q {
        let x = sc.usize1();
        let y = sc.usize1();
        if cmp[x] != cmp[y] {
            put!(#nan);
        } else if infmark[x] || infmark[y] {
            put!(#inf);
        } else {
            put!(height[y].unwrap() - height[x].unwrap());
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
