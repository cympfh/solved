#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let d: Vec<usize> = sc.vec(n);
    trace!(&d);

    use Hyper::*;
    let mut g = Graph::new(n);
    for _ in 1..n {
        let u = sc.usize1();
        let v = sc.usize1();
        let w: i64 = sc.cin();
        g.uedge_with_cost(u, v, Real(w));
    }
    let g = g.to_rooted(0);

    let mut dp_leq = vec![NegInf; n];
    let mut dp_le = vec![NegInf; n];

    let mut dfs = DFS::default();
    dfs.push(&0);
    while let Some(next) = dfs.pop() {
        match next {
            DfsOrd::Pre(u) => {
                for v in g.neigh(u) {
                    dfs.push(&v);
                }
            }
            DfsOrd::Post(u) => {
                trace!(u);
                let mut children = vec![];
                for (v, cost) in g.neigh_with_cost(u) {
                    // cost of (connected, unconnected)
                    children.push((cost + dp_le[v], dp_leq[v]));
                }
                children.sort_by_key(|&(c1, c2)| c2 - c1);
                trace!(d[u], &children);
                {
                    dp_leq[u] = Real(0);
                    for i in 0..children.len() {
                        dp_leq[u] += if i < d[u] && children[i].0 > children[i].1 {
                            children[i].0
                        } else {
                            children[i].1
                        };
                    }
                }
                if d[u] == 0 {
                    dp_le[u] = NegInf;
                } else {
                    dp_le[u] = Real(0);
                    for i in 0..children.len() {
                        dp_le[u] += if i + 1 < d[u] && children[i].0 > children[i].1 {
                            children[i].0
                        } else {
                            children[i].1
                        };
                    }
                }
                trace!(u, dp_leq[u], dp_le[u]);
            }
        }
    }

    let ans = max!(dp_leq[0], dp_le[0]);
    trace!(ans);
    put!(ans.unwrap());
}

// @algorithm/dfs
#[derive(Debug, Default)]
pub struct DFS<X> {
    pub stack: Vec<DfsOrd<X>>,
}
impl<X: Clone> DFS<X> {
    pub fn push(&mut self, x: &X) {
        self.stack.push(DfsOrd::Post(x.clone()));
        self.stack.push(DfsOrd::Pre(x.clone()));
    }
    pub fn pop(&mut self) -> Option<DfsOrd<X>> {
        self.stack.pop()
    }
}
#[derive(Debug)]
pub enum DfsOrd<X> {
    Pre(X),
    Post(X),
}

// @graph/graph
// @algebra/hyper
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

/// Graph class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub n: usize,
    pub data: Vec<Vec<usize>>,
    pub cost: Vec<Vec<Hyper<i64>>>,
}
impl Graph {
    pub fn new(n: usize) -> Self {
        let data = vec![vec![]; n];
        let cost = vec![vec![]; n];
        Self { n, data, cost }
    }
    /// undirected edge
    pub fn uedge(&mut self, u: usize, v: usize) {
        self.dedge(u, v);
        self.dedge(v, u);
    }
    /// directed edge
    pub fn dedge(&mut self, u: usize, v: usize) {
        self.dedge_with_cost(u, v, Hyper::Real(1));
    }
    /// undirected edge + cost
    pub fn uedge_with_cost(&mut self, u: usize, v: usize, cost: Hyper<i64>) {
        self.dedge_with_cost(u, v, cost);
        self.dedge_with_cost(v, u, cost);
    }
    /// directed edge + cost
    pub fn dedge_with_cost(&mut self, u: usize, v: usize, cost: Hyper<i64>) {
        self.data[u].push(v);
        self.cost[u].push(cost);
    }
    /// adj list
    pub fn neigh(&self, u: usize) -> Vec<usize> {
        self.data[u].to_vec()
    }
    /// adj list + cost
    pub fn neigh_with_cost(&self, u: usize) -> Vec<(usize, Hyper<i64>)> {
        self.data[u]
            .iter()
            .cloned()
            .zip(self.cost[u].iter().cloned())
            .collect()
    }
    /// edges have been costed?
    pub fn is_costed(&self) -> bool {
        self.cost.iter().any(|v| !v.is_empty())
    }
    pub fn reverse(&self) -> Self {
        -(self.clone())
    }
    /// undirected -> directed rooted tree
    pub fn to_rooted(&self, root: usize) -> Self {
        let mut r = Graph::new(self.n);
        let mut stack = vec![root];
        let mut visited = vec![false; self.n];
        while let Some(u) = stack.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            for (v, cost) in self.neigh_with_cost(u) {
                if visited[v] {
                    continue;
                }
                r.dedge_with_cost(u, v, cost);
                stack.push(v);
            }
        }
        r
    }
    /// -> adj matrix
    pub fn to_matrix(&self) -> Vec<Vec<Hyper<i64>>> {
        let mut mat = vec![vec![Hyper::Inf; self.n]; self.n];
        for u in 0..self.n {
            for (v, cost) in self.neigh_with_cost(u) {
                mat[u][v] = mat[u][v].min(cost);
            }
            mat[u][u] = Hyper::Real(0);
        }
        mat
    }
}
impl std::ops::Neg for Graph {
    type Output = Graph;
    fn neg(self) -> Self::Output {
        let mut r = Graph::new(self.n);
        for u in 0..self.n {
            for (v, cost) in self.neigh_with_cost(u) {
                r.dedge_with_cost(v, u, cost);
            }
        }
        r
    }
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
