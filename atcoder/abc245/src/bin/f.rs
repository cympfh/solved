#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let m: usize = sc.cin();

    let mut g = vec![vec![]; n];

    for _ in 0..m {
        let u = sc.usize1();
        let v = sc.usize1();
        g[u].push(v);
    }

    let (cmp, dag) = scc(&g);
    trace!(&cmp);
    trace!(&dag);

    let mut cx = vec![0; n];
    for i in cmp.into_iter() {
        cx[i] += 1;
    }
    trace!(&cx);

    let mut ok = vec![false; n];
    {
        let h = reverse(&dag);
        let ord = Topological::sort(&h);
        trace!(&ord);
        for i in ord {
            if cx[i] > 1 {
                ok[i] = true;
            } else if cx[i] == 1 {
                for &v in dag[i].iter() {
                    if ok[v] {
                        ok[i] = true
                    }
                }
            }
        }
    }
    trace!(&ok);

    let ans = (0..n).map(|i| if ok[i] { cx[i] } else { 0 }).sum::<usize>();
    put!(ans);
}

fn reverse(g: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = g.len();
    let mut h = vec![vec![]; n];
    for u in 0..n {
        for &v in g[u].iter() {
            h[v].push(u);
        }
    }
    h
}

// @graph/directed/topological_sort
/// Graph - Directed - Topological Sort

pub struct Topological;

impl Topological {
    pub fn sort(neigh: &Vec<Vec<usize>>) -> Vec<usize> {
        let n = neigh.len();
        let mut rd = vec![vec![]; n];
        for u in 0..n {
            for &v in neigh[u].iter() {
                rd[v].push(u);
            }
        }
        let mut used = vec![false; n];
        let mut ord = vec![];
        for u in 0..n {
            Self::visit(u, &mut used, &rd, &mut ord);
        }
        ord
    }
    fn visit(u: usize, mut used: &mut Vec<bool>, rd: &Vec<Vec<usize>>, mut ord: &mut Vec<usize>) {
        if used[u] {
            return;
        }
        used[u] = true;
        for &v in rd[u].iter() {
            Self::visit(v, &mut used, &rd, &mut ord);
        }
        ord.push(u);
    }
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
