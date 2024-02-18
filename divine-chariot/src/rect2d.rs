////////////////////////////////////////////////////////////////////////////////

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign};

use crate::point2d::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Default,Eq,PartialEq,Ord,PartialOrd,Hash,serde::Serialize,serde::Deserialize)]
pub struct Rect2d {
    pub bl : Point2d,
    pub tr : Point2d,
}

impl Add<Point2d> for Rect2d { type Output=Rect2d; fn add(self, other:Point2d) -> Rect2d { Rect2d{bl:self.bl+other, tr:self.tr+other} } }
impl Sub<Point2d> for Rect2d { type Output=Rect2d; fn sub(self, other:Point2d) -> Rect2d { Rect2d{bl:self.bl-other, tr:self.tr-other} } }

impl Rect2d {
    pub fn new(p1:Point2d, p2:Point2d) -> Self {
        let xmin = p1.x.min(p2.x);
        let xmax = p1.x.max(p2.x);
        let ymin = p1.y.min(p2.y);
        let ymax = p1.y.max(p2.y);
        Rect2d { bl:Point2d{x:xmin,y:ymin}, tr:Point2d{x:xmax,y:ymax} }
    }

    #[inline]
    pub fn size(&self) -> Point2d {
        (self.tr - self.bl)
    }

    #[inline]
    pub fn contains(&self, p:Point2d) -> bool {
        ( self.bl.x <= p.x && p.x < self.tr.x
        && self.bl.y <= p.y && p.y < self.tr.y)
    }

    #[inline]
    pub fn on_boundary(&self, p:Point2d) -> bool {
        p.x==self.bl.x || p.x==self.tr.x-1 || p.y==self.bl.y || p.y==self.tr.y-1
    }

    #[inline]
    pub fn on_corner(&self, p:Point2d) -> bool {
        let l = self.bl.x; let r = self.tr.x-1;
        let b = self.bl.y; let t = self.tr.y-1;
        p==Point2d::new(l,b) || p==Point2d::new(l,t)
        || p==Point2d::new(r,b) || p==Point2d::new(r,t)
    }

    pub fn intersects(&self, other:&Rect2d) -> bool {
        let bl_x = self.bl.x.max(other.bl.x);
        let bl_y = self.bl.y.max(other.bl.y);
        let tr_x = self.tr.x.max(other.tr.x);
        let tr_y = self.tr.y.max(other.tr.y);
        (bl_x < tr_x) && (bl_y < tr_y)
    }

    pub fn intersection(&self, other:&Rect2d) -> Option<Rect2d> {
        let bl_x = self.bl.x.max(other.bl.x);
        let bl_y = self.bl.y.max(other.bl.y);
        let tr_x = self.tr.x.max(other.tr.x);
        let tr_y = self.tr.y.max(other.tr.y);
        if (bl_x < tr_x) && (bl_y < tr_y) {
            Some(Rect2d{bl:Point2d{x:bl_x,y:bl_y}, tr:Point2d{x:tr_x,y:tr_y}})
        } else {
            None
        }
    }

    // converts point in rect to index in 1d-array
    pub fn index(&self, p:Point2d) -> Option<usize> {
        if self.contains(p) {
            let xsize = (self.tr.x - self.bl.x) as usize;
            let adj = p - self.bl;
            Some(adj.x as usize + xsize * adj.y as usize)
        } else {
            None
        }
    }

    // converts 1d-index to point in rect
    pub fn point(&self, idx:usize) -> Option<Point2d> {
        let size = self.size();
        if idx < size.x as usize*size.y as usize {
            Some(self.bl + Point2d::new((idx%size.x as usize) as i32, (idx/size.x as usize) as i32))
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=Point2d> {
        self.into_iter()
    }

    pub fn boundary_iter(&self) -> impl Iterator<Item=Point2d> {
        RectBoundaryIterator::new(*self)
    }
}

impl std::fmt::Debug for Rect2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.bl, self.tr)
    }
}

impl std::fmt::Display for Rect2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.bl, self.tr)
    }
}

////////////////////////////////////////

#[derive(Clone,Copy)]
pub struct RectIterator {
    span: Rect2d,
    index: usize,
}

impl Iterator for RectIterator {
    type Item = Point2d;
    fn next(&mut self) -> Option<Point2d> {
        let res = self.span.point(self.index);
        if !res.is_none() {
            self.index += 1;
        }
        res
    }
}

impl IntoIterator for Rect2d {
    type Item = Point2d;
    type IntoIter = RectIterator;
    fn into_iter(self) -> Self::IntoIter {
        RectIterator { span:self, index:0 }
    }
}

#[derive(Clone,Copy)]
enum RectBoundaryIteratorState { B, R, T, L, Done }

#[derive(Clone,Copy)]
pub struct RectBoundaryIterator {
    span: Rect2d,
    state: RectBoundaryIteratorState,
    next_point: Point2d,
}

impl RectBoundaryIterator {
    fn new(span:Rect2d) -> Self {
        if span.bl.x==span.tr.x || span.bl.y==span.tr.y {
            let state = RectBoundaryIteratorState::Done;
            let next_point = Point2d::default();
            RectBoundaryIterator { span, state, next_point }
        } else {
            let state = RectBoundaryIteratorState::B;
            let next_point = span.bl;
            RectBoundaryIterator { span, state, next_point }
        }
    }
}

impl Iterator for RectBoundaryIterator {
    type Item = Point2d;
    fn next(&mut self) -> Option<Point2d> {
        match self.state {
            RectBoundaryIteratorState::B => {
                let res = Some(self.next_point);
                self.next_point.x += 1;
                if self.next_point.x+1 == self.span.tr.x {
                    self.state = RectBoundaryIteratorState::R;
                }
                res
            }
            RectBoundaryIteratorState::R => {
                let res = Some(self.next_point);
                self.next_point.y += 1;
                if self.next_point.y+1 == self.span.tr.y {
                    self.state = RectBoundaryIteratorState::T;
                }
                res
            }
            RectBoundaryIteratorState::T => {
                let res = Some(self.next_point);
                self.next_point.x -= 1;
                if self.next_point.x == self.span.bl.x {
                    self.state = RectBoundaryIteratorState::L;
                }
                res
            }
            RectBoundaryIteratorState::L => {
                let res = Some(self.next_point);
                self.next_point.y -= 1;
                if self.next_point.y == self.span.bl.y {
                    self.state = RectBoundaryIteratorState::Done;
                }
                res
            }
            RectBoundaryIteratorState::Done => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
