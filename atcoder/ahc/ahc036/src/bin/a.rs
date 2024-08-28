#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Move(usize),
    Signal(usize, usize, usize),
    Nop,
}

#[derive(Debug, Clone)]
pub struct Game {
    n: usize,
    la: usize,
    lb: usize,
    a: Vec<usize>,
    cluster: Cluster,
    signal: Signal,
    action_history: Vec<Action>,
}
impl Game {
    fn new(n: usize, la: usize, lb: usize, cluster: Cluster) -> Self {
        let a = cluster.make_a(la);
        Game {
            n,
            la,
            lb,
            a,
            cluster,
            signal: Signal::new(),
            action_history: vec![],
        }
    }
    fn do_move(&mut self, i: usize) {
        self.action_history.push(Action::Move(i));
    }
    fn do_signal(&mut self, len: usize, pa: usize, pb: usize) {
        self.action_history.push(Action::Signal(len, pa, pb));
        self.signal.update(&self.a[pa..pa + len]);
    }
    /// make u be blue, forcelly
    fn make_blue(&mut self, u: usize) {
        let len = self.lb;
        let pb = 0;
        let pa = self.cluster.segment(u, len);
        self.do_signal(len, pa, pb);
    }
    fn score(&self) -> usize {
        let mut r = 0;
        for &act in self.action_history.iter() {
            if let Action::Signal(_, _, _) = act {
                r += 1;
            }
        }
        r
    }
    fn submit(&self) {
        put!(..self.a);
        for &act in self.action_history.iter() {
            match act {
                Action::Move(i) => {
                    println!("m {}", i);
                }
                Action::Signal(len, pa, pb) => {
                    println!("s {} {} {}", len, pa, pb);
                }
                Action::Nop => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Signal {
    blues: HashSet<usize>,
}
impl Signal {
    fn new() -> Self {
        Signal {
            blues: HashSet::new(),
        }
    }
    fn clear(&mut self) {
        self.blues.clear();
    }
    fn update(&mut self, blues: &[usize]) {
        self.clear();
        for &b in blues {
            self.blues.insert(b);
        }
    }
    fn is_blue(&self, i: usize) -> bool {
        self.blues.contains(&i)
    }
}

#[derive(Debug, Clone)]
pub struct Cluster {
    labels: Vec<usize>,
    flatten: Vec<usize>,
    flatten_labels: Vec<usize>,
    index_of: Vec<usize>,
}
impl Cluster {
    fn new(n: usize, labels: Vec<usize>, rand: &mut XorShift) -> Self {
        let mut ls: Vec<(usize, usize)> = (0..n).map(|i| (i, labels[i])).collect();
        ls.sort_by_key(|&(_, label)| {
            let noise = rand.gen::<usize>();
            (label, noise)
        });
        let mut flatten = vec![];
        let mut flatten_labels: Vec<usize> = vec![];
        for &(i, label) in ls.iter() {
            flatten.push(i);
            flatten_labels.push(label);
        }
        let mut index_of = vec![0; n];
        for i in 0..n {
            index_of[flatten[i]] = i;
        }
        Self {
            labels,
            flatten,
            flatten_labels,
            index_of,
        }
    }
    /// i を含む長さ length のセグメントを返す
    /// できるだけ同じクラスタの中のセグメントを作る
    /// セグメントのオフセットが返り値
    fn segment(&self, i: usize, length: usize) -> usize {
        let mut r = self.index_of[i];
        let label = self.flatten_labels[self.index_of[i]];
        if r + length >= self.flatten_labels.len() {
            r = self.flatten_labels.len() - length;
        }
        for _ in 0..24 {
            if r == 0 {
                break;
            }
            if self.flatten_labels[r - 1] == label && self.flatten_labels[r + length - 2] != label {
                r -= 1;
            } else {
                break;
            }
        }
        r
    }
    fn make_a(&self, length: usize) -> Vec<usize> {
        let mut r = self.flatten.clone();
        while r.len() < length {
            r.push(0);
        }
        r
    }
    fn dump(&self) {
        let mut group = vec![];
        let mut last_label = 98989898989;
        for &i in self.flatten.iter() {
            let label = self.labels[i];
            if label != last_label && !group.is_empty() {
                trace!(label, ..&group);
                group.clear();
            }
            last_label = label;
            group.push(i);
        }
        put!(..group);
    }
}

// fn validate(game: &Game, tour: &Vec<usize>, mat: &Vec<Vec<bool>>) -> bool {
//     let mut u = 0;
//     let mut nextgoal = 0;
//     let mut signal = Signal::new();
//     for &act in game.action_history.iter() {
//         match act {
//             Action::Move(v) => {
//                 if !mat[u][v] || !signal.is_blue(v) {
//                     return false;
//                 }
//                 u = v;
//                 if tour[nextgoal] == v {
//                     nextgoal += 1;
//                     if nextgoal == tour.len() {
//                         return true;
//                     }
//                 }
//             }
//             Action::Signal(len, pa, _pb) => {
//                 signal.update(&game.a[pa..pa + len]);
//             }
//             Action::Nop => {}
//         }
//     }
//     false
// }

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let t: usize = sc.cin();
    let la: usize = sc.cin();
    let lb: usize = sc.cin();
    let mut g = vec![vec![]; n];
    let mut mat = vec![vec![false; n]; n];
    for _ in 0..m {
        let u: usize = sc.cin();
        let v: usize = sc.cin();
        g[u].push(v);
        g[v].push(u);
        mat[u][v] = true;
        mat[v][u] = true;
    }
    let tour: Vec<usize> = sc.vec(t);
    let mut pos: Vec<(f64, f64)> = vec![];
    for _ in 0..n {
        let x: f64 = sc.cin();
        let y: f64 = sc.cin();
        pos.push((x, y));
    }

    let num_cluster = max!(1, (la + lb - 1) / lb);
    // let cluster_labels = kmeans::xy(&pos, num_cluster, 0);
    let cluster_labels = kmeans::graph(num_cluster, &g);
    trace!(&cluster_labels);

    // Debug dump K-means
    // {
    //     for i in 0..n {
    //         println!("{} {} {}", pos[i].0, pos[i].1, cluster_labels[i]);
    //     }
    //     return;
    // }

    let mut best_game = None;
    let mut best_score = None;
    let mut rand = XorShift::new();

    loop_timeout_ms!(1900; {
        let cluster = Cluster::new(n, cluster_labels.clone(), &mut rand);
        let mut game = Game::new(n, la, lb, cluster.clone());
        let mut cur = 0;
        for &goal in tour.iter() {
            if cur == goal {
                continue;
            }
            let route = dijkstra_route(cur, goal, &g);
            for i in 1..route.len() {
                let u = route[i];
                if !game.signal.is_blue(u) {
                    game.make_blue(u);
                }
                game.do_move(u);
            }
            cur = goal;
        }

        trace!(game.score());
        if best_score.is_none() || best_score.unwrap() > game.score() {
            trace!(#NewRecord game.score());
            best_score = Some(game.score());
            best_game = Some(game);
        }
    });

    eprintln!("score: {}", best_score.unwrap_or(999));
    best_game.unwrap().submit();
}

/// Graph - Dijkstra
/// Returns a path from start to goal
pub fn dijkstra_route(start: usize, goal: usize, neigh: &Vec<Vec<usize>>) -> Vec<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = neigh.len();
    let mut d: Vec<Hyper<i64>> = vec![Hyper::Inf; n];
    let mut q = BinaryHeap::new();
    d[start] = Hyper::Real(0);
    q.push((Reverse(d[start]), start));
    while let Some((_, u)) = q.pop() {
        if u == goal {
            continue;
        }
        for &v in neigh[u].iter() {
            if d[v] > d[u] + 1 {
                d[v] = d[u] + 1;
                q.push((Reverse(d[v]), v));
            }
        }
    }
    let mut route = vec![goal];
    let mut u = goal;
    while u != start {
        for &v in neigh[u].iter() {
            if d[v] + 1 == d[u] {
                u = v;
                break;
            }
        }
        route.push(u);
    }
    route.reverse();
    route
}

/// Graph - Dijkstra
/// Returns a min cost table from start
pub fn dijkstra_cost(start: usize, neigh: &Vec<Vec<usize>>) -> Vec<Hyper<i64>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = neigh.len();
    let mut d: Vec<Hyper<i64>> = vec![Hyper::Inf; n];
    let mut q = BinaryHeap::new();
    d[start] = Hyper::Real(0_i64);
    q.push((Reverse(d[start]), start));
    while let Some((_, u)) = q.pop() {
        for &v in neigh[u].iter() {
            if d[v] > d[u] + 1 {
                d[v] = d[u] + 1;
                q.push((Reverse(d[v]), v));
            }
        }
    }
    d
}

// @/algebra/hyper
// @algebra/group_additive
/// Algebra - AGroup (Additive Group) (+, -, 0)
pub trait AGroup:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::iter::Sum
where
    Self: std::marker::Sized,
{
    fn zero() -> Self;
}

#[macro_export]
macro_rules! agroup {
    (
        $type:ty where [ $( $params:tt )* ] ;
        zero = $zero:expr ;
        add($self:ident, $y:ident) = $code:block ;
        neg($self_neg:ident) = $code_neg:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Add for $type {
            type Output = Self;
            fn add($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::Neg for $type {
            type Output = Self;
            fn neg($self_neg) -> Self { $code_neg }
        }
        impl<$($params)*> std::ops::Sub for $type {
            type Output = Self;
            fn sub($self, other: Self) -> Self { ($self) + (-other) }
        }
        impl<$($params)*> std::ops::AddAssign for $type where Self: Clone {
            fn add_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() + $y;
            }
        }
        impl<$($params)*> std::ops::SubAssign for $type where Self: Clone {
            fn sub_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() - $y;
            }
        }
        impl<$($params)*> std::iter::Sum for $type {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), std::ops::Add::add)
            }
        }
        impl<$($params)*> AGroup for $type {
            fn zero() -> Self { $zero }
        }
    };
    (
        $type:ty ;
        zero = $zero:expr ;
        add($self:ident, $y:ident) = $code:block ;
        neg($self_neg:ident) = $code_neg:block
        $(;)*
    ) => {
        agroup! { $type where []; zero = $zero; add($self, $y) = $code; neg($self_neg) = $code_neg; }
    };
}

impl AGroup for i64 {
    fn zero() -> Self {
        0
    }
}
impl AGroup for i128 {
    fn zero() -> Self {
        0
    }
}
impl AGroup for f64 {
    fn zero() -> Self {
        0.0
    }
}

// @algebra/monoid
/// Algebra - Def of Monoid (*, 1)
pub trait Monoid: std::ops::Mul<Output = Self> + std::iter::Product
where
    Self: std::marker::Sized,
{
    fn one() -> Self;
}

#[macro_export]
macro_rules! monoid {
    (
        $type:ty where [ $( $params:tt )* ];
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Mul for $type {
            type Output = Self;
            fn mul($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::MulAssign for $type where Self: Clone {
            fn mul_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() * $y;
            }
        }
        impl<$($params)*> std::iter::Product for $type {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::one(), std::ops::Mul::mul)
            }
        }
        impl<$($params)*> Monoid for $type {
            fn one() -> Self { $one }
        }
    };
    (
        $type:ty;
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        monoid! { $type where []; one = $one; mul($self, $y) = $code; }
    };
}

impl Monoid for i64 {
    fn one() -> Self {
        1
    }
}
impl Monoid for i128 {
    fn one() -> Self {
        1
    }
}
impl Monoid for f64 {
    fn one() -> Self {
        1.0
    }
}

// @algebra/ring
/// Algebra - Ring ((+, 0), (*, 1))
pub trait Ring: AGroup + Monoid {}

#[macro_export]
macro_rules! ring {
    (
        $type:ty where [ $( $params:tt )* ];
        div($self:ident, $other:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Div for $type {
            type Output = Self;
            fn div($self, $other: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::DivAssign for $type where Self: Clone {
            fn div_assign(&mut $self, $other: Self) { *$self = (*$self).clone() / $other; }
        }
        impl Ring for $type {}
    };
    (
        $type:ty;
        div($self:ident, $other:ident) = $code:block
        $(;)*
    ) => {
        ring! { $type where []; div($self, $other) = $code; }
    };
}

impl Ring for i64 {}
impl Ring for f64 {}

/// Algebra - Hyper Numbers (numbers with infinity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hyper<X> {
    NegInf,
    Real(X),
    Inf,
}
use Hyper::{Inf, NegInf, Real};
impl<X> Hyper<X> {
    pub fn unwrap(self) -> X {
        if let Hyper::Real(x) = self {
            x
        } else {
            panic!("Could not unwrap Hyper")
        }
    }
}
agroup! {
    Hyper<X> where [X: AGroup];
    zero = Real(X::zero());
    add(self, other) = {
        match (self, other) {
            (Real(x), Real(y)) => Real(x + y),
            (Inf, _) => Inf,
            (_, Inf) => Inf,
            _ => NegInf,
        }
    };
    neg(self) = {
        match self {
            Inf => NegInf,
            NegInf => Inf,
            Real(x) => Real(-x),
        }
    };
}
monoid! {
    Hyper<X> where [X: Monoid];
    one = Real(X::one());
    mul(self, other) = {
        match (self, other) {
            (Real(x), Real(y)) => Real(x * y),
            (Inf, Inf) | (NegInf, NegInf) => Inf,
            _ => NegInf,
        }
    };
}
impl<X: AGroup + Monoid> Ring for Hyper<X> {}
impl<X: std::ops::Add<Output = X>> std::ops::Add<X> for Hyper<X> {
    type Output = Self;
    fn add(self, y: X) -> Hyper<X> {
        match (self, y) {
            (Real(x), y) => Real(x + y),
            (Inf, _) => Inf,
            _ => NegInf,
        }
    }
}
impl<X: Clone + AGroup> std::ops::AddAssign<X> for Hyper<X> {
    fn add_assign(&mut self, y: X) {
        *self = (*self).clone() + Real(y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct P(f64, f64);
impl P {
    fn new(x: f64, y: f64) -> P {
        P(x, y)
    }
    fn distance(&self, other: &P) -> f64 {
        let mut r = 0.0;
        r += (self.0 - other.0).powi(2);
        r += (self.1 - other.1).powi(2);
        r
    }
    fn equals(&self, other: &P) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
    fn zero() -> Self {
        P(0.0, 0.0)
    }
    fn max() -> Self {
        P(f64::MAX, f64::MAX)
    }
}
impl std::ops::AddAssign<P> for P {
    fn add_assign(&mut self, other: P) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl std::ops::DivAssign<f64> for P {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
    }
}

pub mod kmeans {
    /// xy 二次元上の普通の k-means
    pub fn xy(pos: &Vec<(f64, f64)>, num_cluster: usize, capacity: usize) -> Vec<usize> {
        let pos: Vec<P> = pos.iter().map(|&(x, y)| P::new(x, y)).collect();
        let mut centroids: Vec<P> = (0..num_cluster).map(|i| pos[i]).collect();
        let mut labels = vec![0; pos.len()];
        for _time in 0..300 {
            let mut counts = vec![0; num_cluster];
            let mut _score = 0.0;
            for (i, &p) in pos.iter().enumerate() {
                let mut min_dist = f64::MAX;
                let mut min_idx = 0;
                for (j, &centroid) in centroids.iter().enumerate() {
                    if capacity > 0 && counts[j] >= capacity {
                        continue;
                    }
                    let dist = centroid.distance(&p);
                    if dist < min_dist {
                        min_dist = dist;
                        min_idx = j;
                    }
                }
                labels[i] = min_idx;
                counts[min_idx] += 1;
                _score += min_dist;
            }
            trace!(#kmeans _time, _score);
            // セントロイドの更新
            let mut new_centroids = vec![P::zero(); num_cluster];
            for (i, &label) in labels.iter().enumerate() {
                new_centroids[label] += pos[i];
            }
            for i in 0..num_cluster {
                if counts[i] > 0 {
                    new_centroids[i] /= counts[i] as f64;
                } else {
                    new_centroids[i] = P::max();
                }
            }
            if centroids == new_centroids {
                break;
            }
            centroids = new_centroids;
        }
        labels
    }

    // グラフ上の距離で k-means
    pub fn graph(num_cluster: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
        let n = g.len();
        let mut centroids: Vec<usize> = (0..num_cluster).collect();
        let mut labels = vec![0; n];
        // cost table
        let mut d = vec![];
        for i in 0..n {
            d.push(dijkstra_cost(i, g));
        }
        // グラフ g の上で group の重心らしきところを探す
        fn get_centroid(group: &Vec<usize>, d: &Vec<Vec<Hyper<i64>>>) -> usize {
            let mut r = vec![];
            let n = d.len();
            for u in 0..n {
                let mut sum = Hyper::Real(0);
                for &v in group.iter() {
                    sum = sum + d[u][v];
                }
                r.push((sum, u));
            }
            r.sort();
            r[0].1
        }
        for _time in 0..300 {
            let mut groups = vec![vec![]; num_cluster];
            let mut _score = 0;
            for u in 0..n {
                let mut min_dist = Hyper::Inf;
                let mut min_idx = 0;
                for j in 0..num_cluster {
                    let v = centroids[j];
                    let dist = d[u][v];
                    if dist < min_dist {
                        min_dist = dist;
                        min_idx = j;
                    }
                }
                labels[u] = min_idx;
                groups[min_idx].push(u);
                _score += min_dist.unwrap();
            }
            trace!(#graph_kmeans _time, _score);
            let mut new_centroids = vec![0; num_cluster];
            for j in 0..num_cluster {
                new_centroids[j] = get_centroid(&groups[j], &d);
            }
            if centroids == new_centroids {
                break;
            }
            centroids = new_centroids;
            trace!(#centroids &centroids)
        }
        labels
    }
}

// @/num/random/xorshift
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
    pub fn shuffle<T>(&mut self, xs: &mut Vec<T>) {
        let n = xs.len();
        for i in (1..n).rev() {
            let j = self.gen::<usize>() % (i + 1);
            if i != j {
                xs.swap(i, j);
            }
        }
    }
}

// @/datetime/timed_loop
/// Time Limited Loop
#[macro_export]
macro_rules! loop_timeout_ms {
    ( $milli_seconds:expr; $body:expr ) => {
        let now = std::time::SystemTime::now();
        loop {
            match now.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_millis() > $milli_seconds {
                        break;
                    }
                    $body
                }
                Err(e) => {
                    eprintln!("Err, {:?}", e);
                }
            }
        }
    };
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
