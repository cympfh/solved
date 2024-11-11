#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone)]
pub struct Game {
    rand: XorShift,
    best: Option<Solution>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            rand: XorShift::new(),
            best: None,
        }
    }
    fn update(&mut self, solution: Solution) {
        match &self.best {
            None => {
                self.best = Some(solution);
            }
            Some(best) if best.score < solution.score => {
                self.best = Some(solution);
            }
            _ => {}
        }
    }
    fn show(&self) {
        match &self.best {
            Some(best) => {
                best.dump();
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct Solution {
    n: usize,
    ps: Vec<(usize, usize)>,
    score: i64,
}
impl Solution {
    fn new(ps: Vec<(usize, usize)>, score: i64) -> Self {
        Self {
            n: ps.len(),
            ps,
            score,
        }
    }
    fn dump(&self) {
        println!("{}", self.n);
        for &(x, y) in self.ps.iter() {
            println!("{} {}", x, y);
        }
    }
}

struct Index<T> {
    map: BTreeMap<T, usize>,
}
impl<T: Clone + Ord + Eq> Index<T> {
    fn new(xset: BTreeSet<T>) -> Self {
        let map: std::collections::BTreeMap<T, usize> = xset
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), i))
            .collect();
        Self { map }
    }
    fn get(&self, t: T) -> usize {
        self.map.get(&t).unwrap().clone()
    }
    fn len(&self) -> usize {
        self.map.len()
    }
}

const LIMIT_VERTEX: usize = 1000;
const LIMIT_LENGTH: usize = 400_000;

fn main() {
    let mut sc = Scanner::default();
    let mut game = Game::new();

    let n: usize = sc.cin();
    let mut sabas: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let x: usize = sc.cin();
            let y: usize = sc.cin();
            (x, y)
        })
        .collect();
    let mut iwashies: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let x: usize = sc.cin();
            let y: usize = sc.cin();
            (x, y)
        })
        .collect();

    // shuffle & sampling
    {
        for i in (0..n).rev() {
            let j = game.rand.gen::<usize>() % (i + 1);
            sabas.swap(i, j);
        }
        for i in (0..n).rev() {
            let j = game.rand.gen::<usize>() % (i + 1);
            iwashies.swap(i, j);
        }
    }
    let n_samples: usize = 1660;
    let sabas: Vec<_> = sabas[0..n_samples].iter().cloned().collect();
    let iwashies: Vec<_> = iwashies[0..n_samples].iter().cloned().collect();

    // 座標圧縮
    let xmap = Index::new(
        sabas
            .iter()
            .map(|(x, _)| *x)
            .chain(iwashies.iter().map(|(x, _)| *x))
            .collect(),
    );
    let ymap = Index::new(
        sabas
            .iter()
            .map(|(_, x)| *x)
            .chain(iwashies.iter().map(|(_, x)| *x))
            .collect(),
    );
    let mut sabamap = ndarray![0_i64; xmap.len(), ymap.len()];
    for &(x, y) in sabas.iter() {
        sabamap[xmap.get(x)][ymap.get(y)] += 1;
    }
    let mut iwashimap = ndarray![0_i64; xmap.len(), ymap.len()];
    for &(x, y) in iwashies.iter() {
        iwashimap[xmap.get(x)][ymap.get(y)] += 1;
    }

    // ２次元累積わ
    let cum_saba = Cumsum2d::new(&sabamap);
    let cum_iwashi = Cumsum2d::new(&iwashimap);

    fn count(
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        xmap: &Index<usize>,
        ymap: &Index<usize>,
        cum: &Cumsum2d,
    ) -> i64 {
        let i0 = xmap.get(x0);
        let i1 = xmap.get(x1) + 1;
        let j0 = ymap.get(y0);
        let j1 = ymap.get(y1) + 1;
        cum.sum(i0..i1, j0..j1)
    }

    // 矩形の場合
    // EXACT
    for i in 0..n_samples {
        for j in i + 1..n_samples {
            let minx = min!(sabas[i].0, sabas[j].0);
            let miny = min!(sabas[i].1, sabas[j].1);
            let maxx = max!(sabas[i].0, sabas[j].0);
            let maxy = max!(sabas[i].1, sabas[j].1);
            let num_saba = count(minx, miny, maxx, maxy, &xmap, &ymap, &cum_saba);
            let num_iwashi = count(minx, miny, maxx, maxy, &xmap, &ymap, &cum_iwashi);
            let score = num_saba - num_iwashi;
            let ps = vec![(minx, miny), (minx, maxy), (maxx, maxy), (maxx, miny)];
            game.update(Solution::new(ps, score));
        }
    }
    game.show();

    // 今一番良い矩形に矩形を付け足す
    let best_rect = game.best.clone().unwrap();
    let (x0, y0) = best_rect.ps[0];
    let (x1, y1) = best_rect.ps[2];
    for i in 0..n_samples {
        for j in i + 1..n_samples {
            let x0_ = min!(sabas[i].0, sabas[j].0);
            let y0_ = min!(sabas[i].1, sabas[j].1);
            let x1_ = max!(sabas[i].0, sabas[j].0);
            let y1_ = max!(sabas[i].1, sabas[j].1);
            // Not overlapped?
            if x1_ <= x0 || x1 <= x0_ || y1_ <= y0 || y1 <= y0_ {
                continue;
            }
            // overlap
            let x0__ = max!(x0, x0_);
            let y0__ = max!(y0, y0_);
            let x1__ = min!(x1, x1_);
            let y1__ = min!(y1, y1_);
            let score2 = count(x0_, y0_, x1_, y1_, &xmap, &ymap, &cum_saba)
                - count(x0_, y0_, x1_, y1_, &xmap, &ymap, &cum_iwashi);
            let score3 = count(x0__, y0__, x1__, y1__, &xmap, &ymap, &cum_saba)
                - count(x0__, y0__, x1__, y1__, &xmap, &ymap, &cum_iwashi);
            let score = best_rect.score + score2 - score3;
            let ps = if x0_ < x0 && x1 < x1_ && y0 < y0_ && y1_ < y1 {
                vec![
                    (x0, y0),
                    (x0__, y0__),
                    (x0_, y0_),
                    (x0_, y1_),
                    (x0__, y1__),
                    (x0, y1),
                    (x1, y1),
                    (x1__, y1__),
                    (x1_, y1_),
                    (x1_, y0_),
                    (x1__, y0__),
                    (x1, y0),
                ]
            } else if x0 < x0_ && x1_ < x1 && y0_ < y0 && y1 < y1_ {
                vec![
                    (x0, y0),
                    (x0__, y0__),
                    (x0_, y0_),
                    (x1_, y0_),
                    (x1__, y0__),
                    (x1, y0),
                    (x1, y1),
                    (x1__, y1__),
                    (x1_, y1_),
                    (x0_, y1_),
                    (x0__, y1__),
                    (x0, y1),
                ]
            } else if x0 < x0_ && x0_ < x1 && x1 < x1_ && y0 < y0_ && y0_ < y1 && y1 < y1_ {
                vec![
                    (x0, y0),
                    (x1, y0),
                    (x1__, y0__),
                    (x1_, y0_),
                    (x1_, y1_),
                    (x0_, y1_),
                    (x0__, y1__),
                    (x0, y1),
                ]
            } else if x0_ < x0 && x0 < x1_ && x1_ < x1 && y0_ < y0 && y0 < y1_ && y1_ < y1 {
                vec![
                    (x0_, y0_),
                    (x1_, y0_),
                    (x1__, y0__),
                    (x1, y0),
                    (x1, y1),
                    (x0, y1),
                    (x0__, y1__),
                    (x0_, y1_),
                ]
            } else if x0_ < x0 && x0 < x1_ && x1_ < x1 && y0 < y0_ && y0_ < y1 && y1 < y1_ {
                vec![
                    (x0_, y0_),
                    (x0__, y0__),
                    (x0, y0),
                    (x1, y0),
                    (x1, y1),
                    (x1__, y1__),
                    (x1_, y1_),
                    (x0_, y1_),
                ]
            } else if x0 < x0_ && x0_ < x1 && x1 < x1_ && y0_ < y0 && y0 < y1_ && y1_ < y1 {
                vec![
                    (x0, y0),
                    (x0__, y0__),
                    (x0_, y0_),
                    (x1_, y0_),
                    (x1_, y1_),
                    (x1__, y1__),
                    (x1, y1),
                    (x0, y1),
                ]
            } else if x0 < x0_ && x0_ < x1 && x1 < x1_ && y0 < y0_ && y1_ < y1 {
                vec![
                    (x0, y0),
                    (x1, y0),
                    (x1__, y0__),
                    (x1_, y0_),
                    (x1_, y1_),
                    (x1__, y1__),
                    (x1, y1),
                    (x0, y1),
                ]
            } else if x0_ < x0 && x0 < x1_ && x1_ < x1 && y0_ < y0 && y1 < y1_ {
                vec![
                    (x0_, y0_),
                    (x1_, y0_),
                    (x1__, y0__),
                    (x1, y0),
                    (x1, y1),
                    (x1__, y1__),
                    (x1_, y1_),
                    (x0_, y1_),
                ]
            } else if x0_ < x0 && x0 < x1_ && x1_ < x1 && y0 < y0_ && y1_ < y1 {
                vec![
                    (x0_, y0_),
                    (x0__, y0__),
                    (x0, y0),
                    (x1, y0),
                    (x1, y1),
                    (x0, y1),
                    (x0__, y1__),
                    (x0_, y1_),
                ]
            } else if x0 < x0_ && x0_ < x1 && x1 < x1_ && y0_ < y0 && y1 < y1_ {
                vec![
                    (x0, y0),
                    (x0__, y0__),
                    (x0_, y0_),
                    (x1_, y0_),
                    (x1_, y1_),
                    (x0_, y1_),
                    (x0__, y1__),
                    (x0, y1),
                ]
            } else {
                vec![]
            };
            if ps.len() > 0 {
                game.update(Solution::new(ps, score));
            }
        }
    }

    eprintln!("{:?}", &game.best);
    game.show();
}

// @sequence/cumsum2d
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

/// Sequence - Cumulative Summation 2D of Additive Group (+, 0)
#[derive(Debug)]
pub struct Cumsum2d(Vec<Vec<i64>>);
impl Cumsum2d {
    pub fn new(data: &Vec<Vec<i64>>) -> Self {
        let h = data.len();
        let w = data[0].len();
        trace!(h, w);
        let mut cs = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                cs[i + 1][j + 1] = data[i][j] + cs[i][j + 1] + cs[i + 1][j] - cs[i][j];
            }
        }
        trace!("done");
        Self(cs)
    }
    fn sum_up(&self, x: usize, y: usize) -> i64 {
        let x = std::cmp::min(x, self.0.len());
        let y = std::cmp::min(y, self.0[0].len());
        self.0[x][y]
    }
    pub fn sum(&self, xrange: std::ops::Range<usize>, yrange: std::ops::Range<usize>) -> i64 {
        if xrange.end <= xrange.start || yrange.end <= yrange.start {
            0
        } else {
            self.sum_up(xrange.end, yrange.end)
                - self.sum_up(xrange.start, yrange.end)
                - self.sum_up(xrange.end, yrange.start)
                + self.sum_up(xrange.start, yrange.start)
        }
    }
}

/// {{{ random
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
#[derive(Debug, Clone)]
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
}
/// }}}
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
