#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Project {
    cost: i64,
    value: i64,
}
impl Project {
    fn new(cost: i64, value: i64) -> Self {
        Self { cost, value }
    }
    /// 仕事を w だけ進める, 完了したら value を返す
    fn progress(&mut self, w: i64) -> Option<i64> {
        self.cost -= w;
        if self.cost <= 0 {
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Work(i64),
    WorkAll(i64),
    Cancel,
    Tenkan,
    PowerUp,
}
impl Card {
    fn new(ty: usize, w: i64) -> Self {
        match ty {
            0 => Card::Work(w),
            1 => Card::WorkAll(w),
            2 => Card::Cancel,
            3 => Card::Tenkan,
            _ => Card::PowerUp,
        }
    }
}

/// 相場
#[derive(Debug, Clone)]
struct Souba {
    data: VecDeque<i64>,
    size: usize,
}
impl Souba {
    fn new(size: usize) -> Self {
        Self {
            data: VecDeque::new(),
            size,
        }
    }
    fn add(&mut self, x: i64) {
        self.data.push_back(x);
        while self.data.len() > self.size {
            self.data.pop_front();
        }
    }
    fn value(&self) -> i64 {
        let mut sum = 0;
        if self.data.is_empty() {
            return 0;
        }
        for i in 0..self.data.len() {
            sum += self.data[i];
        }
        sum / self.data.len() as i64
    }
}

struct Game {
    hand: Vec<Card>,
    num_hand: usize, // N
    projects: Vec<Project>,
    num_project: usize, // M
    times: usize,
    fulltime: usize,
    num_draw: usize, // K
    money: i64,
    ell: usize,   // L, 増資カードの使用回数
    souba: Souba, // 提示されるカードの相場
    rand: XorShift,
}
impl Game {
    fn new(hand: Vec<Card>, projects: Vec<Project>, fulltime: usize, num_draw: usize) -> Self {
        Self {
            num_hand: hand.len(),
            hand,
            num_project: projects.len(),
            projects,
            times: 0,
            fulltime,
            num_draw,
            money: 0,
            ell: 0,
            souba: Souba::new(30),
            rand: XorShift::new(),
        }
    }
    fn exit(&self) -> bool {
        self.times >= self.fulltime
    }
    /// solver
    fn play(&mut self) {
        trace!(#play self.times);
        trace!(self.money);
        trace!(&self.projects);
        trace!(&self.hand);
        trace!(self.souba.value());

        let used_index;
        // 使用カードの選択
        {
            let mut cands = BinaryHeap::new();
            let i = self.rand.gen::<usize>() % self.hand.len();
            cands.push((0, i, 0)); // priority, card-index, project-index
                                   // for i in 0..self.hand.len() {
                                   //     let c = &self.hand[i];
                                   //     match c {
                                   //         &Card::PowerUp if self.times < 500 && self.ell < 1 => {
                                   //             cands.push((1000, i, 0));
                                   //         }
                                   //         &Card::WorkAll(w) => {
                                   //             cands.push((300 - w, i, 0));
                                   //         }
                                   //         &Card::Work(w) => {
                                   //             let mut maxvalue = 0;
                                   //             let mut maxarg = (0..self.projects.len())
                                   //                 .min_by_key(|&i| self.projects[i].cost)
                                   //                 .unwrap();
                                   //             for i in 0..self.projects.len() {
                                   //                 let p = &self.projects[i];
                                   //                 if p.cost <= w && p.value > maxvalue {
                                   //                     maxvalue = p.value;
                                   //                     maxarg = i;
                                   //                 }
                                   //             }
                                   //             cands.push((10, i, maxarg));
                                   //         }
                                   //         _ => {}
                                   //     }
                                   // }
            let (_, c, p) = cands.pop().unwrap();
            trace!(#use &c, &self.hand[c], p);
            used_index = c;
            println!("{} {}", c, p);
            flush();
        }

        // 使用の副作用
        match self.hand[used_index] {
            Card::PowerUp => {
                self.ell += 1;
            }
            _ => {}
        }

        // プロジェクト状態とお金の更新
        self.refresh();

        // 貰うカードの選択
        {
            let given_cards = self.get_next_hands();
            trace!(&given_cards);
            let mut cands = BinaryHeap::new();
            cands.push((0, 0)); // priority, card-index
                                // for i in 0..given_cards.len() {
                                //     if given_cards[i].1 > self.money {
                                //         continue;
                                //     }
                                //     match given_cards[i].0 {
                                //         Card::PowerUp if self.times < 500 && self.ell < 1 => {
                                //             cands.push((100000, i));
                                //         }
                                //         Card::WorkAll(w) if self.times <= 940 => {
                                //             cands.push((w + 100, i));
                                //         }
                                //         Card::Work(w) if self.times <= 900 => {
                                //             cands.push((w + 10, i));
                                //         }
                                //         _ => {}
                                //     }
                                // }
            let (_, c) = cands.pop().unwrap();
            trace!(#get c, &given_cards[c]);
            println!("{}", c);
            flush();
            self.money -= given_cards[c].1;
            self.hand[used_index] = given_cards[c].0.clone();

            for i in 1..given_cards.len() {
                self.souba.add(given_cards[i].1);
            }
        }

        self.times += 1;
        trace!(#end);
    }
    fn refresh(&mut self) {
        let mut sc = Scanner::default();
        self.projects = (0..self.num_project)
            .map(|_| {
                let h: i64 = sc.cin();
                let v: i64 = sc.cin();
                Project::new(h, v)
            })
            .collect();
        self.money = sc.cin();
    }
    fn get_next_hands(&self) -> Vec<(Card, i64)> {
        let mut sc = Scanner::default();
        (0..self.num_draw)
            .map(|_| {
                let ty: usize = sc.cin();
                let w: i64 = sc.cin();
                let p: i64 = sc.cin();
                (Card::new(ty, w), p)
            })
            .collect()
    }
}

fn main() {
    let mut game = {
        let mut sc = Scanner::default();
        let n: usize = sc.cin();
        let m: usize = sc.cin();
        let k: usize = sc.cin();
        let t: usize = sc.cin();
        let hand: Vec<Card> = (0..n)
            .map(|_| {
                let ty: usize = sc.cin();
                let w: i64 = sc.cin();
                Card::new(ty, w)
            })
            .collect();
        let projects: Vec<Project> = (0..m)
            .map(|_| {
                let h: i64 = sc.cin();
                let v: i64 = sc.cin();
                Project::new(h, v)
            })
            .collect();
        Game::new(hand, projects, t, k)
    };
    while !game.exit() {
        game.play();
        flush();
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
    (# $label:ident ) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}]", stringify!($label));
    };
    (# $label:ident $x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!("[{}] {} = {:?}", stringify!($label), stringify!($x), $x)
    };
    (# $label:ident $($xs:expr),*) => { trace!(# $label ($($xs),*)) };
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
#[derive(Debug)]
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

// }}}
