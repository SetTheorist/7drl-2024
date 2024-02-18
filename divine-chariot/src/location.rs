////////////////////////////////////////////////////////////////////////////////

use crate::map::*;
use crate::rect2d::*;
use crate::point2d::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Default,Eq,PartialEq,Ord,PartialOrd,Hash,serde::Serialize,serde::Deserialize)]
pub struct Location {
    pub p: Point2d,
    pub m: MapId,
}

impl Location {
    pub fn new(p:Point2d, m:MapId) -> Self {
        Location { p, m }
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "L{}@{}", self.p, self.m)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "L{}@{}", self.p, self.m)
    }
}

////////////////////////////////////////////////////////////////////////////////
