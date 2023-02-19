#![allow(unused_imports, unused_macros, dead_code)]
use std::ops::Range;
use std::process::exit;
use std::{cmp::*, collections::*};
use Hyper::*;

type P = (i128, i128);
type Pset = BTreeSet<P>;

#[derive(Debug, PartialEq, Eq)]
enum Dig {
    Broken,
    Rest,
    Panic,
}

#[derive(Debug, Clone)]
struct Game {
    n: i128,
    c: i128,
    waters: Pset,
    homes: Pset,
    broken: Pset,
}
impl Game {
    fn new(n: i128, c: i128, waters: Pset, homes: Pset) -> Self {
        let broken = BTreeSet::new();
        Self {
            n,
            c,
            waters,
            homes,
            broken,
        }
    }
    /// あるセルの4隣接点
    fn neigh(&self, p: P) -> Vec<P> {
        let dxy = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut r = vec![];
        for &(dx, dy) in dxy.iter() {
            let q = (p.0 + dx, p.1 + dy);
            if q.0 < 0 || q.0 > self.n || q.1 < 0 || q.1 > self.n {
                continue;
            }
            r.push(q);
        }
        r
    }
    /// あるセルを掘る
    fn dig(&mut self, p: P, power: i128) -> Dig {
        if self.broken.contains(&p) {
            return Dig::Broken;
        }
        println!("{} {} {}", p.0, p.1, power);
        flush();
        let mut sc = Scanner::default();
        let res: i32 = sc.cin();
        match res {
            0 => Dig::Rest,
            1 => {
                self.broken.insert(p);
                Dig::Broken
            }
            2 => exit(0),
            _ => panic!("Invalid dig({:?}, {}) => {}", p, power, res),
        }
    }
    /// 確実に掘れるまで掘る
    fn dig_full(&mut self, p: P) {
        let powers = vec![100, 500, 1000, 5000];
        for power in powers {
            self.dig(p, power);
        }
    }
    /// 矩形区間を同じパワーで全部掘る
    /// x_range, y_range を辺に持つ矩形を選ぶ
    fn dig_range(&mut self, x_range: Range<i128>, y_range: Range<i128>, power: i128) {
        println!("# Digging range({:?}, {:?})", &x_range, &y_range);
        for x in x_range {
            for y in y_range.clone() {
                self.dig((x, y), power);
            }
        }
    }
    /// 線分上を掘る
    /// 厚みを持たせてやや周りも掘る
    fn dig_line(&mut self, s: P, t: P, width: f64, power: i128, akirame: usize) {
        println!("# Digging line({:?} => {:?})", s, t);
        fn to(a: i128, b: i128) -> Vec<i128> {
            if a <= b {
                (a..=b).collect()
            } else {
                (b..=a).rev().collect()
            }
        }
        let mut num = 0;
        for x in to(s.0, t.0) {
            for y in to(s.1, t.1) {
                let k = ((t.1 - s.1) * x - (t.0 - s.0) * y - (t.1 - s.1) * s.0 + (t.0 - s.0) * s.1)
                    .abs() as f64;
                let den = (((t.0 - s.0).pow(2) + (t.1 - s.1).pow(2)) as f64).sqrt();
                let dist = k / den;
                if dist <= width {
                    let res = self.dig((x, y), power);
                    if res == Dig::Rest {
                        num += 1
                    } else {
                        num = 0;
                    }
                    if num >= 5 {
                        println!("# akirame");
                        return;
                    }
                }
            }
        }
    }
    /// 水を伝播させる
    fn waterflow(&mut self) {
        let mut q: Vec<P> = self.waters.clone().into_iter().collect();
        let mut appendwaters = BTreeSet::new();
        let mut checked = BTreeSet::new();
        while let Some(u) = q.pop() {
            for v in self.neigh(u) {
                if !self.broken.contains(&v) {
                    continue;
                }
                if checked.contains(&v) {
                    continue;
                }
                checked.insert(v);
                if !self.waters.contains(&v) && !appendwaters.contains(&v) {
                    appendwaters.insert(v);
                    q.push(v);
                }
            }
        }
        self.waters.extend(appendwaters);
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: i128 = sc.cin();
    let w: usize = sc.cin();
    let k: usize = sc.cin();
    let c: i128 = sc.cin();
    // 水源
    let waters: Pset = (0..w)
        .map(|_| {
            let x: i128 = sc.cin();
            let y: i128 = sc.cin();
            (x, y)
        })
        .collect();
    // 家
    let homes: Pset = (0..k)
        .map(|_| {
            let x: i128 = sc.cin();
            let y: i128 = sc.cin();
            (x, y)
        })
        .collect();
    trace!(&waters);
    trace!(&homes);
    let mut game = Game::new(n, c, waters, homes);

    // 全ての家のセルを先に破壊する
    {
        for p in game.homes.clone() {
            println!("# Dig-full home({:?})", &p);
            game.dig_full(p);
        }
    }

    // 全ての水源を掘る
    {
        for p in game.waters.clone() {
            println!("# Dig-full home({:?})", &p);
            game.dig_full(p);
        }
    }

    loop {
        // 各家から水源までの最短路を見つける
        let mut ps: Pset = BTreeSet::new();
        ps.extend(game.homes.clone());
        for i in 0..game.n {
            for j in 0..game.n {
                let p = (i, j);
                if !game.homes.contains(&p) && !game.waters.contains(&p) && game.broken.contains(&p)
                {
                    ps.insert((i, j));
                }
            }
        }
        // 家+空きマスで kNN グラフ
        let mut neigh = knn_graph(&ps, 2);
        // 全点 -> 水源を追加
        for &p in ps.iter() {
            for &w in game.waters.iter() {
                let d = dist(p, w);
                neigh.entry(p).and_modify(|nears| nears.push((w, d)));
            }
        }
        ps.extend(game.waters.clone());

        let memo: BTreeMap<P, Hyper<i128>> = ps.iter().map(|&p| (p, Inf)).collect();
        for home in game.homes.clone() {
            // すでに水路が引かれている
            if game.waters.contains(&home) {
                continue;
            }
            // Dijkstra-path
            let path: Vec<(i128, i128)> = {
                let mut path = vec![];
                let mut memo = memo.clone();
                memo.insert(home, Real(0));
                let mut from = BTreeMap::new();
                let mut q = BinaryHeap::new();
                q.push((Reverse(Real(0)), home));
                while let Some((Reverse(d), u)) = q.pop() {
                    if memo[&u] != d {
                        continue;
                    }
                    // goal.
                    if game.waters.contains(&u) {
                        path.push(u);
                        loop {
                            let last = path[path.len() - 1];
                            if let Some(&v) = from.get(&last) {
                                path.push(v);
                                continue;
                            }
                            break;
                        }
                        break;
                    }
                    for &(v, d) in neigh[&u].iter() {
                        let d = d as i128;
                        if memo[&v] > memo[&u] + d {
                            memo.insert(v, memo[&u] + d);
                            from.insert(v, u);
                            q.push((Reverse(memo[&v]), v));
                        }
                    }
                }
                path
            };
            println!(
                "# Dijkstra path: home=({:?}) -> water=({:?})",
                path[path.len() - 1],
                path[0],
            );
            trace!(&path);
            for i in 1..path.len() {
                let p = path[i - 1];
                let q = path[i];
                game.dig_line(p, q, 1.0, 500, 20);
            }
            game.waterflow();
        }
    }
}

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

fn dist(x: (i128, i128), y: (i128, i128)) -> i128 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

/// ps を頂点とする kNN グラフを作る
/// 近似で作る: https://cympfh.cc/paper/eff-knn-graph
fn knn_graph(
    ps: &BTreeSet<(i128, i128)>,
    k: usize,
) -> BTreeMap<(i128, i128), Vec<((i128, i128), i128)>> {
    let ps: Vec<(i128, i128)> = ps.iter().cloned().collect();
    let n = ps.len();
    let mut rand = XorShift::new();

    let mut knn = BTreeMap::new(); // p から見た有向 k-近傍
    for &p in ps.iter() {
        let idx = rand.sample(0..n, k + 1);
        let mut nears = BinaryHeap::new();
        for i in idx {
            let q = ps[i];
            let d = dist(p, q);
            nears.push((Reverse(d), q));
        }
        knn.insert(p, nears);
    }
    // p の (無向) 近傍, これは knn に逆辺を加えたもの
    let mut u = knn.clone();
    for &p in ps.iter() {
        for &(Reverse(d), q) in knn[&p].iter() {
            u.entry(q).and_modify(|nears| nears.push((Reverse(d), p)));
        }
    }

    // TODO(ここに肝心のアルゴリズムを各)

    // knn を隣接リストに変換して返却
    // 自己辺を除く
    let mut neigh = BTreeMap::new();
    for &p in ps.iter() {
        let mut nears = vec![];
        for &(Reverse(d), q) in knn[&p].iter() {
            if p != q {
                nears.push((q, d));
            }
        }
        neigh.insert(p, nears);
    }
    neigh
}

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
    pub fn shuffle<T>(&mut self, xs: &mut Vec<T>) {
        let n = xs.len();
        for i in (1..n).rev() {
            let j = self.gen::<usize>() % i;
            if i != j {
                xs.swap(i, j);
            }
        }
    }
    pub fn sample(&mut self, range: std::ops::Range<usize>, k: usize) -> Vec<usize> {
        let mut r: Vec<usize> = range.collect();
        if r.len() <= k {
            r
        } else {
            self.shuffle(&mut r);
            r.truncate(k);
            r.sort();
            r
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
