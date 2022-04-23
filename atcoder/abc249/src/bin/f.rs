#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let mut k: usize = sc.cin();
    let qs: Vec<(usize, i64)> = (0..n)
        .map(|_| {
            let t: usize = sc.cin();
            let y: i64 = sc.cin();
            (t, y)
        })
        .collect();

    // trivial
    if qs.iter().all(|&(ty, _)| ty == 2) {
        let mut ys: Vec<i64> = qs.iter().map(|&(_, y)| y).collect();
        ys.sort();
        let mut x = 0;
        for &y in ys.iter() {
            if y < 0 && k > 0 {
                k -= 1;
            } else {
                x += y;
            }
        }
        put!(x);
        return;
    }

    let mut ans = None;

    let mut ysum = 0;
    let mut yset = BinaryHeap::new(); // 小さい y を最大 k 個もつ

    let mut qs: Vec<(_, _)> = qs.iter().rev().cloned().collect();
    qs.push((1, 0));

    for &(ty, y) in qs.iter() {
        trace!(ty, y);
        if ty == 1 {
            {
                let x = y + ysum;
                trace!(x, (y, ysum));
                ans = max!(ans, Some(x));
            }
            if k == 0 {
                break;
            }
            k -= 1;
            while yset.len() > k {
                let maxy = yset.pop().unwrap();
                ysum += maxy;
            }
        }
        if ty == 2 {
            if y < 0 {
                if yset.len() < k {
                    yset.push(y);
                } else {
                    yset.push(y);
                    let maxy = yset.pop().unwrap();
                    ysum += maxy;
                }
            } else {
                ysum += y;
            }
        }
        trace!(k, y, ysum, &yset);
    }

    put!(ans.unwrap());
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;

struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }
    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
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
