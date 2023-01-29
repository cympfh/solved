#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

/// N <= 1000
/// M <= 300
/// D <= 30
/// K <= 2*M/D
struct Game {
    graph: Graph,
    days: usize,
    k: usize,
    rand: XorShift,
}
impl Game {
    fn new(graph: Graph, days: usize, k: usize) -> Self {
        let rand = XorShift::new();
        Self {
            graph,
            days,
            k,
            rand,
        }
    }
}

struct Graph {
    n: usize,
    m: usize,
    edges: Vec<(usize, usize, i64)>,
    list: Vec<Vec<(usize, i64)>>, // 隣接リスト
    mat: Vec<Vec<i64>>,           // 隣接行列
    distance: Vec<Vec<i64>>,      // 点対最短距離 (近似)
}
impl Graph {
    fn new(n: usize, m: usize, edges: Vec<(usize, usize, i64)>) -> Self {
        let list = {
            let mut g = vec![vec![]; n];
            for &(u, v, w) in edges.iter() {
                g[u].push((v, w));
                g[v].push((u, w));
            }
            g
        };
        let mat = {
            let mut f = ndarray![1_000_000_000; n, n];
            for i in 0..n {
                f[i][i] = 0;
            }
            for &(u, v, w) in edges.iter() {
                f[u][v] = w;
                f[v][u] = w;
            }
            f
        };
        // Aux-min-distance
        let distance = {
            let mut f = mat.clone();
            warshall_floyd(&mut f);
            f
        };
        Self {
            n,
            m,
            edges,
            list,
            mat,
            distance,
        }
    }
}

#[derive(Clone, Debug)]
struct Plan {
    data: Vec<Vec<usize>>,
    scores: Vec<i64>, // 日毎の不満の和
    score: f64,       // 平均不満度
}
impl Plan {
    fn new(data: Vec<Vec<usize>>) -> Self {
        let days = data.len();
        let scores = vec![0; days];
        let score = 0.0;
        Self {
            data,
            scores,
            score,
        }
    }
    fn add(&mut self, id_day: usize, id_edge: usize) {
        self.data[id_day].push(id_edge);
    }
}

impl Game {
    fn submit(&self, plan: &Plan) {
        let mut ans = vec![0; self.graph.m];
        for i in 0..self.days {
            for &j in plan.data[i].iter() {
                ans[j] = i + 1;
            }
        }
        for j in 0..self.graph.m {
            assert!(1 <= ans[j] && ans[j] <= self.days, ">>> ans = {:?}", ans);
        }
        put!(..ans);
    }
}

fn main() {
    let mut sc = Scanner::default();

    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let d: usize = sc.cin();
    let k: usize = sc.cin();
    let edges: Vec<_> = (0..m)
        .map(|_| {
            let u = sc.usize1();
            let v = sc.usize1();
            let w: i64 = sc.cin();
            (u, v, w)
        })
        .collect();
    let _pos: Vec<_> = (0..n)
        .map(|_| {
            let x: i64 = sc.cin();
            let y: i64 = sc.cin();
            (x, y)
        })
        .collect();

    let graph = Graph::new(n, m, edges);
    let game = Game::new(graph, d, k);

    #[allow(unused_assignments)]
    let mut plan_submit = Plan::new(vec![]);

    // let mut plan = baseline(&game);
    // game.update_score_all(&mut plan);
    // trace!(#Baseline, plan.score);
    // plan_submit = plan.clone();

    let mut plan = disjoint_planning(&game);
    game.update_score_all(&mut plan);
    trace!(#Disjoint, plan.score);
    plan_submit = plan.clone();

    game.submit(&plan_submit);
}

fn baseline(game: &Game) -> Plan {
    let mut data = vec![vec![]; game.days];
    for i in 0..game.graph.m {
        let j = i % game.days;
        data[j].push(i);
    }
    Plan::new(data)
}

// 辺どうしが共通頂点を持たないように選ぶ
fn disjoint_planning(game: &Game) -> Plan {
    let mut data = vec![vec![]; game.days];
    let mut edge_ids: BTreeSet<usize> = (0..game.graph.m).collect();
    let mut d = 0;
    let norma = (game.k + ((game.k + game.days - 1) / game.days)) / 2;
    while !edge_ids.is_empty() {
        let mut vset = BTreeSet::new();
        let mut used = vec![];
        for &i in edge_ids.iter() {
            let (u, v, _) = game.graph.edges[i];
            if vset.contains(&u) || vset.contains(&v) {
                continue;
            }
            used.push(i);
            vset.insert(u);
            vset.insert(v);
            data[d].push(i);
            if data[d].len() >= norma {
                break;
            }
        }
        for i in used {
            edge_ids.remove(&i);
        }
        if edge_ids.is_empty() {
            break;
        }
        d = (d + 1) % game.days;
        while data[d].len() >= game.k {
            d = (d + 1) % game.days;
        }
    }
    Plan::new(data)
}

// DFS で一筆書きっぽく書いたパスを分散させてく
// 今バグってる。辺のIDの代わりに頂点IDが入っちゃってる
fn dfs_planning(game: &Game) -> Plan {
    let n = game.graph.n;
    let mut visited = vec![false; n];
    let mut ord = vec![];
    let mut stack = vec![0];
    while let Some(u) = stack.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        ord.push(u);
        for &(v, _) in game.graph.list[u].iter() {
            if visited[v] {
                continue;
            }
            stack.push(v);
        }
    }
    assert_eq!(ord.len(), n);
    let mut data = vec![vec![]; game.days];
    for i in 0..ord.len() {
        data[i % game.days].push(ord[i]);
    }
    Plan::new(data)
}

impl Game {
    fn update_score_all(&self, plan: &mut Plan) {
        for d in 0..self.days {
            self.update_score_oneday(plan, d);
        }
        plan.score = plan.scores.iter().sum::<i64>() as f64
            / (self.graph.n * (self.graph.n - 1) / 2 * self.days) as f64
            * 1000.0;
    }
    fn update_score_oneday(&self, plan: &mut Plan, i: usize) {
        let score_old = plan.scores[i];
        let score_new = self.score_oneday(plan, i);
        plan.scores[i] += score_new - score_old;
    }
    fn score_oneday(&self, plan: &Plan, i: usize) -> i64 {
        let mut d = self.graph.mat.clone();
        for &i in plan.data[i].iter() {
            let (u, v, _) = self.graph.edges[i];
            d[u][v] = 1_000_000_000;
            d[v][u] = 1_000_000_000;
        }
        warshall_floyd(&mut d);
        let mut r = 0;
        for u in 0..self.graph.n {
            for v in 0..u {
                r += d[u][v] - self.graph.distance[u][v];
            }
        }
        r
    }
}

/// Graph - Warshall-Floyd
pub fn warshall_floyd(f: &mut Vec<Vec<i64>>) {
    let n = f.len();
    // for i in 0..n { f[i][i] = 0; }
    let maxk = min!(n, 2_000_000 / n / n);
    for k in 0..maxk {
        for i in 0..n {
            for j in 0..n {
                let w = f[i][k] + f[k][j];
                if w < f[i][j] {
                    f[i][j] = w;
                }
            }
        }
    }
}

// {{{ rand
// @num/random/xorshift
// @num/random/fromu64
/// Number - Utility - FromU64
pub trait FromU64 {
    fn coerce(x: u64) -> Self;
}
impl FromU64 for u64 {
    fn coerce(x: u64) -> Self {
        x
    }
}
macro_rules! define_fromu64 {
    ($ty:ty) => {
        impl FromU64 for $ty {
            fn coerce(x: u64) -> Self {
                x as $ty
            }
        }
    };
}
define_fromu64!(usize);
define_fromu64!(u32);
define_fromu64!(u128);
define_fromu64!(i32);
define_fromu64!(i64);
define_fromu64!(i128);
impl FromU64 for bool {
    fn coerce(x: u64) -> Self {
        x % 2 == 0
    }
}
impl FromU64 for f32 {
    fn coerce(x: u64) -> Self {
        (x as f32) / (std::u64::MAX as f32)
    }
}
impl FromU64 for f64 {
    fn coerce(x: u64) -> Self {
        (x as f64) / (std::u64::MAX as f64)
    }
}

/// Random Number - Xor-Shift Algorithm
pub struct XorShift(u64);
impl XorShift {
    pub fn new() -> Self {
        XorShift(88_172_645_463_325_252)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x = x ^ (x << 13);
        x = x ^ (x >> 7);
        x = x ^ (x << 17);
        self.0 = x;
        x
    }
    pub fn gen<T: FromU64>(&mut self) -> T {
        T::coerce(self.next())
    }
    pub fn choose<T: Copy>(&mut self, ls: &Vec<T>) -> T {
        let i = self.gen::<usize>() % ls.len();
        ls[i]
    }
    pub fn choose2<T: Copy>(&mut self, ls: &Vec<T>) -> (T, T) {
        let i = self.gen::<usize>() % ls.len();
        let j = self.gen::<usize>() % ls.len();
        if i != j {
            (ls[i], ls[j])
        } else {
            self.choose2(ls)
        }
    }
}
// }}}

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
    (# $label:ident, $($xs:expr),*) => {
        #[cfg(debug_assertions)]
        eprintln!(
            "[{}] \t>>> {} = {:?}",
            stringify!($label),
            stringify!(($($xs),*)),
            ($($xs),*)
        );
    };
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x),$x);
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
