#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};
const INF: i64 = 1_000_000_000;

// {{{ struct Game
/// N <= 1000
/// M <= 300
/// D <= 30
/// K <= 2*M/D
struct Game {
    graph: Graph,
    days: usize,
    k: usize,
    bestplan: Plan,
    bestscore: f64,
}
impl Game {
    fn new(graph: Graph, days: usize, k: usize) -> Self {
        let bestplan = Plan::new(vec![]);
        let bestscore = 0.0;
        Self {
            graph,
            days,
            k,
            bestplan,
            bestscore,
        }
    }
}
impl Game {
    /// self.bestplan を出力
    fn submit(&self) {
        let mut ans = vec![0; self.graph.m];
        for i in 0..self.days {
            for &j in self.bestplan.data[i].iter() {
                ans[j] = i + 1;
            }
        }
        for j in 0..self.graph.m {
            assert!(1 <= ans[j] && ans[j] <= self.days, ">>> ans = {:?}", ans);
        }
        put!(..ans);
    }
    /// スコアが良ければ self.bestplan を更新する
    fn challenge(&mut self, plan: &mut Plan) {
        self.update_score(plan);
        if self.bestscore <= 0.0 || self.bestscore > plan.score {
            self.bestplan = plan.clone();
            self.bestscore = plan.score;
        }
    }
    /// plan.score と plan.scores を更新する
    fn update_score(&self, plan: &mut Plan) {
        for d in 0..self.days {
            plan.scores[d] = self.compute_score_oneday(plan, d);
        }
        plan.score = plan.scores.iter().sum::<f64>() / self.days as f64 * 1000.0;
    }
    /// d日目の不満度を計算する
    fn compute_score_oneday(&self, plan: &Plan, d: usize) -> f64 {
        let ignores: BTreeSet<usize> = plan.data[d].iter().cloned().collect();
        let mut g = vec![vec![]; self.graph.n];
        for i in 0..self.graph.m {
            if ignores.contains(&i) {
                continue;
            }
            let (u, v, w) = self.graph.edges[i];
            g[u].push((v, w));
            g[v].push((u, w));
        }
        let dist = Graph::dijkstra_matrix(&g);
        let mut fuman = 0.0;
        for u in 0..self.graph.n {
            for v in u + 1..self.graph.n {
                fuman += (dist[u][v] - self.graph.distance[u][v]) as f64;
            }
        }
        let den = self.graph.n * (self.graph.n - 1) / 2;
        fuman / den as f64
    }
}
// }}}

// struct Graph {{{
struct Graph {
    n: usize,
    m: usize,
    position: Vec<(i64, i64)>, // 頂点座標
    edges: Vec<(usize, usize, i64)>,
    list: Vec<Vec<(usize, i64)>>, // 隣接リスト
    mat: Vec<Vec<i64>>,           // 隣接行列
    distance: Vec<Vec<i64>>,      // 点対最短距離 (近似)
}
impl Graph {
    fn new(n: usize, m: usize, position: Vec<(i64, i64)>, edges: Vec<(usize, usize, i64)>) -> Self {
        let list = {
            let mut g = vec![vec![]; n];
            for &(u, v, w) in edges.iter() {
                g[u].push((v, w));
                g[v].push((u, w));
            }
            g
        };
        let mat = {
            let mut f = ndarray![INF; n, n];
            for i in 0..n {
                f[i][i] = 0;
            }
            for &(u, v, w) in edges.iter() {
                f[u][v] = w;
                f[v][u] = w;
            }
            f
        };
        let distance = Graph::dijkstra_matrix(&list);
        Self {
            n,
            m,
            position,
            edges,
            list,
            mat,
            distance,
        }
    }
}
impl Graph {
    fn dijkstra(g: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
        let mut dist = vec![INF; g.len()];
        let mut que = BinaryHeap::new();
        que.push((Reverse(0), s));
        dist[s] = 0;
        while let Some((Reverse(d), u)) = que.pop() {
            if dist[u] != d {
                continue;
            }
            for &(v, w) in &g[u] {
                let d2 = d + w;
                if dist[v] > d2 {
                    dist[v] = d2;
                    que.push((Reverse(d2), v));
                }
            }
        }
        dist
    }
    fn dijkstra_matrix(g: &Vec<Vec<(usize, i64)>>) -> Vec<Vec<i64>> {
        let mut dist = vec![];
        for s in 0..g.len() {
            dist.push(Graph::dijkstra(g, s));
        }
        dist
    }
}
// }}}

// {{{ struct Plan
#[derive(Clone, Debug)]
struct Plan {
    data: Vec<Vec<usize>>,
    scores: Vec<f64>, // 一日の不満
    score: f64,       // 平均不満度
}
impl Plan {
    fn new(data: Vec<Vec<usize>>) -> Self {
        let days = data.len();
        let scores = vec![0.0; days];
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
// }}}

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
    let position: Vec<_> = (0..n)
        .map(|_| {
            let x: i64 = sc.cin();
            let y: i64 = sc.cin();
            (x, y)
        })
        .collect();

    let graph = Graph::new(n, m, position, edges);
    let mut game = Game::new(graph, d, k);

    // 要らない
    // let mut plan = baseline(&game);
    // game.challenge(&mut plan);
    // trace!(#Baseline, plan.score);

    let norma = game.k;
    let mut plan = disjoint_planning(&game, norma);
    game.challenge(&mut plan);
    trace!(#Disjoint, norma, plan.score);

    let norma = (game.k + ((game.k + game.days - 1) / game.days)) / 2;
    let mut plan = disjoint_planning(&game, norma);
    game.challenge(&mut plan);
    trace!(#Disjoint, norma, plan.score);

    // let mut plan = kmeans_planning(&game);
    // game.challenge(&mut plan);
    // trace!(#KMeans, plan.score);

    trace!(#Score, game.bestscore);
    game.submit();
}

/// 適当に振り分けてる
/// 全く強くないのでやらなくて良い
fn baseline(game: &Game) -> Plan {
    let mut data = vec![vec![]; game.days];
    for i in 0..game.graph.m {
        let j = i % game.days;
        data[j].push(i);
    }
    Plan::new(data)
}

// 辺どうしが共通頂点を持たないように選ぶ
fn disjoint_planning(game: &Game, norma: usize) -> Plan {
    let mut data = vec![vec![]; game.days];
    let mut edge_ids: BTreeSet<usize> = (0..game.graph.m).collect();
    let mut d = 0;
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

/// 実用性がまだない
/// 辺に座標を割り当てて k-means クラスタリングをやる
/// それぞれのクラスタから選ぶことでできるだけバラバラなものを選べる
fn kmeans_planning(game: &Game) -> Plan {
    let num_clusters = (game.graph.m + game.days - 1) / game.days;
    fn dist(p: (i64, i64), q: (i64, i64)) -> i64 {
        (p.0 - q.0).pow(2) + (p.1 - q.1).pow(2)
    }
    fn add(p: (i64, i64), q: (i64, i64)) -> (i64, i64) {
        (p.0 + q.0, p.1 + q.1)
    }
    let pos: Vec<(i64, i64)> = (0..game.graph.m)
        .map(|i| {
            let (u, v, _) = game.graph.edges[i];
            let (xu, yu) = game.graph.position[u];
            let (xv, yv) = game.graph.position[v];
            (xu + xv, yu + yv)
        })
        .collect();

    let mut g: Vec<(i64, i64)> = pos[0..num_clusters].iter().cloned().collect();
    for _ in 0..10 {
        let mut h = vec![(0, 0); g.len()];
        for &(x, y) in pos.iter() {
            let (_, i) = (0..g.len()).map(|i| (dist((x, y), g[i]), i)).min().unwrap();
            h[i] = add(g[i], (x, y));
        }
        g = h;
    }

    // clusters[i] = クラスタ i に所属するエッジ集合
    let mut clusters = vec![vec![]; num_clusters];
    for i in 0..game.graph.m {
        let (x, y) = pos[i];
        let (_, k) = (0..g.len()).map(|i| (dist((x, y), g[i]), i)).min().unwrap();
        clusters[k].push(i);
    }

    let mut data = vec![vec![]; game.days];
    let mut num = 0;
    loop {
        for d in 0..game.days {
            for k in 0..num_clusters {
                if let Some(i) = clusters[k].pop() {
                    data[d].push(i);
                    num += 1;
                }
            }
            if num >= game.graph.m {
                break;
            }
        }
        if num >= game.graph.m {
            break;
        }
    }

    Plan::new(data)
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
