#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct P(i64, i64);

impl P {
    fn rev_x(&mut self) {
        self.0 *= -1;
    }
    fn rev_y(&mut self) {
        self.1 *= -1;
    }
    fn hiku(&mut self, q: P) {
        self.0 -= q.0;
        self.1 -= q.1;
    }
}

// z が [x,y] にあるかどうか
fn between(z: i64, x: i64, y: i64) -> bool {
    if x <= z && z <= y {
        true
    } else if y <= z && z <= x {
        true
    } else {
        false
    }
}

// 荷物 (r) を避けて移動 p->q する
fn idou(p: P, q: P, r: Option<P>) -> i64 {
    if p == q {
        return 0;
    }
    let man = (p.0 - q.0).abs() + (p.1 - q.1).abs();
    if let Some(r) = r {
        if p.0 == q.0 && q.0 == r.0 && between(r.1, p.1, q.1) {
            man + 2
        } else if p.1 == q.1 && q.1 == r.1 && between(r.0, p.0, q.0) {
            man + 2
        } else {
            man
        }
    } else {
        man
    }
}

fn solver_migi_ue(a: P, b: P, c: P) -> i64 {
    let mut a = a;
    let mut b = b;
    let mut sum = 0;
    let hidari = P(b.0 - 1, b.1);
    sum += idou(a, hidari, Some(b));
    a = hidari;
    let shita = P(c.0, b.1);
    let shitahidari = P(c.0 - 1, b.1);
    sum += idou(a, shitahidari, None);
    a = shitahidari;
    b = shita;
    let shita = P(b.0, b.1 - 1);
    sum += idou(a, shita, Some(b));
    a = shita;
    let chokka = P(c.0, c.1 - 1);
    sum += idou(a, chokka, None);
    sum
}

fn solver_ue_migi(a: P, b: P, c: P) -> i64 {
    let a = P(a.1, a.0);
    let b = P(b.1, b.0);
    let c = P(c.1, c.0);
    solver_migi_ue(a, b, c)
}

fn solver_icchoku(a: P, b: P, c: P) -> i64 {
    if b.1 == c.1 {
        let mut sum = 0;
        sum += idou(a, P(b.0 - 1, b.1), Some(b));
        sum += idou(b, c, None);
        sum
    } else if b.0 == c.0 {
        let mut sum = 0;
        sum += idou(a, P(b.0, b.1 - 1), Some(b));
        sum += idou(b, c, None);
        sum
    } else {
        i64::MAX
    }
}

fn main() {
    let mut sc = Scanner::default();
    let mut a = P(sc.cin(), sc.cin());
    let mut b = P(sc.cin(), sc.cin());
    let mut c = P(sc.cin(), sc.cin());
    if c.0 < b.0 {
        a.rev_x();
        b.rev_x();
        c.rev_x();
    }
    if c.1 < b.1 {
        a.rev_y();
        b.rev_y();
        c.rev_y();
    }
    a.hiku(b);
    c.hiku(b);
    b = P(0, 0);
    trace!(a, b, c);

    let ans = min!(
        solver_migi_ue(a, b, c),
        solver_ue_migi(a, b, c),
        solver_icchoku(a, b, c)
    );
    put!(ans);
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
