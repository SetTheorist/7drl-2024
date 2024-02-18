////////////////////////////////////////////////////////////////////////////////

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy,Clone,Default,Eq,PartialEq,Ord,PartialOrd,Hash,Debug,serde::Serialize,serde::Deserialize)]
pub struct B64(pub i64);

impl B64 {
    pub fn new(a:i64, h:i64, m:i64, s:i64) -> Self {
        B64(a*60*60*60 + h*60*60 + m*60 + s*1)
    }
}

impl From<f64> for B64 {
    fn from(x:f64) -> B64 {
        let w = x as i64;
        let f = x - (w as f64);
        let h = ((f*60.0) as i64) % 60;
        let m = ((f*60.0*60.0) as i64) % 60;
        let s = ((f*60.0*60.0*60.0) as i64) % 60;
        B64::new(w, h, m, s)
    }
}

impl From<B64> for f64 {
    fn from(x:B64) -> f64 {
        let w = x.0 / (60*60*60);
        let f = x.0 - w*60*60*60;
        (w as f64) + (f as f64)/(60.0*60.0*60.0)
    }
}

impl Add<B64> for B64 { type Output = B64; fn add(self, other:B64) -> B64 { B64(self.0 + other.0) } }
impl Sub<B64> for B64 { type Output = B64; fn sub(self, other:B64) -> B64 { B64(self.0 - other.0) } }
impl Mul<B64> for B64 { type Output = B64; fn mul(self, other:B64) -> B64 { B64(self.0 * other.0 / (60*60*60)) } }
impl Div<B64> for B64 { type Output = B64; fn div(self, other:B64) -> B64 { B64((60*60*60) * self.0 / other.0) } }

impl AddAssign<B64> for B64 { fn add_assign(&mut self, other:B64) { self.0 = self.0 + other.0; } }
impl SubAssign<B64> for B64 { fn sub_assign(&mut self, other:B64) { self.0 = self.0 - other.0; } }
impl MulAssign<B64> for B64 { fn mul_assign(&mut self, other:B64) { self.0 = self.0 * other.0 / (60*60*60); } }
impl DivAssign<B64> for B64 { fn div_assign(&mut self, other:B64) { self.0 = self.0 / other.0 * (60*60*60); } }

//            0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
//U+1038x     𐎀   𐎁   𐎂   𐎃   𐎄   𐎅   𐎆   𐎇   𐎈   𐎉   𐎊   𐎋   𐎌   𐎍   𐎎   𐎏
//U+1039x     𐎐   𐎑   𐎒   𐎓   𐎔   𐎕   𐎖   𐎗   𐎘   𐎙   𐎚   𐎛   𐎜   𐎝       𐎟 
                     
impl ::std::fmt::Display for B64 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let n = {
            if self.0 < 0 {
                write!(f, "B-")?;
                //write!(f, "𐎖 ")?;
                -self.0
            } else {
                write!(f, "B+")?;
                //write!(f, "𐎈 ")?;
                self.0
            }
        };
        let w = n/(60*60*60);
        let h = (n/(   60*60)) % 60;
        let m = (n/(      60)) % 60;
        let s = (n/(       1)) % 60;

        if w == 0 {
            write!(f, "0")?;
        } else {
            let mut v = Vec::new();
            let mut w = w;
            while w > 0 {
                v.push(w % (60*60));
                w = w / (60*60);
            }
            v.reverse();
            write!(f, "{}", v[0])?;
            for x in &v[1..] {
                write!(f, ":{:04}", x)?;
                //write!(f, "𐎚{:04}", x)?;
            }
        }
        if h>0 || m>0 || s>0 {
            write!(f, ".{:02}", h)?;
            //write!(f, "𐎉{:02}", h)?;
        }
        if m>0 || s>0 {
            write!(f, "'{:02}", m)?;
            //write!(f, "𐎂{:02}", m)?;
        }
        if s>0 {
            write!(f, "\"{:02}", s)
            //write!(f, "𐎕{:02}", s)
        } else {
            write!(f, "")
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
