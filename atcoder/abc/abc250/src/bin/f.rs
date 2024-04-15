/// Unsolved!
#![allow(unused_imports, unused_macros, dead_code)]
use std::{cmp::*, collections::*};

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.cin();
    let ps: Vec<Point> = (0..n)
        .map(|_| {
            let x: i128 = sc.cin();
            let y: i128 = sc.cin();
            Point(x * 2, y * 2)
        })
        .collect();

    // 面積は今 8 倍されてる
    let poly = Polygon(ps.clone());
    let all = poly.area();
    trace!(poly.area());

    // cycled
    let ps: Vec<Point> = ps.iter().chain(ps.iter()).cloned().collect();

    // 三角形 [O, ps[i], ps[i+1]]
    let mut triangle_areas = vec![0; n * 3];
    for i in 0..n {
        let triangle = poly!(Point::zero(), ps[i], ps[(i + 1) % n]);
        triangle_areas[i] = triangle.area();
        triangle_areas[n + i] = triangle.area();
        triangle_areas[n + n + i] = triangle.area();
    }
    trace!(&triangle_areas);

    let st = SegmentTreeSum::from(triangle_areas);

    let mut ans = all;
    for u in 0..n {
        let mut left = 1;
        let mut right = n - 1;
        for _ in 0..50 {
            if left + 1 == right {
                break;
            }
            let m1 = (left + left + right) / 3;
            let m2 = (left + right + right) / 3;
            trace!(u, n, (left, right), (m1, m2));
            let z1 = {
                let tri = poly!(Point::zero(), ps[u], ps[u + m1]);
                let d1 = st.product(u..u + m1) - tri.area();
                let d2 = all - d1;
                let dif1 = (all - d1 * 4).abs();
                let dif2 = (all - d2 * 4).abs();
                ans = min!(ans, dif1, dif2);
                min!(dif1, dif2)
            };
            let z2 = {
                let tri = poly!(Point::zero(), ps[u], ps[u + m2]);
                let d1 = st.product(u..u + m2) - tri.area();
                let d2 = all - d1;
                let dif1 = (all - d1 * 4).abs();
                let dif2 = (all - d2 * 4).abs();
                ans = min!(ans, dif1, dif2);
                min!(dif1, dif2)
            };
            trace!(z1, z2);
            if z1 == z2 {
                left = min!(left + 1, right - 1);
            } else if z1 < z2 {
                right = m2;
            } else {
                left = m1;
            }
            if m1 + 1 >= m2 {
                break;
            }
        }
    }
    put!(ans / 4);
}

// @geometry2d/polygon
// @geometry2d/ccw
// @geometry2d/line
// @geometry2d/point
/// Geometry2D - Definition of Point
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i128, pub i128);

impl Point {
    pub fn new(x: i128, y: i128) -> Self {
        Self(x, y)
    }
    pub fn zero() -> Point {
        Point(0, 0)
    }
    pub fn det(&self, other: &Point) -> i128 {
        self.0 * other.1 - self.1 * other.0
    }
    // pub fn distance(&self, other: &Point) -> i128 {
    //     (*self - *other).norm()
    // }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point(-self.0, -self.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        self + (-other)
    }
}
// scalar multiplication
impl std::ops::Mul<Point> for i128 {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point(self * other.0, self * other.1)
    }
}
impl std::ops::Mul<i128> for Point {
    type Output = Point;
    fn mul(self, other: i128) -> Point {
        Point(other * self.0, other * self.1)
    }
}
// inner-product
impl std::ops::Mul<Point> for Point {
    type Output = i128;
    fn mul(self, other: Point) -> i128 {
        self.0 * other.0 + self.1 * other.1
    }
}
impl std::ops::Div<i128> for Point {
    type Output = Point;
    fn div(self, other: i128) -> Point {
        Point(self.0 / other, self.1 / other)
    }
}

/// Geometry2D - Definition of Line
#[derive(Debug, Clone, Copy)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        assert!(x != y);
        Self(x, y)
    }
    // pub fn distance_from(&self, p: Point) -> i128 {
    //     let u = p - self.0;
    //     let v = self.1 - self.0;
    //     (u.det(&v) / v.norm()).abs()
    // }
}
// impl std::cmp::PartialEq for Line {
//     fn eq(&self, other: &Line) -> bool {
//         let a = Point::zero();
//         let b = Point(1, 0);
//         let c = Point(0, 1);
//         let eps = 1e-6;
//         for p in &[a, b, c] {
//             if (self.distance_from(*p) - other.distance_from(*p)).abs() > eps {
//                 return false;
//             }
//         }
//         true
//     }
// }
// impl std::cmp::Eq for Line {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment(pub Point, pub Point);

impl LineSegment {
    pub fn new(x: Point, y: Point) -> Self {
        assert!(x != y);
        Self(x, y)
    }
    pub fn to_line(&self) -> Line {
        Line(self.0, self.1)
    }
}
impl std::ops::Neg for LineSegment {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.1, self.0)
    }
}

#[macro_export]
macro_rules! line {
    ($x0:expr, $y0:expr; $x1:expr, $y1:expr) => {
        Line::new(Point($x0, $y0), Point($x1, $y1))
    };
    ($x0:expr, $y0:expr => $x1:expr, $y1:expr) => {
        LineSegment::new(Point($x0, $y0), Point($x1, $y1))
    };
    ($a:expr; $b:expr) => {
        Line::new($a, $b)
    };
    ($a:expr => $b:expr) => {
        LineSegment::new($a, $b)
    };
}

/// Geometry2D - CCW (線分と点の関係)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CCW {
    Front,
    Back,
    Right,
    Left,
    On,
}

pub fn ccw(l: LineSegment, p: Point) -> CCW {
    use CCW::*;
    let dif = p - l.0;
    let dir = l.1 - l.0;
    if dir.0 == 0 {
        if dif.0 == 0 {
            let k = dif.1 / dir.1;
            if k > 1 {
                Front
            } else if k < 0 {
                Back
            } else {
                On
            }
        } else if dir.det(&dif) > 0 {
            Left
        } else {
            Right
        }
    } else {
        let k = dif.0 / dir.0;
        if dir.1 * k == dif.1 {
            if k > 1 {
                Front
            } else if k < 0 {
                Back
            } else {
                On
            }
        } else if dir.det(&dif) > 0 {
            Left
        } else {
            Right
        }
    }
}

/// Geometry2D - Definition of Polygon
#[derive(Debug, Clone)]
pub struct Polygon(Vec<Point>);

impl Polygon {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// self must be counter-clockwised.
    pub fn contains(&self, p: &Point, is_strict: bool) -> bool {
        let n = self.len();
        for i in 0..n {
            let u = self.0[i];
            let v = self.0[(i + 1) % n];
            let edge = LineSegment(u, v);
            match ccw(edge, *p) {
                CCW::On => {
                    if is_strict {
                        return false;
                    }
                }
                CCW::Left => continue,
                _ => return false,
            }
        }
        true
    }
}

impl std::ops::Index<usize> for Polygon {
    type Output = Point;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl Polygon {
    pub fn area(&self) -> i128 {
        (1..self.len() - 1)
            .map(|i| {
                let u = self[i] - self[0];
                let v = self[i + 1] - self[0];
                u.det(&v)
            })
            .sum::<i128>()
    }
}

#[macro_export]
macro_rules! poly {
    ($($x:expr),+) => {{
        let v = vec![$($x),+];
        Polygon(v)
    }};
    ($($x:expr),+ ,) => (poly!($($x),+))
}

// @sequence/tree/segment_tree_sum
// @algebra/monoid_sum
// @algebra/monoid
/// Algebra - Def of Monoid (*, 1)
pub trait Monoid: std::ops::Mul<Output = Self> + std::iter::Product
where
    Self: std::marker::Sized,
{
    fn one() -> Self;
}

#[macro_export]
macro_rules! monoid {
    (
        $type:ty where [ $( $params:tt )* ];
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Mul for $type {
            type Output = Self;
            fn mul($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::MulAssign for $type where Self: Clone {
            fn mul_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() * $y;
            }
        }
        impl<$($params)*> std::iter::Product for $type {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::one(), std::ops::Mul::mul)
            }
        }
        impl<$($params)*> Monoid for $type {
            fn one() -> Self { $one }
        }
    };
    (
        $type:ty;
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        monoid! { $type where []; one = $one; mul($self, $y) = $code; }
    };
}

impl Monoid for i128 {
    fn one() -> Self {
        1
    }
}

/// Algebra - Def of Monoid (i128, +)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sum(pub i128);
monoid! {
    Sum;
    one = Sum(0);
    mul(self, other) = {
        Self(self.0 + other.0)
    };
}

// @sequence/tree/segment_tree
/// Sequence - Segment Tree
#[derive(Debug, Clone)]
pub struct SegmentTree<X> {
    length: usize,       // of leaves
    length_upper: usize, // power of 2
    size: usize,         // of nodes
    data: Vec<X>,
}
impl<X> std::ops::Index<usize> for SegmentTree<X> {
    type Output = X;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[self.size / 2 + i]
    }
}
impl<X: Copy + Monoid> SegmentTree<X> {
    pub fn new(length: usize) -> Self {
        let mut length_upper = 1;
        while length_upper < length {
            length_upper <<= 1
        }
        let size = length_upper * 2 - 1;
        let data = vec![X::one(); size];
        SegmentTree {
            length,
            length_upper,
            size,
            data,
        }
    }
    pub fn from(xs: Vec<X>) -> Self {
        let mut tree = Self::new(xs.len());
        for i in 0..xs.len() {
            tree.data[tree.size / 2 + i] = xs[i];
        }
        for i in (0..tree.size / 2).rev() {
            tree.data[i] = tree.data[2 * i + 1] * tree.data[2 * i + 2];
        }
        tree
    }
    pub fn to_vec(self) -> Vec<X> {
        self.data[self.size / 2..].to_vec()
    }
    pub fn update(&mut self, i: usize, t: X) {
        let mut u = self.size / 2 + i;
        self.data[u] = t;
        while u > 0 {
            u = (u - 1) / 2;
            self.data[u] = self.data[u * 2 + 1] * self.data[u * 2 + 2];
        }
    }
    fn product_sub(
        &self,
        range: std::ops::Range<usize>,
        u: usize,
        focus: std::ops::Range<usize>,
    ) -> X {
        if focus.end <= range.start || range.end <= focus.start {
            X::one()
        } else if range.start <= focus.start && focus.end <= range.end {
            self.data[u]
        } else {
            let mid = (focus.start + focus.end) / 2;
            let a = self.product_sub(range.clone(), u * 2 + 1, focus.start..mid);
            let b = self.product_sub(range.clone(), u * 2 + 2, mid..focus.end);
            a * b
        }
    }
    pub fn product(&self, range: std::ops::Range<usize>) -> X {
        self.product_sub(range, 0, 0..self.length_upper)
    }
}
impl<X: std::fmt::Debug> SegmentTree<X> {
    pub fn debug(&self) {
        #[cfg(debug_assertions)]
        for i in 0..self.size {
            if i > 0 && (i + 1).count_ones() == 1 {
                eprintln!();
            }
            eprint!("{:?} ", &self.data[i]);
        }
        eprintln!();
    }
}

/// Sequence - Segment Tree of Sum
pub struct SegmentTreeSum {
    pub t: SegmentTree<Sum>,
}
impl SegmentTreeSum {
    pub fn new(size: usize) -> Self {
        let t = SegmentTree::new(size);
        Self { t }
    }
    pub fn from(xs: Vec<i128>) -> Self {
        let t = SegmentTree::from(xs.iter().map(|&x| Sum(x)).collect());
        Self { t }
    }
    pub fn to_vec(self) -> Vec<i128> {
        self.t.to_vec().iter().map(|sm| sm.0).collect()
    }
    pub fn update(&mut self, i: usize, x: i128) {
        self.t.update(i, Sum(x));
    }
    pub fn product(&self, range: std::ops::Range<usize>) -> i128 {
        self.t.product(range).0
    }
}

// {{{
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }
    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
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
        let x = self.cin::<S>();
        let y = self.cin::<T>();
        (x, y)
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
