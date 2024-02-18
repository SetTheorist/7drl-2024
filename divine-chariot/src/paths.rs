////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap};
use std::hash::Hash;
use std::ops::{Add};

////////////////////////////////////////////////////////////////////////////////

use crate::point2d::*;
use crate::rect2d::*;
use crate::priority_queue::*;

////////////////////////////////////////////////////////////////////////////////

pub fn bresenham(start:Point2d, goal:Point2d, path:&mut Vec<Point2d>) -> usize
{
    path.clear();
    for p in BresenhamIterator::new(start, goal) {
        path.push(p);
    }
    return path.len();
}

#[derive(Clone,Copy)]
pub struct BresenhamIterator {
    p1: Point2d,
    p2: Point2d,
    dp: Point2d,
    ip: Point2d,
    dp2: Point2d,
    error: i32, derr1: i32, derr2: i32,
    mod1: Point2d, mod2: Point2d,
    iflag: bool,
    mode: BresenhamIteratorMode,
}


#[derive(Clone,Copy)]
enum BresenhamIteratorMode {
    Start, Path, Done
}

impl BresenhamIterator {
    pub fn new(start:Point2d, goal:Point2d) -> Self {
        let p1 = start;
        let p2 = goal;
        let dp = p2 - p1;
        let dp2 = Point2d::new(dp.x.abs(), dp.y.abs())*2;
        let ipx = if dp.x>0 {1} else if dp.x<0 {-1} else {0};
        let ipy = if dp.y>0 {1} else if dp.y<0 {-1} else {0};
        let ip = Point2d::new(ipx, ipy);
        let (error,derr1,derr2,mod1,mod2,iflag) = {
            if dp2.x >= dp2.y {
                let error = dp2.y - (dp2.x / 2);
                let derr1 = -dp2.x;
                let derr2 = dp2.y;
                let mod1 = Point2d::new(0,ip.y);
                let mod2 = Point2d::new(ip.x,0);
                let iflag = ip.x > 0;
                (error, derr1, derr2, mod1, mod2, iflag)
            } else {
                let error = dp2.x - (dp2.y / 2);
                let derr1 = -dp2.y;
                let derr2 = dp2.x;
                let mod1 = Point2d::new(ip.x,0);
                let mod2 = Point2d::new(0,ip.y);
                let iflag = ip.y > 0;
                (error, derr1, derr2, mod1, mod2, iflag)
            }
        };
        let mode = BresenhamIteratorMode::Start;
        BresenhamIterator { p1, p2, dp, ip, dp2, error, derr1, derr2, mod1, mod2, iflag, mode }
    }
}

impl Iterator for BresenhamIterator {
    type Item = Point2d;
    fn next(&mut self) -> Option<Point2d> {
        match self.mode {
            BresenhamIteratorMode::Start => {
                self.mode = BresenhamIteratorMode::Path;
                Some(self.p1)
            }
            BresenhamIteratorMode::Path => {
                if self.p1 == self.p2 {
                    self.mode = BresenhamIteratorMode::Done;
                    None
                } else {
                    if self.error >= 0 && (self.error!=0 || self.iflag) {
                        self.error += self.derr1;
                        self.p1 += self.mod1;
                    }
                    self.error += self.derr2;
                    self.p1 += self.mod2;
                    Some(self.p1)
                }
            }
            BresenhamIteratorMode::Done => {
                None
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn rectangle_points(span:Rect2d, path:&mut Vec<Point2d>) -> usize {
    path.clear();

    let ll = span.bl.x;
    let rr = span.tr.x;
    let bb = span.bl.y;
    let tt = span.tr.y;

    let x = rr; for y in bb..tt         { path.push(Point2d::new(x  ,y  )); }
    let y = tt; for x in (ll..rr).rev() { path.push(Point2d::new(x+1,y  )); }
    let x = ll; for y in (bb..tt).rev() { path.push(Point2d::new(x  ,y+1)); }
    let y = bb; for x in ll..rr         { path.push(Point2d::new(x  ,y  )); }

    return path.len();
}

////////////////////////////////////////////////////////////////////////////////

pub fn circle_points(center:Point2d, radius:u32, path:&mut Vec<Point2d>, method:isize) -> usize {
    fn reflect (path:&mut Vec<Point2d>, center:Point2d, x:i32, y:i32) {
        path.push(center + Point2d::new( x,  y));
        path.push(center + Point2d::new( x, -y));
        path.push(center + Point2d::new(-x,  y));
        path.push(center + Point2d::new(-x, -y));
        path.push(center + Point2d::new( y,  x));
        path.push(center + Point2d::new( y, -x));
        path.push(center + Point2d::new(-y,  x));
        path.push(center + Point2d::new(-y, -x));
    };
    path.clear();

    if method<=2 {
        // naive, slow method
        let r2 = (radius*radius) as f64;
        let mut x = radius as i32;
        let mut y = 0;
        while y < x {
            reflect(path, center, x, y);
            y += 1;
            if method==0 {
                x = (r2 - ((y*y) as f64)).sqrt().round() as i32;
            } else if method==1 {
                x = (r2 - ((y*y) as f64)).sqrt().floor() as i32;
            } else if method==2 {
                x = (r2 - ((y*y) as f64)).sqrt().ceil() as i32;
            }
        }
        reflect(path, center, x, y);
    } else if method==3 {
        // decent approach by John Kennedy (paper at http://web.engr.oregonstate.edu/~sllu/bcircle.pdf)
        // so far the best looking approach...
        let r = radius as i32;
        let mut x = r;
        let mut y = 0;
        let mut xchange = 1 - 2*r;
        let mut ychange = 1;
        let mut radius_error = 0;
        while x >= y {
            reflect(path, center, x, y);
            y += 1;
            radius_error += ychange;
            ychange += 2;
            if 2*radius_error + xchange > 0 {
                x -= 1;
                radius_error += xchange;
                xchange += 2;
            }
        }
    } else if method==4 {
        let r = radius as i32;
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2*r;
        reflect(path, center, x, y);
        while y >= x {
            x += 1;
            if d>0 {
                y -= 1;
                d += 4*(x - y) + 10;
            } else {
                d += 4*x + 6;
            }
            reflect(path, center, x, y);
        }
    }

    return path.len();
}

////////////////////////////////////////////////////////////////////////////////

pub fn astar<
    Pos:Copy+Default+Eq+Hash,
    Weight:Add<Weight,Output=Weight>+Copy+Default+Eq+Ord,
    StepCostFn:FnMut(Pos,Pos)->Weight,
    EstimateFn:FnMut(Pos,Pos)->Weight,
    EdgesIterator:Iterator<Item=Pos>,
    BackEdgesIterator:Iterator<Item=Pos>,
    EdgesFn:FnMut(Pos)->EdgesIterator,
    BackEdgesFn:FnMut(Pos)->BackEdgesIterator,
> (
    start : Pos,
    goal: Pos,
    step_cost: &mut StepCostFn,
    edges: &mut EdgesFn,
    back_edges: &mut BackEdgesFn,
    estimate: &mut EstimateFn,
    result: &mut HashMap<Pos,Weight>,
    path: &mut Vec<Pos>
    ) -> bool
{
    result.clear();
    result.insert(start, Weight::default());
    let mut q = PriorityQueue::new();
    q.push(Weight::default(), start, ());
    while let Some((_,z,_)) = q.pop() {
        if z == goal {break;}
        let &r_z = result.get(&z).unwrap();
        for nz in edges(z) {
            let r_nz = r_z + step_cost(z, nz);
            let est_nz = r_nz + estimate(nz, goal);
            if let Some(&or_nz) = result.get(&nz) {
                if r_nz < or_nz {
                    result.insert(nz, r_nz);
                    q.push(est_nz, nz, ()); // TODO: should check if nz in q already &c.
                }
            } else {
                result.insert(nz, r_nz);
                q.push(est_nz, nz, ()); // TODO: should check if nz in q already &c.
            }
        }
    }
    path.clear();
    path.push(goal);
    let mut x = goal;
    let mut foundit = true;
    while x != start {
        let mut min_res = None;
        let mut px = x;
        for pz in back_edges(x) {
            if let Some(res_pz) = result.get(&pz) {
                match min_res {
                    Some(mr) => {
                        if res_pz < mr {
                            px = pz;
                            min_res = Some(res_pz);
                        }
                    }
                    None => {
                        px = pz;
                        min_res = Some(res_pz);
                    }
                }
            }
        }
        if min_res == None { foundit = false; break; } // got stuck!
        x = px;
        path.push(x);
    }
    path.reverse();
    return foundit;
}

////////////////////////////////////////////////////////////////////////////////
