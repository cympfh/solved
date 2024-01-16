#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let a: Vec<usize> = sc.vec(n);

    let mut g = vec![vec![]; n];
    for u in 0..n {
        let v = a[u] - 1;
        if u != v {
            g[u].push(v);
        }
    }
    let (cmp, _) = scc(&g);
    let mut count = vec![0; n];
    for &u in cmp.iter() {
        count[u] += 1;
    }
    trace!(&cmp);
    trace!(&count);
    // put!(#debug);

    let mut ans = 0;
    for i in 0..n {
        if a[i] - 1 == i || count[cmp[i]] > 1 {
            ans += 1;
        }
    }
    put!(ans);
}

// @graph/directed/scc
/// Graph - Directed - Strongly Connected Component (SCC)
/// convert a DiGraph to a DAG
/// scc: (g) -> (cmp, dag)
///   where
///     `g` is a neighbor list of DiGraph
///     `cmp` is mapping vector; cmp[DiGraph-Vertex-Index] = DAG-Vertex-Index
///     `dag` is a neighbor list of DAG
pub fn scc(g: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = g.len();

    // Post-order traversal
    let mut po = vec![];
    {
        fn dfs(u: usize, g: &Vec<Vec<usize>>, mut used: &mut Vec<bool>, mut po: &mut Vec<usize>) {
            if used[u] {
                return;
            }
            used[u] = true;
            for &v in g[u].iter() {
                if !used[v] {
                    dfs(v, &g, &mut used, &mut po);
                }
            }
            po.push(u);
        }
        let mut used = vec![false; n];
        for u in 0..n {
            dfs(u, &g, &mut used, &mut po);
        }
    }

    let mut g_r = vec![vec![]; n];
    for u in 0..n {
        for &v in g[u].iter() {
            g_r[v].push(u);
        }
    }

    // Components
    let mut cmp = vec![0; n];
    let m;
    {
        let mut used = vec![false; n];
        let mut k = 0;
        po.reverse();
        for &u in po.iter() {
            let mut stack = vec![u];
            if used[u] {
                continue;
            }
            while let Some(v) = stack.pop() {
                if used[v] {
                    continue;
                }
                used[v] = true;
                cmp[v] = k;
                for &w in g_r[v].iter() {
                    stack.push(w)
                }
            }
            k += 1;
        }
        m = k;
    }

    // DAG
    let mut dag = vec![vec![]; m];
    for u in 0..n {
        let u2 = cmp[u];
        for &v in g[u].iter() {
            let v2 = cmp[v];
            if u2 != v2 {
                dag[u2].push(v2)
            }
        }
    }
    for u in 0..m {
        dag[u].sort();
        dag[u].dedup();
    }

    (cmp, dag)
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
