#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
    U,
    D,
    Nop,
}
impl Direction {
    fn tochar(&self) -> char {
        use Direction::*;
        match self {
            L => 'L',
            R => 'R',
            U => 'U',
            D => 'D',
            Nop => '.',
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    mov: Direction,
    rot: Vec<Direction>,
    tako: Vec<bool>,
}

impl Operation {
    fn nop(v: usize) -> Self {
        use Direction::*;
        Self {
            mov: Nop,
            rot: vec![Nop; v],
            tako: vec![false; v + 1],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    /// ボードサイズ
    n: i64,
    /// たこ焼きがまだ置いてある座標
    balls: BTreeSet<(i64, i64)>,
    /// たこ焼きを置くべき座標, まだ置かれていない
    requires: BTreeSet<(i64, i64)>,
    /// アーム
    arm: CrossArm,
    /// 初期状態のコピー
    initial_arm: CrossArm,
    /// 操作列
    operations: Vec<Operation>,
    /// for DEBUG
    abort: bool,
    time: usize,
}

impl Game {
    fn new(n: i64, balls: BTreeSet<(i64, i64)>, requires: BTreeSet<(i64, i64)>) -> Self {
        let (arm, initial_commands) = CrossArm::new();
        let initial_arm = arm.clone();
        let initial_time = initial_commands.len();
        Self {
            n,
            balls,
            requires,
            arm,
            initial_arm,
            operations: initial_commands,
            abort: false,
            time: initial_time,
        }
    }
    fn end(&self) -> bool {
        self.abort || self.requires.is_empty()
    }
    fn dump(&self) {
        println!("{}", self.initial_arm.v + 1);
        for &(x, y) in self.initial_arm.tree.iter() {
            println!("{} {}", x, y);
        }
        println!(
            "{} {}",
            self.initial_arm.center.0, self.initial_arm.center.1
        );
        for op in self.operations.iter() {
            print!("{}", op.mov.tochar());
            for d in op.rot.iter() {
                print!("{}", d.tochar());
            }
            for &doit in op.tako.iter() {
                print!("{}", if doit { 'P' } else { '.' });
            }
            println!();
        }
    }
    /// 一つ実行する
    fn execute(&mut self, op: Operation) {
        use Direction::*;
        self.operations.push(op.clone());
        self.time += 1;
        self.arm += op.mov; // 平行移動
        self.arm *= op.rot[0]; // 回転, TODO(全体の回転しか想定してない)
        {
            // get/set
            for i in 0..self.arm.v {
                if op.tako[i + 1] {
                    let pos = self.arm.leave_pos(i);
                    if self.arm.has[i] {
                        // put
                        self.arm.has[i] = false;
                        self.arm.num_tako -= 1;
                        self.requires.remove(&pos);
                    } else {
                        self.arm.has[i] = true;
                        self.arm.num_tako += 1;
                        self.balls.remove(&pos);
                    }
                }
            }
        }
    }
    /// 実行計画を決めて実行する
    fn run(&mut self) {
        use Direction::*;
        let mut cands = vec![];
        for d in [U, D, L, R, Nop] {
            for rot in [L, R, Nop] {
                cands.push(self.goodness(d, rot));
            }
        }
        cands.sort_by_key(&|&(score, _): &(i64, Operation)| -score); // DESC BY score
        #[cfg(debug_assertions)]
        {
            // DEBUG
            if self.time > 2000 {
                trace!(self.time);
                trace!(&self.arm);
                for c in cands.iter() {
                    trace!(c);
                }
            }
            if self.time > 2000 {
                self.abort = true;
            }
        }
        let (_best_score, best_op) = cands[0].clone();
        self.execute(best_op);
    }

    /// (+d*rot) するとしての良さとそのときの Operation
    fn goodness(&self, d: Direction, rot: Direction) -> (i64, Operation) {
        use Direction::*;
        let mut arm = self.arm.clone();
        arm += d;
        arm *= rot;
        let mut score = 0;
        if arm.center.0 < -1
            || arm.center.0 >= self.n
            || arm.center.1 < -1
            || arm.center.1 >= self.n
        {
            // out of range?
            return (-1_000_000_000, Operation::nop(arm.v));
        }
        // フチ
        if arm.center.0 <= 0
            || arm.center.1 <= 0
            || arm.center.0 >= self.n - -1
            || arm.center.1 >= self.n - -1
        {
            score -= 10;
        }
        // 何もしないのは良くない
        {
            if d == Nop {
                score -= 2;
            }
            if rot == Nop {
                score -= 1;
            }
        }
        // 即座に get/put できるかチェック
        let mut tako = vec![false; arm.v + 1];
        {
            for i in 0..arm.v {
                let pos = arm.leave_pos(i);
                if !arm.has[i] && self.balls.contains(&pos) {
                    score += 100;
                    tako[i + 1] = true;
                    arm.num_tako += 1;
                    arm.has[i] = true;
                } else if arm.has[i] && self.requires.contains(&pos) {
                    score += 100;
                    tako[i + 1] = true;
                    arm.num_tako -= 1;
                    arm.has[i] = false;
                }
            }
        }
        if arm.num_tako < arm.v && !self.balls.is_empty() {
            // 新しいたこ焼きを拾いに行く
            let mut min_dist = 1_000_000_000;
            for &(x, y) in self.balls.iter() {
                for i in 0..self.arm.v {
                    if arm.has[i] {
                        continue;
                    }
                    let pos = arm.leave_pos(i);
                    min_dist = min!(min_dist, (pos.0 - x).abs() + (pos.1 - y).abs());
                }
            }
            if min_dist < 1_000_000_000 {
                score -= min_dist;
            }
        } else {
            // 置く
            let mut min_dist = 1_000_000_000;
            for &(x, y) in self.requires.iter() {
                for i in 0..self.arm.v {
                    if !arm.has[i] {
                        continue;
                    }
                    let pos = arm.leave_pos(i);
                    min_dist = min!(min_dist, (pos.0 - x).abs() + (pos.1 - y).abs());
                }
            }
            if min_dist < 1_000_000_000 {
                score -= min_dist;
            }
        }
        (
            score,
            Operation {
                mov: d,
                rot: vec![rot; arm.v],
                tako,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct CrossArm {
    center: (i64, i64),
    v: usize,                  // 葉っぱの数 (center は除く)
    tree: Vec<(usize, usize)>, // tree[i] = 葉っぱ i の (parent, length)
    leaves: Vec<(i64, i64)>,   // leaves[i] = 葉っぱ i の center から見た相対座標
    has: Vec<bool>,            // has[i] = 葉っぱ i がたこ焼きを確保してるか
    num_tako: usize,           // 確保してるたこ焼きの総数
}

impl CrossArm {
    //      4
    //      |
    // 3 -- 0 -- 1
    //      |
    //      2
    fn new() -> (Self, Vec<Operation>) {
        use Direction::*;
        let tree = vec![(0, 1), (0, 1), (0, 1), (0, 1)];
        let leaves = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let has = vec![false, false, false, false];
        let arm = Self {
            center: (1, 1),
            v: 4,
            tree,
            leaves,
            has,
            num_tako: 0,
        };
        let command = vec![
            Operation {
                mov: Nop,
                rot: vec![Nop, R, R, L],
                tako: vec![false, false, false, false, false],
            },
            Operation {
                mov: Nop,
                rot: vec![Nop, Nop, R, Nop],
                tako: vec![false, false, false, false, false],
            },
        ];
        (arm, command)
    }
    fn is_full(&self) -> bool {
        self.num_tako == self.v
    }
    /// 葉っぱ i の座標
    fn leave_pos(&self, i: usize) -> (i64, i64) {
        let x = self.center.0 + self.leaves[i].0;
        let y = self.center.1 + self.leaves[i].1;
        (x, y)
    }
}

/// 全体の平行移動
impl std::ops::AddAssign<Direction> for CrossArm {
    fn add_assign(&mut self, d: Direction) {
        use Direction::*;
        let (dx, dy) = match d {
            U => (-1, 0),
            D => (1, 0),
            R => (0, 1),
            L => (0, -1),
            _ => (0, 0),
        };
        self.center = (self.center.0 + dx, self.center.1 + dy);
    }
}
impl std::ops::Add<Direction> for &CrossArm {
    type Output = CrossArm;
    fn add(self, d: Direction) -> Self::Output {
        let mut r = self.clone();
        r += d;
        r
    }
}

/// 全体の回転
impl std::ops::MulAssign<Direction> for CrossArm {
    fn mul_assign(&mut self, d: Direction) {
        use Direction::*;
        match d {
            L => {
                self.leaves.rotate_right(1);
            }
            R => {
                self.leaves.rotate_left(1);
            }
            _ => {}
        }
    }
}
impl std::ops::Mul<Direction> for &CrossArm {
    type Output = CrossArm;
    fn mul(self, d: Direction) -> Self::Output {
        let mut r = self.clone();
        r *= d;
        r
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let _m: usize = sc.cin();
    let _v: usize = sc.cin();

    let mut balls = BTreeSet::new();
    for i in 0..n {
        let s = sc.chars();
        for j in 0..n {
            if s[j] == '1' {
                balls.insert((i as i64, j as i64));
            }
        }
    }
    let mut requires = BTreeSet::new();
    for i in 0..n {
        let s = sc.chars();
        for j in 0..n {
            if s[j] == '1' {
                requires.insert((i as i64, j as i64));
            }
        }
    }

    // 相殺
    {
        let mut rm = vec![];
        for &(x, y) in balls.iter() {
            if requires.contains(&(x, y)) {
                rm.push((x, y));
            }
        }
        for (x, y) in rm {
            balls.remove(&(x, y));
            requires.remove(&(x, y));
        }
    }

    let mut game = Game::new(n as i64, balls, requires);
    while !game.end() {
        game.run();
    }
    trace!(&game.balls, &game.requires);
    trace!(&game.arm);
    game.dump();
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
