#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let __n: usize = sc.cin();
    let mut s: Vec<char> = sc.chars();
    s.push('$');

    let mut v = vec![];

    for i in 1..s.len() - 1 {
        if s[i] == 'R' {
            let mut last_a = 0;
            let mut last_c = 0;
            for j in (0..i).rev() {
                if s[j] == 'A' {
                    last_a += 1;
                } else {
                    break;
                }
            }
            for j in i + 1..s.len() {
                if s[j] == 'C' {
                    last_c += 1;
                } else {
                    break;
                }
            }
            trace!(last_a, last_c);
            v.push(min!(last_a, last_c));
        }
    }
    trace!(&v);

    let mut ones = 0;
    let mut more = BinaryHeap::new();
    for &x in v.iter() {
        if x == 1 {
            ones += 1;
        } else if x > 1 {
            more.push(x);
        }
    }

    let mut ans = 0;
    while let Some(u) = more.pop() {
        if u == 2 {
            ans += 2;
        } else if u > 2 {
            if ones > 0 {
                ans += 2;
                ones -= 1;
                more.push(u - 1);
            } else {
                ans += 2;
            }
        }
    }
    ans += ones;
    put!(ans);
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
