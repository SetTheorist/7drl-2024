////////////////////////////////////////////////////////////////////////////////

use std::mem::{transmute};
use std::ops::{Add, Mul, Sub};
use std::ops::{Range};

use crate::point2d::*;
use crate::rect2d::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Debug,Default,Eq,PartialEq,serde::Serialize,serde::Deserialize)]
pub struct Rnd {
    pub n : i32,
    pub d : i32,
    pub p : i32,
}

impl Rnd {
    pub fn new(n:i32, d:i32, p:i32) -> Self {
        Rnd { n, d, p }
    }
    #[inline]
    pub fn min(&self) -> i32 { self.n + self.p }
    #[inline]
    pub fn max(&self) -> i32 { self.n*self.d + self.p }
    #[inline]
    pub fn fave(&self) -> i32 { ((self.d+1)*self.n + self.p)/2 }
    #[inline]
    pub fn cave(&self) -> i32 { (((self.d+1)*self.n + self.p)+1)/2 }
    #[inline]
    pub fn ave(&self) -> f64 { ((self.d+1)*self.n + self.p) as f64 * 0.5 }
}

impl ::std::fmt::Display for Rnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.n != 0 {
            write!(f, "{}", self.n)?;
        } else {
            write!(f, "")?;
        }
        if self.p != 0 {
            write!(f, "d{}", self.d)
        } else {
            write!(f, "d{}{:+}", self.d, self.p)
        }

    }
}

impl Sampler<Rnd> for Rng {
    type Output = i32;

    #[inline]
    fn sample(&mut self, r:Rnd) -> i32 {
        let mut sum = r.p;
        for _ in 0 .. r.n {
            sum += self.gen_range(1,r.d);
        }
        sum
    }
}



////////////////////////////////////////////////////////////////////////////////

// A Xorshift[1] random number generator.
//
// Copied from rand crate.
//
// [1]: Marsaglia, George (July 2003). ["Xorshift RNGs"](http://www.jstatsoft.org/v08/i14/paper).
// *Journal of Statistical Software*. Vol. 8 (Issue 14).
//
#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Rng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rng {
    pub fn from_seed(s:[u32; 4]) -> Self {
        let [x,y,z,w] = s;
        Rng { x, y, z, w }
    }

    pub fn seed(&self) -> [u32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w = self.w;
        self.w = w ^ (w >> 19) ^ (t ^ (t >> 8));
        self.w
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64)<<32) | (self.next_u32() as u64)
    }

}

impl Default for Rng {
    fn default() -> Self {
        Rng::from_seed([ 0x193a6754, 0xa8a7d469, 0x97830e05, 0x113ba7bb ])
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Generator<T> {
    fn gen(&mut self) -> T;
    fn gen_range(&mut self, a:T, b:T) -> T;
}

impl Generator<bool> for Rng {
    fn gen(&mut self) -> bool {
        (self.next_u32() & 0x01) == 0
    }
    fn gen_range(&mut self, a:bool, b:bool) -> bool {
        if a==b { a } else { self.gen() }
    }
}

impl Generator<u8> for Rng {
    fn gen(&mut self) -> u8 {
        (self.next_u32() & 0xFF) as u8
    }
    fn gen_range(&mut self, a:u8, b:u8) -> u8 {
        let x : u8 = self.gen();
        a + (x % (b-a))
    }
}

impl Generator<u16> for Rng {
    fn gen(&mut self) -> u16 {
        (self.next_u32() & 0xFFFF) as u16
    }
    fn gen_range(&mut self, a:u16, b:u16) -> u16 {
        let x : u16 = self.gen();
        a + (x % (b-a))
    }
}

impl Generator<u32> for Rng {
    fn gen(&mut self) -> u32 {
        self.next_u32()
    }
    fn gen_range(&mut self, a:u32, b:u32) -> u32 {
        let x : u32 = self.gen();
        a + (x % (b-a))
    }
}

impl Generator<u64> for Rng {
    fn gen(&mut self) -> u64 {
        self.next_u64()
    }
    fn gen_range(&mut self, a:u64, b:u64) -> u64 {
        let x : u64 = self.gen();
        a + (x % (b-a))
    }
}

impl Generator<usize> for Rng {
    fn gen(&mut self) -> usize {
        self.next_u64() as usize
    }
    fn gen_range(&mut self, a:usize, b:usize) -> usize {
        let x : usize = self.gen();
        a + (x % (b-a))
    }
}

impl Generator<i8> for Rng {
    fn gen(&mut self) -> i8 {
        unsafe { transmute((self.next_u32() & 0xFF) as u8) }
    }
    fn gen_range(&mut self, a:i8, b:i8) -> i8 {
        let x : u8 = self.gen();
        a + ((x % (b-a) as u8) as i8)
    }
}

impl Generator<i16> for Rng {
    fn gen(&mut self) -> i16 {
        unsafe { transmute((self.next_u32() & 0xFFFF) as u16) }
    }
    fn gen_range(&mut self, a:i16, b:i16) -> i16 {
        let x : u16 = self.gen();
        a + ((x % (b-a) as u16) as i16)
    }
}

impl Generator<i32> for Rng {
    fn gen(&mut self) -> i32 {
        unsafe { transmute(self.next_u32() as u32) }
    }
    fn gen_range(&mut self, a:i32, b:i32) -> i32 {
        let x : u32 = self.gen();
        a + ((x % (b-a) as u32) as i32)
    }
}

impl Generator<i64> for Rng {
    fn gen(&mut self) -> i64 {
        unsafe { transmute(self.next_u64()) }
    }
    fn gen_range(&mut self, a:i64, b:i64) -> i64 {
        let x : u64 = self.gen();
        a + ((x % (b-a) as u64) as i64)
    }
}

impl Generator<isize> for Rng {
    fn gen(&mut self) -> isize {
        unsafe { transmute(self.next_u64() as usize) }
    }
    fn gen_range(&mut self, a:isize, b:isize) -> isize {
        let x : usize = self.gen();
        a + ((x % (b-a) as usize) as isize)
    }
}

// TODO: this is quick and dirty hack (but not unreasonable)
impl Generator<f32> for Rng {
    fn gen(&mut self) -> f32 {
        (self.next_u32() as f32) / 4294967296.0
    }
    fn gen_range(&mut self, a:f32, b:f32) -> f32 {
        let x : f32 = self.gen();
        a + (x * (b-a))
    }
}

// TODO: this is quick and dirty hack (but not unreasonable)
impl Generator<f64> for Rng {
    fn gen(&mut self) -> f64 {
        (self.next_u64() as f64) / 18446744073709551616.0
    }
    fn gen_range(&mut self, a:f64, b:f64) -> f64 {
        let x : f64 = self.gen();
        a + (x * (b-a))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Sampler<T> {
    type Output;
    fn sample(&mut self, t:T) -> Self::Output;
}

impl Sampler<Rect2d> for Rng {
    type Output = Point2d;

    fn sample(&mut self, t:Rect2d) -> Point2d {
        Point2d::new(self.gen_range(t.bl.x, t.tr.x), self.gen_range(t.bl.y, t.tr.y))
    }
}

impl<T> Sampler<Range<T>> for Rng
    where Rng:Generator<T>
{
    type Output = T;

    fn sample(&mut self, t:Range<T>) -> T {
        self.gen_range(t.start, t.end)
    }
}

impl<'a,T> Sampler<&'a[T]> for Rng {
    type Output = &'a T;

    fn sample(&mut self, t:&'a[T]) -> &'a T {
        &t[self.gen_range(0, t.len())]
    }
}

impl<'a,T> Sampler<&'a Vec<T>> for Rng {
    type Output = &'a T;

    fn sample(&mut self, t:&'a Vec<T>) -> &'a T {
        self.sample(&t[..])
    }
}

////////////////////////////////////////////////////////////////////////////////

// consumes an iterator of unknown (but finite) length and returns a single random sample from it
// note that this is generally quite inefficient, as it scans the entire length of the iteration...
pub fn reservoir_sample<T,I>(rng:&mut Rng, mut it:I) -> Option<T>
    where I:Iterator<Item=T>
{
    let first = it.next();
    if first.is_none() { return None; }
    let mut t = first.unwrap();
    let mut n : f64 = 1.0;
    while let Some(tt) = it.next() {
        n += 1.0;
        let x : f64 = rng.gen();
        if x < 1.0/n { t = tt; }
    }
    Some(t)
}


////////////////////////////////////////////////////////////////////////////////
