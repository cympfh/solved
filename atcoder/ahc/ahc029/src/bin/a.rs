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
            1 => Card::Work(w),
            2 => Card::WorkAll(w),
            3 => Card::Cancel,
            4 => Card::Tenkan,
            _ => Card::PowerUp,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Game {
    hand: Vec<Card>,
    num_hand: usize, // N
    projects: Vec<Project>,
    num_project: usize, // M
    times: usize,
    fulltime: usize,
    num_draw: usize, // K
    money: i64,
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
        }
    }
    fn exit(&self) -> bool {
        self.times >= self.fulltime
    }
    /// solver
    fn play(&mut self) {
        let card = self.hand.remove(0);
        println!("0 0");
        flush();
        match card {
            Card::Work(x) => {
                if let Some(value) = self.projects[0].progress(x) {
                    self.money += value;
                    self.projects.remove(0);
                }
            }
            Card::WorkAll(x) => {
                let m = self.projects.len();
                let mut done = vec![];
                for i in 0..m {
                    if let Some(value) = self.projects[i].progress(x) {
                        self.money += value;
                    } else {
                        done.push(i);
                    }
                }
                done.sort();
                done.reverse();
                for i in done {
                    self.projects.remove(i);
                }
            }
            _ => {}
        }
        self.refresh();
        let cands = self.get_next_hands();
        println!("0");
        flush();
        self.hand.insert(0, cands[0].clone());
        self.times += 1;
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
    fn get_next_hands(&self) -> Vec<Card> {
        let mut sc = Scanner::default();
        (0..self.num_draw)
            .map(|_| {
                let ty: usize = sc.cin();
                let w: i64 = sc.cin();
                Card::new(ty, w)
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
        trace!(game.times);
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
