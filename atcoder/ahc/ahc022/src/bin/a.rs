#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

const DEBUG: usize = 5;

fn padd(p: (usize, usize), dx: (i32, i32), modulo: usize) -> (usize, usize) {
    let i = (p.0 as i32 + dx.0).rem_euclid(modulo as i32) as usize;
    let j = (p.1 as i32 + dx.1).rem_euclid(modulo as i32) as usize;
    (i, j)
}

struct HeatMap {
    len: usize,
    data: Vec<Vec<i32>>,
    ps: Vec<(usize, usize)>,
}
impl HeatMap {
    fn new(len: usize, defaultvalue: i32, ps: Vec<(usize, usize)>) -> Self {
        let data = ndarray![defaultvalue; len, len];
        Self { len, data, ps }
    }
    fn set(&mut self, i: usize, j: usize, value: i32) {
        self.data[i][j] = value;
    }
    fn get(&self, i: usize, j: usize, dy: i32, dx: i32) -> i32 {
        let (y, x) = padd((i, j), (dy, dx), self.len);
        self.data[y][x]
    }
    fn dump(&self) {
        for i in 0..self.len {
            put!(..self.data[i]);
        }
    }
    fn mindist_from_any_hole(&self, i: usize, j: usize) -> i32 {
        let mut mindist = 999_999_999;
        fn diff(x: usize, y: usize) -> i32 {
            if x > y {
                (x - y) as i32
            } else {
                (y - x) as i32
            }
        }
        for &(y, x) in self.ps.iter() {
            let d = diff(i, y) + diff(j, x);
            if d < mindist {
                mindist = d
            }
        }
        mindist
    }
}

struct Measures {
    n: usize,
    map: HeatMap,
    data: Vec<Vec<(i32, i32, i32)>>,
}
impl Measures {
    fn new(n: usize, map: HeatMap) -> Self {
        Self {
            n,
            map,
            data: vec![vec![]; n],
        }
    }
    fn run(&mut self, id: usize, dy: i32, dx: i32) {
        put!(id, dy, dx);
        flush();
        let mut sc = Scanner::default();
        let m: i32 = sc.cin();
        self.data[id].push((dy, dx, m));
    }
    fn mostclose(&self, id: usize) -> (usize, (usize, usize), i32) {
        let mut ret = (999, (999, 999), 999_999_999);
        for (k, &(i, j)) in self.map.ps.iter().enumerate() {
            let mut err = 0;
            for &(dy, dx, m) in self.data[id].iter() {
                let truem = self.map.get(i, j, dy, dx);
                err += (truem - m).pow(2);
            }
            err /= self.data[id].len() as i32;
            if ret.2 > err {
                ret = (k, (i, j), err);
            }
        }
        ret
    }
}

fn main() {
    let mut sc = Scanner::default();
    let len: usize = sc.cin();
    let n: usize = sc.cin();
    let _variance: usize = sc.cin();
    let ps: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let y: usize = sc.cin();
            let x: usize = sc.cin();
            (y, x)
        })
        .collect();

    // assignment
    let mut map = HeatMap::new(len, 500, ps);
    {
        // 3-ary
        {
            let temps = [500, 700, 300, 900, 100];
            let dxes = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];
            for (id, cs) in PowerPermutation::new(3, 5).enumerate() {
                if id >= n {
                    break;
                }
                for i in 0..5 {
                    let m = temps[cs[i]];
                    let dx = dxes[i];
                    let (i, j) = padd(map.ps[id], dx, len);
                    map.set(i, j, m);
                }
            }
        }

        // random
        // {
        //     let mut rand = XorShift::new();
        //     for _ in 0..n {
        //         rand.gen::<usize>();
        //     }
        //     for i in 0..len {
        //         for j in 0..len {
        //             let m = if map.mindist_from_any_hole(i, j) <= 1 {
        //                 rand.gen::<i32>().rem_euclid(1000)
        //             } else {
        //                 500
        //             };
        //             map.set(i, j, m);
        //         }
        //     }
        // }

        map.dump();
    }
    flush();

    // measure
    let mut ms = Measures::new(n, map);
    {
        for id in 0..n {
            ms.run(id, 0, 0);
            ms.run(id, 1, 0);
            ms.run(id, -1, 0);
            ms.run(id, 0, 1);
            ms.run(id, 0, -1);
        }
    }

    // report
    put!(-1, -1, -1);
    for id in 0..n {
        if id < DEBUG {
            trace!(ms.mostclose(id), &ms.data[id]);
        }
        let (k, _, _) = ms.mostclose(id);
        put!(k);
    }
}

// @num/iter/power
/// Number - Iterator - Power Permutation (pow(n, m))
pub struct PowerPermutation {
    n: usize,
    m: usize,
    ar: Vec<usize>,
    done: bool,
}
impl PowerPermutation {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            ar: vec![0; m],
            done: false,
        }
    }
}
impl Iterator for PowerPermutation {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.done {
            return None;
        }
        if self.m == 0 {
            self.done = true;
            return Some(self.ar.clone());
        }
        if self.ar[self.m - 1] >= self.n {
            return None;
        }
        let r = self.ar.clone();
        self.ar[0] += 1;
        for i in 0..self.m - 1 {
            if self.ar[i] == self.n {
                self.ar[i] = 0;
                self.ar[i + 1] += 1;
            } else {
                break;
            }
        }
        Some(r)
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
