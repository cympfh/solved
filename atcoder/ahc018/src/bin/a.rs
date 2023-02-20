#![allow(unused_imports, unused_macros, dead_code)]
/// アイデア: サンプリングして地盤の硬さマップを作る
/// 推定マップの上で最短パスを BFS 探索する
use std::ops::Range;
use std::process::exit;
use std::{cmp::*, collections::*};

// 水源, W <= 4
// 家, K <= 10

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
    damage: DefaultDict<P, i128>,
}
impl Game {
    fn new(n: i128, c: i128, waters: Pset, homes: Pset) -> Self {
        let broken = BTreeSet::new();
        let damage = DefaultDict::new(0);
        Self {
            n,
            c,
            waters,
            homes,
            broken,
            damage,
        }
    }
    /// あるセルの4隣接点
    fn neigh(&self, p: P) -> Vec<P> {
        let dxy = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut r = vec![];
        for &(dx, dy) in dxy.iter() {
            let q = (p.0 + dx, p.1 + dy);
            if q.0 < 0 || q.0 >= self.n || q.1 < 0 || q.1 >= self.n {
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
        self.damage[p] += power;
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
        if self.broken.contains(&p) {
            return;
        }
        println!("# Digging full({:?})", &p);
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
    fn dig_line(&mut self, s: P, t: P, width: f64, power: i128) {
        println!("# Digging line({:?} => {:?})", s, t);
        fn to(a: i128, b: i128) -> Vec<i128> {
            if a <= b {
                (a..=b).collect()
            } else {
                (b..=a).rev().collect()
            }
        }
        for x in to(s.0, t.0) {
            for y in to(s.1, t.1) {
                let k = ((t.1 - s.1) * x - (t.0 - s.0) * y - (t.1 - s.1) * s.0 + (t.0 - s.0) * s.1)
                    .abs() as f64;
                let den = (((t.0 - s.0).pow(2) + (t.1 - s.1).pow(2)) as f64).sqrt();
                let d = k / den;
                if d <= width {
                    self.dig((x, y), power);
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

/// 地盤の硬さの推定マップ
#[derive(Debug, Clone)]
struct Map {
    n: i128,
    data: BTreeMap<P, i128>,
}
impl Map {
    fn new(n: i128) -> Self {
        let mut data = BTreeMap::new();
        let avg = 700;
        for i in 0..n {
            for j in 0..n {
                data.insert((i, j), avg);
            }
        }
        Self { n, data }
    }
    /// 硬さの推定値
    fn strength(&self, p: P) -> i128 {
        let mut r = vec![];
        let radius = 2;
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let l2 = dx * dx + dy * dy;
                let x = p.0 + dx;
                let y = p.1 + dy;
                if x < 0 || y < 0 || x >= self.n || y >= self.n {
                    continue;
                }
                if l2 <= 4 {
                    if let Some(&s) = self.data.get(&(x, y)) {
                        r.push(s);
                    }
                }
            }
        }
        if r.is_empty() {
            0
        } else {
            r.iter().sum::<i128>() / r.len() as i128
        }
    }
    /// p 周辺は strength であると推定する
    /// coreraidus 以下は strength を代入
    /// radius 以下は strength と現在推定値の平均を取る
    fn set(&mut self, p: P, strength: i128, coreradius: i128, radius: i128) {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let l2 = dx * dx + dy * dy;
                let x = p.0 + dx;
                let y = p.1 + dy;
                if x < 0 || y < 0 || x >= self.n || y >= self.n {
                    continue;
                }
                if l2 < coreradius.pow(2) {
                    self.data.insert((x, y), strength);
                } else if l2 < radius.pow(2) {
                    let s = self.data.get(&(x, y)).unwrap();
                    let s = (s + strength) / 2;
                    self.data.insert((x, y), s);
                }
            }
        }
    }
    /// サンプリングして地盤の硬さを調べる
    fn scan(&mut self, game: &mut Game) {
        let mut rects = vec![];
        for h in game.homes.clone() {
            for w in game.waters.clone() {
                rects.push(Rect(h, w));
            }
        }

        const N: i128 = 20; // サンプル数
        let width = game.n / N;
        let coreradius = width - 3;
        let radius = width;
        let pows = vec![20, 50, 100, 500, 1000];
        for i in 0..=N {
            for j in 0..=N {
                let x = width / 2 + width * i;
                let y = width / 2 + width * j;
                if x >= game.n || y >= game.n {
                    continue;
                }

                if !rects.iter().any(|r| r.contains((x, y), radius)) {
                    continue;
                }

                println!("# Scan {:?}", (x, y));
                let mut accumulate = 0;
                for &power in pows.iter() {
                    accumulate += power;
                    let res = game.dig((x, y), power);
                    if res == Dig::Broken {
                        self.set((x, y), accumulate, coreradius, radius);
                        break;
                    } else if power == pows[pows.len() - 1] {
                        self.set((x, y), 4800, coreradius, radius);
                    }
                }
            }
        }
    }
    /// 推定されたマップの可視化
    fn dump(&self) {
        #[cfg(debug_assertions)]
        for i in 0..40 {
            for j in 0..40 {
                let x = i * 5;
                let y = j * 5;
                let s = self.strength((x, y));
                eprint!(
                    "{}",
                    match s {
                        _ if s < 20 => ' ',
                        _ if s < 100 => '.',
                        _ if s < 400 => '-',
                        _ if s < 700 => '+',
                        _ if s < 1500 => '*',
                        _ => '#',
                    }
                );
            }
            eprintln!();
        }
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
    let mut map = Map::new(n);
    map.scan(&mut game);
    map.dump();

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
        trace!(#loop);

        // 家 vs 最近水源の割当
        let mut nearest: Vec<(P, P, i128)> = {
            game.homes
                .iter()
                .filter(|&home| {
                    // まだ水源が引かれてない家
                    !game.waters.contains(&home)
                })
                .map(|&home| {
                    let (d, w) = game
                        .waters
                        .iter()
                        .map(|&w| {
                            let d = dist::manhattan(home, w);
                            (d, w)
                        })
                        .min()
                        .unwrap();
                    (home, w, d)
                })
                .collect()
        };
        nearest.sort_by_key(|&(_, _, d)| d);

        for &(home, _w, _d) in nearest.iter() {
            trace!(home);
            let mut q = BinaryHeap::new();
            let mut checked = BTreeSet::new();
            for &w in game.waters.iter() {
                let h = dist::manhattan(w, home);
                q.push((Reverse((h, 0)), w));
            }
            let mut from = BTreeMap::new();
            trace!(#Astar);
            while let Some((Reverse((_, cost)), u)) = q.pop() {
                if checked.contains(&u) {
                    continue;
                }
                if u == home {
                    break;
                }
                checked.insert(u);
                for v in game.neigh(u) {
                    if game.waters.contains(&v) {
                        continue;
                    }
                    if checked.contains(&v) {
                        continue;
                    }
                    let appendcost = if game.broken.contains(&v) {
                        0
                    } else {
                        game.c + map.strength(v) - game.damage[v]
                    };
                    from.insert(v, u);
                    let h = dist::manhattan(v, home);
                    q.push((Reverse((h + cost + appendcost, cost + appendcost)), v));
                }
            }
            trace!(#done);
            let mut path = vec![home];
            while let Some(&prev) = from.get(&path[path.len() - 1]) {
                path.push(prev);
                if game.waters.contains(&prev) {
                    break;
                }
            }
            for p in path {
                let estimated_power = map.strength(p) - game.damage[p];
                if estimated_power > 0 {
                    println!(
                        "# estimate_strength={}, damaged={}",
                        map.strength(p),
                        game.damage[p]
                    );
                    game.dig(p, estimated_power + 10);
                }
                game.dig_full(p);
            }
            trace!(#waterflow);
            game.waterflow();
        }
        break;
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

pub mod dist {
    pub fn manhattan(x: (i128, i128), y: (i128, i128)) -> i128 {
        (x.0 - y.0).abs() + (x.1 - y.1).abs()
    }
    pub fn l2(x: (i128, i128), y: (i128, i128)) -> i128 {
        (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2)
    }
    pub fn l2_norm(x: (i128, i128), y: (i128, i128)) -> i128 {
        let d2 = (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2);
        (d2 as f64).sqrt() as i128
    }
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

struct Rect(P, P);
impl Rect {
    fn contains(&self, p: P, padding: i128) -> bool {
        let left = min!(self.0 .0, self.1 .0);
        let right = max!(self.0 .0, self.1 .0);
        if p.0 < left - padding || right + padding < p.0 {
            return false;
        }
        let left = min!(self.0 .1, self.1 .1);
        let right = max!(self.0 .1, self.1 .1);
        if p.1 < left - padding || right + padding < p.1 {
            return false;
        }
        true
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
    (# $x:ident) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}]", stringify!($x));
    };
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
