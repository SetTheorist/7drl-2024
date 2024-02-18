////////////////////////////////////////////////////////////////////////////////

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Default,Eq,PartialEq,Ord,PartialOrd,Hash,serde::Serialize,serde::Deserialize)]
pub struct Point2d {
    pub x : i32,
    pub y : i32,
}

static DIRS_4 : [Point2d; 4] = [ Point2d{x:1,y:0}, Point2d{x:0,y:1}, Point2d{x:-1,y:0}, Point2d{x:0,y:-1} ];
static DIRS_8 : [Point2d; 4] = [ Point2d{x:1,y:0}, Point2d{x:0,y:1}, Point2d{x:-1,y:0}, Point2d{x:0,y:-1} ];

impl Point2d {
    pub fn new(x:i32,y:i32) -> Self {
        Point2d{x,y}
    }

    pub fn dirs4() -> impl Iterator<Item=Point2d> {
        DIRS_4.iter().cloned()
    }

    pub fn dirs8() -> impl Iterator<Item=Point2d> {
        DIRS_8.iter().cloned()
    }

    pub fn neighbors4(p:Point2d) -> impl Iterator<Item=Point2d> {
        Point2d::dirs4().map(move|d|p+d)
    }

    pub fn neighbors8(p:Point2d) -> impl Iterator<Item=Point2d> {
        Point2d::dirs8().map(move|d|p+d)
    }
}

impl Add<Point2d> for Point2d { type Output=Point2d; fn add(self, other:Point2d) -> Point2d { Point2d{x:self.x+other.x, y:self.y+other.y} } }
impl Sub<Point2d> for Point2d { type Output=Point2d; fn sub(self, other:Point2d) -> Point2d { Point2d{x:self.x-other.x, y:self.y-other.y} } }
impl Mul<i32    > for Point2d { type Output=Point2d; fn mul(self, other:i32    ) -> Point2d { Point2d{x:self.x*other  , y:self.y*other  } } }
impl Div<i32    > for Point2d { type Output=Point2d; fn div(self, other:i32    ) -> Point2d { Point2d{x:self.x/other  , y:self.y/other  } } }
impl Neg for Point2d { type Output=Point2d; fn neg(self) -> Point2d { Point2d{x:-self.x, y:-self.y} } }
impl Shl<Point2d> for Point2d { type Output=bool; fn shl(self, other:Point2d) -> bool { self.x<=other.x && self.y<=other.y } }
//impl Shr<Point2d> for Point2d { type Output=bool; fn shr(self, other:Point2d) -> bool { self.x> other.x && self.y> other.y } }
impl AddAssign<Point2d> for Point2d { fn add_assign(&mut self, other:Point2d) { *self = *self + other; } }
impl SubAssign<Point2d> for Point2d { fn sub_assign(&mut self, other:Point2d) { *self = *self - other; } }
impl MulAssign<i32    > for Point2d { fn mul_assign(&mut self, other:i32    ) { *self = *self * other; } }
impl DivAssign<i32    > for Point2d { fn div_assign(&mut self, other:i32    ) { *self = *self / other; } }

impl std::fmt::Debug for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::fmt::Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

////////////////////////////////////////////////////////////////////////////////
