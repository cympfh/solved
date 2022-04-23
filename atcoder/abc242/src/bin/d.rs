#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn ord(c: char) -> usize {
    c as usize - 'A' as usize
}

fn chr(c: usize) -> char {
    ((c + 'A' as usize) as u8) as char
}

fn naiive(t: usize, k: usize, s: &Vec<usize>, offset: usize) -> usize {
    if t == 0 {
        (s[k] + offset) % 3
    } else {
        naiive(
            t - 1,
            k / 2,
            s,
            if k % 2 == 0 { offset + 1 } else { offset + 2 },
        )
    }
}

fn main() {
    let mut sc = Scanner::new();
    let s: Vec<usize> = sc.chars().into_iter().map(|c| ord(c)).collect();
    trace!(&s);
    let q: usize = sc.cin();
    for _ in 0..q {
        let t: usize = sc.cin();
        let k: usize = sc.cin::<usize>() - 1;
        trace!(t, k);
        if t < 1000 {
            let ans = naiive(t, k, &s, 0);
            put!(chr(ans));
        } else {
            let mut t = t;
            let mut k = k;
            let mut ans = s[0];
            while k > 0 {
                if k % 2 == 0 {
                    ans += 1;
                } else {
                    ans += 2;
                }
                ans %= 3;
                k /= 2;
                t -= 1;
            }
            ans = (ans + t) % 3;
            put!(chr(ans));
        }
    }
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
