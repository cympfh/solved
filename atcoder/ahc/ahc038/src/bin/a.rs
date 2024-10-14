#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Direction {
    R,
    D,
    L,
    U,
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

/// 実行計画の気持ち
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Get,
    Put,
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
    arm: Arm,
    /// 初期状態のコピー
    initial_arm: Arm,
    /// 操作列
    operations: Vec<Operation>,
    /// 実行計画
    mode: Mode,
    /// for DEBUG
    abort: bool,
    time: usize,
}

impl Game {
    fn new(
        n: usize,
        balls: BTreeSet<(i64, i64)>,
        requires: BTreeSet<(i64, i64)>,
        arm: Arm,
    ) -> Self {
        let initial_commands = arm.initial_commands.clone();
        let initial_arm = arm.clone();
        let initial_time = initial_commands.len();
        Self {
            n: n as i64,
            balls,
            requires,
            arm,
            initial_arm,
            operations: initial_commands,
            mode: Mode::Get,
            abort: false,
            time: initial_time,
        }
    }
    fn end(&self) -> bool {
        self.abort || self.requires.is_empty()
    }
    fn succsess(&self) -> bool {
        self.balls.is_empty() && self.requires.is_empty()
    }
    fn score(&self) -> usize {
        self.operations.len()
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
        {
            // 回転
            self.arm *= op.rot[0]; // 全体の回転
            for child in self.arm.children.iter_mut() {
                *child *= op.rot[2];
            }
        }
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
        // 無限ループで失敗だと判断
        if self.time > 2333 {
            self.abort = true;
        }
        use Direction::*;
        let mut cands = vec![];
        for d in [U, D, L, R, Nop] {
            for rot in [L, R, Nop] {
                if self.arm.children.is_empty() {
                    if let Some(c) = self.goodness(d, rot, rot) {
                        cands.push(c);
                    }
                } else {
                    for rot2 in [L, R, Nop] {
                        if let Some(c) = self.goodness(d, rot, rot2) {
                            cands.push(c);
                        }
                    }
                }
            }
        }
        cands.sort_by_key(|(score, op, _): &(i64, Operation, _)| (-score, op.mov));
        #[cfg(debug_assertions)]
        {
            // DEBUG
            const BREAKPOINT: usize = 2000;
            if self.time + 5 > BREAKPOINT {
                eprintln!("\x1b[41mtime\x1b[0m {}", self.time);
                trace!(self.mode);
                // trace!(&self.arm);
                // for c in cands.iter() {
                //     trace!(c);
                // }
            }
            if self.time >= BREAKPOINT {
                self.abort = true;
            }
        }
        let (_best_score, best_op, best_mode) = cands[0].clone();
        self.execute(best_op);
        self.mode = best_mode;
    }

    /// (+d*rot) するとしての良さとそのときの Operation
    /// 違法手の場合は None
    fn goodness(
        &self,
        d: Direction,
        rot: Direction,
        rot2: Direction,
    ) -> Option<(i64, Operation, Mode)> {
        use Direction::*;
        let mut arm = self.arm.clone();
        arm += d;
        arm *= rot;
        for child in arm.children.iter_mut() {
            *child *= rot2;
        }
        let mut score = 0;
        if arm.center.0 < 0 || arm.center.0 >= self.n || arm.center.1 < 0 || arm.center.1 >= self.n
        {
            // out of range
            return None;
        }
        if arm.center.0 < 1
            || arm.center.0 >= self.n - 1
            || arm.center.1 < 1
            || arm.center.1 >= self.n - 1
        {
            score -= 20;
        }
        let mut tako = vec![false; arm.v + 1];
        let mut rm_balls = BTreeSet::new();
        let mut rm_requires = BTreeSet::new();
        {
            for i in 0..arm.v {
                if !arm.is_leave(i) {
                    continue;
                }
                let pos = arm.leave_pos(i);
                if !arm.has[i] && self.balls.contains(&pos) && !rm_balls.contains(&pos) {
                    score += 100;
                    tako[i + 1] = true;
                    arm.num_tako += 1;
                    arm.has[i] = true;
                    rm_balls.insert(pos);
                } else if arm.has[i] && self.requires.contains(&pos) && !rm_requires.contains(&pos)
                {
                    score += 100;
                    tako[i + 1] = true;
                    arm.num_tako -= 1;
                    arm.has[i] = false;
                    rm_requires.insert(pos);
                }
            }
        }
        // 次の実行計画
        let mode = if self.mode == Mode::Get && (arm.is_full() || self.balls.is_empty()) {
            Mode::Put
        } else if self.mode == Mode::Put && arm.is_empty() {
            Mode::Get
        } else {
            self.mode
        };
        if mode == Mode::Get {
            // 新しいたこ焼きを拾いに行く
            let mut min_dist = 1_000_000_000;
            for &(x, y) in self.balls.iter() {
                if rm_balls.contains(&(x, y)) {
                    continue;
                }
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
                if rm_requires.contains(&(x, y)) {
                    continue;
                }
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
        let mut r = vec![rot; arm.v];
        for k in 2..arm.v {
            r[k] = rot2;
        }
        Some((
            score,
            Operation {
                mov: d,
                rot: r,
                tako,
            },
            mode,
        ))
    }
}

/// 90 度回転
fn rot90(x: (i64, i64)) -> (i64, i64) {
    (x.1, -x.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArmType {
    Cross,
    I,
    L,
    T,
    OneHand,
    LCross,
    LL,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArmConfig {
    atype: ArmType,
    v: usize,
    scale: usize,
    center: (i64, i64),
}
impl ArmConfig {
    pub fn new(atype: ArmType, v: usize, scale: usize, center: (i64, i64)) -> Self {
        Self {
            atype,
            v,
            scale,
            center,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arm {
    name: String,
    center: (i64, i64),
    v: usize,                  // 葉っぱの数 (center は除く)
    tree: Vec<(usize, usize)>, // tree[i] = 葉っぱ i の (parent, length)
    leaves: Vec<(i64, i64)>,   // leaves[i] = 葉っぱ i の center から見た相対座標
    initial_commands: Vec<Operation>,
    has: Vec<bool>,     // has[i] = 葉っぱ i がたこ焼きを確保してるか
    num_tako: usize,    // 確保してるたこ焼きの総数
    children: Vec<Arm>, // 子ども
    parent: Option<usize>, // 自身が他のアームの子どもである場合, 親を持つ
                        // 親は親アームの中での ID
}

impl Arm {
    fn from(conf: &ArmConfig) -> Self {
        let label = match conf.atype {
            ArmType::Cross => "+",
            ArmType::T => "T",
            ArmType::I => "I",
            ArmType::L => "L",
            ArmType::OneHand => "1",
            ArmType::LCross => "L+",
            ArmType::LL => "LL",
        };
        let name = format!("{}({}; {:?})", label, conf.scale, conf.center);
        match conf.atype {
            ArmType::Cross => Self::crossarm(name, conf.v, conf.scale, conf.center),
            ArmType::T => Self::tarm(name, conf.v, conf.scale, conf.center),
            ArmType::I => Self::iarm(name, conf.v, conf.scale, conf.center),
            ArmType::L => Self::larm(name, conf.v, conf.scale, conf.center),
            ArmType::OneHand => Self::onehand_arm(name, conf.v, conf.scale, conf.center),
            ArmType::LCross => Self::lcross(name, conf.v, conf.scale, conf.center),
            ArmType::LL => Self::ll(name, conf.v, conf.scale, conf.center),
        }
    }
    /// 十字型のアームを構築する
    ///
    ///      4
    ///      |
    /// 3 -- 0 -- 1
    ///      |
    ///      2
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn crossarm(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![];
        let mut leaves = vec![];
        for i in 0..v {
            let group_id = i / 4;
            let len = group_id + 1 + scale;
            let mut pos = (0_i64, len as i64);
            for _ in 0..i % 4 {
                pos = rot90(pos);
            }
            tree.push((0, len));
            leaves.push(pos);
        }
        let has = vec![false; v];
        let mut initial_commands = vec![];
        {
            let mut rot = vec![Nop; v];
            let tako = vec![false; v + 1];
            for i in 0..v {
                rot[i] = match i % 4 {
                    1 | 2 => R,
                    3 => L,
                    _ => Nop,
                };
            }
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
            for i in 0..v {
                rot[i] = match i % 4 {
                    2 => R,
                    _ => Nop,
                };
            }
            initial_commands.push(Operation {
                mov: Nop,
                rot,
                tako,
            });
        };
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![],
            parent: None,
        }
    }
    /// I字型のアームを構築する
    ///
    /// 2 -- 0 -- 1
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn iarm(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![];
        let mut leaves = vec![];
        for i in 0..v {
            let group_id = i / 2;
            let len = group_id + 1 + scale;
            let mut pos = (0_i64, len as i64);
            if i % 2 == 1 {
                pos = rot90(rot90(pos));
            }
            tree.push((0, len));
            leaves.push(pos);
        }
        let has = vec![false; v];
        let mut initial_commands = vec![];
        {
            let mut rot = vec![Nop; v];
            let tako = vec![false; v + 1];
            for i in 0..v {
                rot[i] = if i % 2 == 0 { Nop } else { R };
            }
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
            initial_commands.push(Operation {
                mov: Nop,
                rot,
                tako,
            });
        };
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![],
            parent: None,
        }
    }
    /// L字型のアームを構築する
    ///
    ///      0 -- 1
    ///      |
    ///      2
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn larm(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![];
        let mut leaves = vec![];
        for i in 0..v {
            let group_id = i / 2;
            let len = group_id + 1 + scale;
            let mut pos = (0_i64, len as i64);
            if i % 2 == 1 {
                pos = rot90(pos);
            }
            tree.push((0, len));
            leaves.push(pos);
        }
        let has = vec![false; v];
        let mut initial_commands = vec![];
        {
            let mut rot = vec![Nop; v];
            let tako = vec![false; v + 1];
            for i in 0..v {
                rot[i] = if i % 2 == 0 { Nop } else { R };
            }
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
        };
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![],
            parent: None,
        }
    }
    /// T字型のアームを構築する
    ///
    /// 3 -- 0 -- 1
    ///      |
    ///      2
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn tarm(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![];
        let mut leaves = vec![];
        for i in 0..v {
            let group_id = i / 3;
            let len = group_id + 1 + scale;
            let mut pos = (0_i64, len as i64);
            for _ in 0..i % 3 {
                pos = rot90(pos);
            }
            tree.push((0, len));
            leaves.push(pos);
        }
        let has = vec![false; v];
        let mut initial_commands = vec![];
        {
            let mut rot = vec![Nop; v];
            let tako = vec![false; v + 1];
            for i in 0..v {
                rot[i] = if i % 3 == 0 { Nop } else { R };
            }
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
            for i in 0..v {
                rot[i] = if i % 3 != 2 { Nop } else { R };
            }
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
        };
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![],
            parent: None,
        }
    }
    /// 片手字型のアームを構築する
    ///
    ///      0 -- 1
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn onehand_arm(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![];
        let mut leaves = vec![];
        for i in 0..v {
            let group_id = i / 2;
            let len = group_id + 1 + scale;
            let pos = (0_i64, len as i64);
            tree.push((0, len));
            leaves.push(pos);
        }
        let has = vec![false; v];
        let initial_commands = vec![];
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![],
            parent: None,
        }
    }
    /// 全体は L でその先に Cross を乗せる
    ///
    ///      0 --+
    ///      |
    ///      +
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn lcross(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![(0, scale), (0, scale)];
        let leaves = vec![(0, scale as i64), rot90((0, scale as i64))];
        let m = (v - 2) / 2;
        let mut cross1 = Self::crossarm(format!("{}-c1", name), m, 0, leaves[0]);
        let mut cross2 = Self::crossarm(format!("{}-c2", name), v - 2 - m, 0, leaves[1]);
        cross1.parent = Some(0);
        cross2.parent = Some(1);
        cross2 *= R;
        for k in 0..cross1.v {
            tree.push((1, cross1.tree[k].1));
        }
        for k in 0..cross2.v {
            tree.push((2, cross2.tree[k].1));
        }
        let mut initial_commands = vec![];
        {
            let mut rot = vec![Nop; v];
            let tako = vec![false; v + 1];
            rot[1] = R;
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
            rot[1] = Nop;
            for time in 0..2 {
                for k in 2..2 + m {
                    rot[k] = cross1.initial_commands[time].rot[k - 2];
                }
                for k in 2 + m..v {
                    rot[k] = cross2.initial_commands[time].rot[k - m - 2];
                }
                initial_commands.push(Operation {
                    mov: Nop,
                    rot: rot.clone(),
                    tako: tako.clone(),
                });
            }
        };
        let has = vec![false; v];
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![cross1, cross2],
            parent: None,
        }
    }
    /// 全体は L でその先に L を乗せる
    ///
    ///      0 --L
    ///      |
    ///      L
    ///
    /// * `v` - 根っこ (0) を除く頂点数
    /// * `scale` - アームの長さ (0以上)
    fn ll(name: String, v: usize, scale: usize, center: (i64, i64)) -> Self {
        use Direction::*;
        let mut tree = vec![(0, scale), (0, scale)];
        let leaves = vec![(0, scale as i64), rot90((0, scale as i64))];
        let m = (v - 2) / 2;
        let mut child1 = Self::larm(format!("{}-l1", name), m, 0, leaves[0]);
        let mut child2 = Self::larm(format!("{}-l2", name), v - 2 - m, 0, leaves[1]);
        child1.parent = Some(0);
        child2.parent = Some(1);
        child2 *= R;
        for k in 0..child1.v {
            tree.push((1, child1.tree[k].1));
        }
        for k in 0..child2.v {
            tree.push((2, child2.tree[k].1));
        }
        let mut initial_commands = vec![];
        {
            let mut rot = vec![Nop; v];
            let tako = vec![false; v + 1];
            rot[1] = R;
            initial_commands.push(Operation {
                mov: Nop,
                rot: rot.clone(),
                tako: tako.clone(),
            });
            rot[1] = Nop;
            for time in 0..1 {
                for k in 2..2 + m {
                    rot[k] = child1.initial_commands[time].rot[k - 2];
                }
                for k in 2 + m..v {
                    rot[k] = child2.initial_commands[time].rot[k - m - 2];
                }
                initial_commands.push(Operation {
                    mov: Nop,
                    rot: rot.clone(),
                    tako: tako.clone(),
                });
            }
        };
        let has = vec![false; v];
        Self {
            name,
            center,
            v,
            tree,
            initial_commands,
            leaves,
            has,
            num_tako: 0,
            children: vec![child1, child2],
            parent: None,
        }
    }
    fn is_full(&self) -> bool {
        self.num_tako == self.v - self.children.len()
    }
    fn is_empty(&self) -> bool {
        self.num_tako == 0
    }
    /// 葉っぱ i の座標
    fn leave_pos(&self, i: usize) -> (i64, i64) {
        if i < self.leaves.len() {
            let x = self.center.0 + self.leaves[i].0;
            let y = self.center.1 + self.leaves[i].1;
            (x, y)
        } else {
            let m = self.children[0].v;
            if i < self.leaves.len() + m {
                self.children[0].leave_pos(i - 2)
            } else {
                self.children[1].leave_pos(i - 2 - m)
            }
        }
    }
    /// 葉っぱ (たこ焼きをつかめる) かどうか判定
    fn is_leave(&self, i: usize) -> bool {
        if i < self.v {
            let set: BTreeSet<usize> = self
                .children
                .iter()
                .map(|child| child.parent.unwrap_or(999))
                .collect();
            !set.contains(&i)
        } else {
            let mut i = i - self.v;
            for child in self.children.iter() {
                if i < child.v {
                    return child.is_leave(i);
                }
                i -= child.v;
            }
            return false;
        }
    }
}

/// 全体の平行移動
impl std::ops::AddAssign<Direction> for Arm {
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
        for child in self.children.iter_mut() {
            *child += d;
        }
    }
}
impl std::ops::Add<Direction> for &Arm {
    type Output = Arm;
    fn add(self, d: Direction) -> Self::Output {
        let mut r = self.clone();
        r += d;
        r
    }
}

/// 全体の回転
impl std::ops::MulAssign<Direction> for Arm {
    fn mul_assign(&mut self, d: Direction) {
        use Direction::*;
        for i in 0..self.leaves.len() {
            if d == R {
                self.leaves[i] = rot90(self.leaves[i]);
            } else if d == L {
                self.leaves[i] = rot90(self.leaves[i]);
                self.leaves[i] = rot90(self.leaves[i]);
                self.leaves[i] = rot90(self.leaves[i]);
            }
        }
        for child in self.children.iter_mut() {
            *child *= d;
            let pa = child.parent.unwrap();
            let x = self.center.0 + self.leaves[pa].0;
            let y = self.center.1 + self.leaves[pa].1;
            child.center = (x, y);
        }
    }
}
impl std::ops::Mul<Direction> for &Arm {
    type Output = Arm;
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
    let v: usize = sc.cin();

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

    // アーム設定に関する探索空間
    // (期待するスコア, 世代, ArmConfig)
    let mut space: BinaryHeap<(Reverse<usize>, usize, ArmConfig)> = BinaryHeap::new();
    {
        let half = (n / 2) as i64;
        let mut scale = 1;
        while scale <= n / 2 {
            for atype in [
                // ArmType::Cross,
                // ArmType::T,
                ArmType::L,
                ArmType::LCross,
                ArmType::LL,
            ] {
                space.push((
                    Reverse(n - scale),
                    0,
                    ArmConfig::new(atype, v - 1, scale, (half, half)),
                ));
            }
            scale *= 2;
        }
    }

    let mut rand = XorShift::new();
    let mut best_score = None;
    let mut best_game = None;
    let now = std::time::SystemTime::now();
    let mut aborting = false; // もうすぐタイムアウト
    loop {
        if let Ok(time) = now.elapsed() {
            if time.as_millis() > 2800 {
                break;
            } else if time.as_millis() > 2300 {
                aborting = true;
            }
        }
        if let Some((_, gen, conf)) = space.pop() {
            let arm = Arm::from(&conf);
            let mut game = Game::new(n, balls.clone(), requires.clone(), arm);
            while !game.end() {
                if aborting && game.time % 20 == 0 {
                    if let Ok(time) = now.elapsed() {
                        if time.as_millis() > 2900 {
                            game.abort = true;
                            break;
                        }
                    }
                }
                game.run();
            }
            if game.succsess() {
                let score = game.score();
                if best_score.is_none() || Some(game.score()) < best_score {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "{} => {}; \x1b[42mupdate\x1b[0m",
                        game.arm.name,
                        game.score()
                    );
                    best_score = Some(game.score());
                    best_game = Some(game);
                } else {
                    #[cfg(debug_assertions)]
                    eprintln!("{} => {}", game.arm.name, game.score());
                }
                {
                    // 次の探索アーム
                    if conf.scale < n / 2 {
                        space.push((
                            Reverse(score),
                            0,
                            ArmConfig::new(conf.atype, conf.v, conf.scale + 1, conf.center),
                        ));
                    }
                    // 初期値の探索
                    if gen < 3 {
                        let x = rand.gen::<usize>() % n;
                        let y = rand.gen::<usize>() % n;
                        space.push((
                            Reverse(score),
                            gen + 1,
                            ArmConfig::new(conf.atype, conf.v, conf.scale, (x as i64, y as i64)),
                        ));
                    }
                }
            } else {
                #[cfg(debug_assertions)]
                eprintln!("{} => ><", game.arm.name);
            }
        } else {
            #[cfg(debug_assertions)]
            eprintln!("No space");
            break;
        }
    }
    if let Some(score) = best_score {
        eprintln!("\x1b[42mscore\x1b[0m {}", score);
        // eprintln!("{}", &best_game.clone().unwrap().arm.name);
        best_game.unwrap().dump();
    } else {
        eprintln!("\x1b[31mFailed\x1b[0m");
    }
}

// @num/random {{{
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
    pub fn shuffle<T>(&mut self, xs: &mut Vec<T>) {
        for i in (1..xs.len()).rev() {
            let j = self.gen::<usize>() % (i + 1);
            if i != j {
                xs.swap(i, j);
            }
        }
    }
}

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
// }}}
// {{{ base
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
