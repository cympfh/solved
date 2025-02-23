#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

const STATION: u8 = 0;
const RAIL_HORIZONTAL: u8 = 1;
const RAIL_VERTICAL: u8 = 2;
const RAIL_LEFT_DOWN: u8 = 3;
const RAIL_LEFT_UP: u8 = 4;
const RAIL_RIGHT_UP: u8 = 5;
const RAIL_RIGHT_DOWN: u8 = 6;
const COST_STATION: u64 = 5000;
const COST_RAIL: u64 = 100;
const COST_INF: u64 = 999_999_999_999;

fn udiff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}

// {{{ P
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct P {
    x: usize,
    y: usize,
}
impl P {
    fn new(x: usize, y: usize) -> Self {
        P { x, y }
    }
    fn distance(self, other: P) -> usize {
        udiff(self.x, other.x) + udiff(self.y, other.y)
    }
}
// }}}
// {{{ Cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Station,
    Rail(u8),
}
impl Cell {
    fn is_station(&self) -> bool {
        self == &Cell::Station
    }
    fn is_rail(&self) -> bool {
        if let Cell::Rail(_) = self {
            true
        } else {
            false
        }
    }
    fn connected_to_up(&self) -> bool {
        use Cell::*;
        match self {
            Station | Rail(RAIL_VERTICAL) | Rail(RAIL_LEFT_UP) | Rail(RAIL_RIGHT_UP) => true,
            _ => false,
        }
    }
    fn connected_to_down(&self) -> bool {
        use Cell::*;
        match self {
            Station | Rail(RAIL_VERTICAL) | Rail(RAIL_LEFT_DOWN) | Rail(RAIL_RIGHT_DOWN) => true,
            _ => false,
        }
    }
    fn connected_to_left(&self) -> bool {
        use Cell::*;
        match self {
            Station | Rail(RAIL_HORIZONTAL) | Rail(RAIL_LEFT_DOWN) | Rail(RAIL_LEFT_UP) => true,
            _ => false,
        }
    }
    fn connected_to_right(&self) -> bool {
        use Cell::*;
        match self {
            Station | Rail(RAIL_HORIZONTAL) | Rail(RAIL_RIGHT_UP) | Rail(RAIL_RIGHT_DOWN) => true,
            _ => false,
        }
    }
}
// }}}
// {{{ Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Put(Cell, P),
    Pause,
}

impl Action {
    fn show(self) -> String {
        match self {
            Action::Put(Cell::Empty, p) => panic!("Cannot put Empty (p={:?})", p),
            Action::Put(Cell::Station, p) => format!("{} {} {}", 0, p.x, p.y),
            Action::Put(Cell::Rail(ty), p) => format!("{} {} {}", ty, p.x, p.y),
            Action::Pause => format!("-1"),
        }
    }
}
// }}}

#[derive(Debug, Clone)]
struct Game {
    size: usize,
    board: Vec<Vec<Cell>>,
    uf: UnionFind,
    okane_init: u64,
    okane: u64,
    timeup: usize,
    time: usize,
}

impl Game {
    fn new(size: usize, okane: u64, timeup: usize) -> Self {
        let board = ndarray![Cell::Empty; size, size];
        let uf = UnionFind::new(size * size);
        let time = 0;
        Self {
            size,
            board,
            uf,
            okane_init: okane,
            okane,
            timeup,
            time,
        }
    }
    /// P => ID for UnionFind
    fn pos2id(&self, x: usize, y: usize) -> usize {
        x * self.size + y
    }
    fn left_time(&self) -> usize {
        self.timeup - self.time
    }
    fn act(&mut self, a: Action) {
        match a {
            Action::Put(c, p) => {
                self.build(c, p.x, p.y);
            }
            Action::Pause => {}
        }
        self.time += 1;
    }
    fn build(&mut self, cell: Cell, x: usize, y: usize) {
        use Cell::*;
        assert!(cell != Empty);
        assert!(self.board[x][y] != Station);
        if cell != Station {
            assert!(
                self.board[x][y] == Empty,
                "putting {:?} onto ({}, {}), but this is {:?}",
                cell,
                x,
                y,
                self.board[x][y]
            );
        }
        self.board[x][y] = cell;

        if cell.is_station() {
            self.okane -= COST_STATION
        } else if cell.is_rail() {
            self.okane -= COST_RAIL
        }

        if cell.connected_to_up() {
            if x > 0 && self.board[x - 1][y].connected_to_down() {
                let u = self.pos2id(x, y);
                let v = self.pos2id(x - 1, y);
                self.uf.merge(u, v);
            }
        }
        if cell.connected_to_down() {
            if x < self.size - 1 && self.board[x + 1][y].connected_to_up() {
                let u = self.pos2id(x, y);
                let v = self.pos2id(x + 1, y);
                self.uf.merge(u, v);
            }
        }
        if cell.connected_to_left() {
            if y > 0 && self.board[x][y - 1].connected_to_right() {
                let u = self.pos2id(x, y);
                let v = self.pos2id(x, y - 1);
                self.uf.merge(u, v);
            }
        }
        if cell.connected_to_right() {
            if y < self.size - 1 && self.board[x][y + 1].connected_to_left() {
                let u = self.pos2id(x, y);
                let v = self.pos2id(x, y + 1);
                self.uf.merge(u, v);
            }
        }
    }
    fn is_connected(&mut self, s: P, t: P) -> bool {
        let ss = self.around_stations(s);
        let ts = self.around_stations(t);
        for u in ss {
            for v in ts.iter() {
                let a = self.pos2id(u.x, u.y);
                let b = self.pos2id(v.x, v.y);
                if self.uf.is_same(a, b) {
                    return true;
                }
            }
        }
        false
    }
    fn around_stations(&self, p: P) -> Vec<P> {
        let mut stations = vec![];
        let neigh = neighbor::Grid4(self.size, self.size);
        for (x, y) in neigh.around(p.x, p.y) {
            if self.board[x][y].is_station() {
                stations.push(P::new(x, y));
            }
        }
        stations
    }
}

struct Solver {
    game: Game,
    ps: Vec<(P, P)>,
    plan: Vec<(Action, u64)>, // (action, 実行後のお金)
    connected: Vec<bool>,     // connected[i] = (ps[i] has been already done)
}
impl Solver {
    fn new(game: Game, ps: Vec<(P, P)>) -> Self {
        let plan = vec![];
        let connected = vec![false; ps.len()];
        Self {
            game,
            ps,
            plan,
            connected,
        }
    }
    fn income(&mut self) -> u64 {
        let mut sum = 0;
        for i in 0..self.ps.len() {
            // if self.connected[i] {
            //     let (u, v) = self.ps[i];
            //     let d = u.distance(v);
            //     sum += d as u64;
            // }
            let (u, v) = self.ps[i];
            if self.game.is_connected(u, v) {
                let d = u.distance(v);
                sum += d as u64;
            }
        }
        sum
    }

    fn rollback(&mut self) {
        let mut max_okane = 0;
        let mut max_time = 0;
        for t in 0..self.plan.len() {
            if max_okane < self.plan[t].1 {
                max_okane = self.plan[t].1;
                max_time = t;
            }
        }
        if max_time == 0 {
            comment!("#rollback to 0 (${})!!", max_okane);
            for t in 0..self.plan.len() {
                self.plan[t] = (Action::Pause, max_okane);
            }
        } else {
            comment!("#rollback to {} (${})", max_time + 1, max_okane);
            for t in max_time + 1..self.plan.len() {
                self.plan[t] = (Action::Pause, max_okane);
            }
        }
    }

    fn show(&self) {
        for &(p, _) in self.plan.iter() {
            println!("{}", p.show());
        }
    }

    fn do_plan(&mut self, newplan: Vec<Action>) {
        for &p in newplan.iter() {
            self.game.act(p);
            self.game.okane += self.income();
            self.plan.push((p, self.game.okane));
        }
    }

    /// 東京駅をハブとしてつなぐ
    /// 全体は連結であることを保証しながら作る
    fn solve_tokyo(&mut self) {
        use Action::*;
        use Cell::*;
        let neigh = neighbor::Grid4(self.game.size, self.game.size);
        let pairs = {
            let mut map = HashMap::new();
            for &(p, q) in self.ps.iter() {
                map.insert(p, q);
            }
            map
        };
        let tokyo = {
            let mut tokyo_kari = P::new(999, 999);
            let mut max_sum_income = 0;
            for x in 0..self.game.size {
                for y in 0..self.game.size {
                    let mut min_cost = COST_INF;
                    let mut sum_income = 0;
                    for (x2, y2) in neigh.around(x, y) {
                        let p = P::new(x2, y2);
                        if let Some(&q) = pairs.get(&p) {
                            let cost = 10_000 + 100 * (p.distance(q) - 5) as u64;
                            min_cost = min!(min_cost, cost);
                            sum_income += p.distance(q);
                        }
                    }
                    if max_sum_income < sum_income {
                        max_sum_income += sum_income;
                        tokyo_kari = P::new(x, y);
                    }
                }
            }
            tokyo_kari
        };

        // failed
        if tokyo == P::new(999, 999) {
            comment!("# Not found tokyo");
            self.do_plan(vec![Pause; self.game.timeup]);
            return;
        }

        // 東京駅
        comment!("#tokyo={:?}", tokyo);
        self.do_plan(vec![Put(Station, tokyo)]);

        let mut goals = vec![];
        for i in 0..self.ps.len() {
            let (p, q) = self.ps[i];
            let g = if tokyo.distance(p) <= 2 {
                Some(q)
            } else if tokyo.distance(q) <= 2 {
                Some(p)
            } else {
                None
            };
            if let Some(u) = g {
                // let cost = 10_000 + 100 * (tokyo.distance(u) - 5) as u64;
                goals.push((i, u, tokyo.distance(u)));
                comment!("#goal[{}] = {:?}, d={}", i, u, tokyo.distance(u));
            }
        }
        let mut goals: HashSet<_> = goals.into_iter().collect();

        // building
        loop {
            comment!(
                "#loop [{}/{}] ${}",
                self.game.time,
                self.game.timeup,
                self.game.okane
            );
            let mut minimal_cost = COST_INF; // 建築可能性を問わない最小コスト
            let mut best_plan = None; // 建築可能な最高プラン
            let mut best_plans_d = 0;
            for &(i, p, d) in goals.iter() {
                if self.connected[i] {
                    continue;
                }
                let (plan, cost_okane, cost_time) = self.connect_tokyo(tokyo, p);
                comment!(
                    "#planning ... tokyo -> {} ({:?}); cost=${}, t={}",
                    i,
                    p,
                    cost_okane,
                    cost_time
                );
                if plan.is_empty() {
                    continue;
                }
                minimal_cost = min!(minimal_cost, cost_okane);
                // 建築可能
                if cost_okane <= self.game.okane && cost_time <= self.game.left_time() {
                    if best_plan.is_none() || best_plans_d < d {
                        best_plan = Some((i, (i, p, d), plan, cost_okane, cost_time));
                        best_plans_d = d;
                    }
                }
            }
            trace!(&best_plan, best_plans_d);
            if let Some((i, goal, plan, cost_okane, cost_time)) = best_plan {
                self.do_plan(plan);
                self.connected[i] = true;
                goals.remove(&goal);
                comment!(
                    "#do_plan ... -> {} ({:?}) ${} t={}",
                    goal.0,
                    goal.1,
                    cost_okane,
                    cost_time
                );
            } else if minimal_cost < COST_INF {
                if self.income() > 0 {
                    let t = clip!(
                        ((minimal_cost - self.game.okane) / self.income()) as usize,
                        1,
                        self.game.left_time()
                    );
                    let pauses = vec![Action::Pause; t];
                    self.do_plan(pauses);
                    comment!("#pause t={}", t);
                } else {
                    comment!("#no income");
                    break;
                }
            } else {
                comment!("#No path found");
                break;
            }
            if self.game.left_time() == 0 {
                break;
            }
        }
        let t = self.game.left_time();
        let pauses = vec![Action::Pause; t];
        self.do_plan(pauses);
    }

    fn solve_greedy(&mut self) {
        loop {
            comment!(
                "#loop [{}/{}] ${}",
                self.game.time,
                self.game.timeup,
                self.game.okane
            );
            type PlanT = Option<(usize, Vec<Action>, u64, usize, f64)>;
            let mut minimal_plan: PlanT = None; // コスト内とは限らない最小
            let mut minimal_keyi_plan: PlanT = None; // コスト内で最小
            for i in 0..self.ps.len() {
                if self.connected[i] {
                    continue;
                }
                let (u, v) = self.ps[i];
                let (plan, cost_okane, cost_time) = self.connect_plan(u, v);
                if plan.is_empty() {
                    continue;
                }
                let cost_performance = {
                    let d = u.distance(v);
                    cost_okane as f64 / d as f64
                };
                let someplan = Some((i, plan, cost_okane, cost_time, cost_performance));
                let keyi = cost_okane <= self.game.okane && cost_time <= self.game.left_time();
                if minimal_plan.is_none() || minimal_plan.as_ref().unwrap().4 > cost_performance {
                    minimal_plan = someplan.clone();
                }
                if keyi
                    && (minimal_keyi_plan.is_none()
                        || minimal_keyi_plan.as_ref().unwrap().4 > cost_performance)
                {
                    minimal_keyi_plan = someplan.clone();
                }
            }
            if let Some((i, plan, cost_okane, cost_time, _)) = minimal_keyi_plan {
                self.do_plan(plan);
                self.connected[i] = true;
                comment!("#do_plan ${} t={}", cost_okane, cost_time);
            } else if let Some((_, _, cost_okane, _, _)) = minimal_plan {
                if self.income() > 0 {
                    let t = clip!(
                        ((cost_okane - self.game.okane) / self.income()) as usize,
                        1,
                        self.game.left_time()
                    );
                    let pauses = vec![Action::Pause; t];
                    self.do_plan(pauses);
                    comment!("#pause t={}", t);
                } else {
                    comment!("#no income");
                    break;
                }
            } else {
                comment!("#No path found");
                break;
            }
            if self.game.left_time() == 0 {
                break;
            }
        }
        let t = self.game.left_time();
        let pauses = vec![Action::Pause; t];
        self.do_plan(pauses);
    }

    /// 到達不能な可能性があることに注意
    /// u, v それぞれの周りの駅を勝手に使う
    fn connect_plan(&self, u: P, v: P) -> (Vec<Action>, u64, usize) {
        use Action::*;
        use Cell::*;

        let neigh = neighbor::Grid4(self.game.size, self.game.size);
        let mut costtable = ndarray![COST_INF; self.game.size, self.game.size];
        let mut fromtable = ndarray![(0, 0); self.game.size, self.game.size];
        let mut q = VecDeque::new();

        // start
        for (sx, sy) in neigh.around(u.x, u.y) {
            match self.game.board[sx][sy] {
                Station => {
                    costtable[sx][sy] = 0;
                    fromtable[sx][sy] = (sx, sy);
                }
                _ => {
                    costtable[sx][sy] = COST_STATION;
                    fromtable[sx][sy] = (sx, sy);
                }
            }
            q.push_back((Reverse(costtable[sx][sy]), sx, sy));
        }
        while let Some((Reverse(cost), x, y)) = q.pop_front() {
            if cost > costtable[x][y] {
                continue;
            }
            for (x2, y2) in neigh.iter(x, y) {
                let is_goal = v.distance(P::new(x2, y2)) <= 2;
                let pluscost = if self.game.board[x2][y2].is_station() {
                    0
                } else if is_goal {
                    COST_STATION
                } else {
                    COST_RAIL
                };
                let newcost = cost + pluscost;
                if costtable[x2][y2] > newcost {
                    costtable[x2][y2] = newcost;
                    fromtable[x2][y2] = (x, y);
                    q.push_back((Reverse(newcost), x2, y2));
                }
            }
        }

        // goal?
        let mut t = v.clone();
        {
            let mut minimal_goal_cost = COST_INF;
            for (x, y) in neigh.around(t.x, t.y) {
                if costtable[x][y] < COST_INF && minimal_goal_cost > costtable[x][y] {
                    t = P::new(x, y);
                    minimal_goal_cost = costtable[x][y];
                }
            }
            // failed
            if minimal_goal_cost >= COST_INF {
                return (vec![], 0, 0);
            }
        }

        // build a path
        let mut failed = false;
        let mut path = vec![];
        let mut dcost = vec![];
        {
            let mut x = t.x;
            let mut y = t.y;
            loop {
                path.push((x, y));
                let (xp, yp) = fromtable[x][y];
                if P::new(x, y).distance(P::new(xp, yp)) != 1 {
                    failed = true;
                    break;
                }
                dcost.push(costtable[x][y] - costtable[xp][yp]);
                (x, y) = (xp, yp);
                let is_goal = P::new(x, y).distance(u) <= 2;
                if is_goal {
                    break;
                }
            }
            path.push((x, y));
            dcost.push(costtable[x][y]);
        }
        if failed {
            return (vec![], 0, 0);
        }

        let mut plan = vec![];
        {
            // goal
            let (x, y) = path[0];
            if !self.game.board[x][y].is_station() {
                plan.push(Put(Station, P::new(x, y)))
            }
        }
        {
            //start
            let (x, y) = path[path.len() - 1];
            if !self.game.board[x][y].is_station() {
                plan.push(Put(Station, P::new(x, y)))
            }
        }
        for i in 1..path.len() - 1 {
            let (x, y) = path[i];
            if self.game.board[x][y].is_station() {
                continue;
            }
            if dcost[i] == COST_STATION {
                plan.push(Put(Station, P::new(x, y)));
            } else {
                let (xp, yp) = path[i - 1];
                let (xq, yq) = path[i + 1];
                let rail_type = if xp == xq {
                    RAIL_HORIZONTAL
                } else if yp == yq {
                    RAIL_VERTICAL
                } else if max!(xp, xq) == x && max!(yp, yq) == y {
                    RAIL_LEFT_UP
                } else if max!(xp, xq) == x && min!(yp, yq) == y {
                    RAIL_RIGHT_UP
                } else if min!(xp, xq) == x && max!(yp, yq) == y {
                    RAIL_LEFT_DOWN
                } else {
                    RAIL_RIGHT_DOWN
                };
                plan.push(Put(Rail(rail_type), P::new(x, y)));
            }
        }

        let cost_okane = costtable[t.x][t.y];
        let cost_time = plan.len();
        (plan, cost_okane, cost_time)
    }

    /// tokyo: P はすでに東京駅があるのでこれを使う
    /// v はその周辺に適当に駅を作る
    fn connect_tokyo(&self, tokyo: P, v: P) -> (Vec<Action>, u64, usize) {
        use Action::*;
        use Cell::*;

        let neigh = neighbor::Grid4(self.game.size, self.game.size);
        let mut costtable = ndarray![COST_INF; self.game.size, self.game.size];
        let mut fromtable = ndarray![(0, 0); self.game.size, self.game.size];
        let mut q = VecDeque::new();

        assert!(self.game.board[tokyo.x][tokyo.y].is_station());
        costtable[tokyo.x][tokyo.y] = 0;
        fromtable[tokyo.x][tokyo.y] = (tokyo.x, tokyo.y);
        q.push_back((Reverse(0), tokyo.x, tokyo.y));

        while let Some((Reverse(cost), x, y)) = q.pop_front() {
            if cost > costtable[x][y] {
                continue;
            }
            for (x2, y2) in neigh.iter(x, y) {
                let is_goal = v.distance(P::new(x2, y2)) <= 2;
                let pluscost = if self.game.board[x2][y2].is_station() {
                    0
                } else if is_goal {
                    COST_STATION
                } else if self.game.board[x2][y2].is_rail() {
                    COST_STATION
                } else {
                    COST_RAIL
                };
                let newcost = cost + pluscost;
                if costtable[x2][y2] > newcost {
                    costtable[x2][y2] = newcost;
                    fromtable[x2][y2] = (x, y);
                    q.push_back((Reverse(newcost), x2, y2));
                }
            }
        }

        // goal?
        let mut t = v.clone();
        {
            let mut minimal_goal_cost = COST_INF;
            for (x, y) in neigh.around(t.x, t.y) {
                if costtable[x][y] < COST_INF && minimal_goal_cost > costtable[x][y] {
                    t = P::new(x, y);
                    minimal_goal_cost = costtable[x][y];
                }
            }
            // failed
            if minimal_goal_cost >= COST_INF {
                return (vec![], 0, 0);
            }
        }

        // build a path
        let mut failed = false;
        let mut path = vec![];
        let mut dcost = vec![];
        {
            let mut x = t.x;
            let mut y = t.y;
            loop {
                path.push((x, y));
                let (xp, yp) = fromtable[x][y];
                if P::new(x, y).distance(P::new(xp, yp)) != 1 {
                    failed = true;
                    break;
                }
                dcost.push(costtable[x][y] - costtable[xp][yp]);
                (x, y) = (xp, yp);
                if tokyo == P::new(x, y) {
                    break;
                }
            }
            path.push((x, y));
            dcost.push(costtable[x][y]);
        }
        if failed {
            return (vec![], 0, 0);
        }

        // 東京駅はすでにある
        let mut plan = vec![];
        {
            // goal
            let (x, y) = path[0];
            if !self.game.board[x][y].is_station() {
                plan.push(Put(Station, P::new(x, y)))
            }
        }
        for i in 1..path.len() - 1 {
            let (x, y) = path[i];
            if self.game.board[x][y].is_station() {
                continue;
            }
            if dcost[i] == COST_STATION {
                plan.push(Put(Station, P::new(x, y)));
            } else {
                let (xp, yp) = path[i - 1];
                let (xq, yq) = path[i + 1];
                let rail_type = if xp == xq {
                    RAIL_HORIZONTAL
                } else if yp == yq {
                    RAIL_VERTICAL
                } else if max!(xp, xq) == x && max!(yp, yq) == y {
                    RAIL_LEFT_UP
                } else if max!(xp, xq) == x && min!(yp, yq) == y {
                    RAIL_RIGHT_UP
                } else if min!(xp, xq) == x && max!(yp, yq) == y {
                    RAIL_LEFT_DOWN
                } else {
                    RAIL_RIGHT_DOWN
                };
                plan.push(Put(Rail(rail_type), P::new(x, y)));
            }
        }

        let cost_okane = costtable[t.x][t.y];
        let cost_time = plan.len();
        (plan, cost_okane, cost_time)
    }
}

fn main() {
    let mut sc = Scanner::default();
    let n: usize = sc.cin();
    let m: usize = sc.cin();
    let k: u64 = sc.cin();
    let t: usize = sc.cin();

    let ps: Vec<(P, P)> = (0..m)
        .map(|_| {
            let a: usize = sc.cin();
            let b: usize = sc.cin();
            let c: usize = sc.cin();
            let d: usize = sc.cin();
            (P::new(a, b), P::new(c, d))
        })
        .collect();
    let game = Game::new(n, k, t);

    let mut solver = Solver::new(game, ps);
    solver.solve_tokyo();
    comment!("#Score ${}", solver.game.okane);
    // solver.rollback();
    solver.show();
}

// {{{ @set/union_find
/// Set - Union-Find
#[derive(Debug, Clone)]
pub struct UnionFind {
    data: Vec<UF>,
}

#[derive(Debug, Clone)]
enum UF {
    Root(usize),
    Child(usize),
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            data: vec![UF::Root(1); n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        match self.data[x] {
            UF::Root(_) => x,
            UF::Child(parent) => {
                let root = self.root(parent);
                self.data[x] = UF::Child(root);
                root
            }
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        match self.data[r] {
            UF::Root(size) => size,
            UF::Child(_) => 0,
        }
    }
    pub fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            let size_x = self.size(root_x);
            let size_y = self.size(root_y);
            let (i, j) = if size_x > size_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };
            self.data[i] = UF::Root(size_x + size_y);
            self.data[j] = UF::Child(i);
        }
    }
}
// }}}
// {{{ @misc/neighbor
/// Misc - Neighbor
pub mod neighbor {
    pub struct Grid4(pub usize, pub usize);
    impl Grid4 {
        pub fn iter(&self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            let mut v = vec![];
            for s in 0..3 {
                for t in 0..3 {
                    if (s + t) % 2 == 1
                        && (1..self.0 + 1).contains(&(i + s))
                        && (1..self.1 + 1).contains(&(j + t))
                    {
                        v.push((i + s - 1, j + t - 1));
                    }
                }
            }
            VecIter(v)
        }
        /// 距離2以下の場所
        pub fn around(&self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            fn udiff(x: usize, y: usize) -> usize {
                if x > y {
                    x - y
                } else {
                    y - x
                }
            }
            let mut v = vec![];
            for s in 0..5 {
                for t in 0..5 {
                    if udiff(s, 2) + udiff(t, 2) <= 2
                        && (2..self.0 + 2).contains(&(i + s))
                        && (2..self.1 + 2).contains(&(j + t))
                    {
                        v.push((i + s - 2, j + t - 2));
                    }
                }
            }
            VecIter(v)
        }
    }
    pub struct Grid8(pub usize, pub usize);
    impl Grid8 {
        pub fn iter<'a>(&'a self, i: usize, j: usize) -> VecIter<(usize, usize)> {
            let mut v = vec![];
            for s in 0..3 {
                for t in 0..3 {
                    if (s * t) != 1
                        && (1..self.0 + 1).contains(&(i + s))
                        && (1..self.1 + 1).contains(&(j + t))
                    {
                        v.push((i + s - 1, j + t - 1));
                    }
                }
            }
            VecIter(v)
        }
    }
    pub struct VecIter<T>(Vec<T>);
    impl<T: Copy> Iterator for VecIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }
}
// }}}
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
macro_rules! clip {
    ($x:expr, $min:expr, $max:expr) => {{
        max!($min, min!($max, $x))
    }};
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
macro_rules! comment {
    ($format:expr) => {
        eprintln!($format);
        println!($format);
    };
    ($format:expr, $($xs:expr),*) => {
        eprintln!($format, $($xs),*);
        println!($format, $($xs),*);
    }
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
