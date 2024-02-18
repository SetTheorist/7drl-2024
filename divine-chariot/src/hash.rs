////////////////////////////////////////////////////////////////////////////////

use std::mem::transmute;
use std::num::Wrapping as W;

use crate::point2d::*;
use crate::rect2d::*;

////////////////////////////////////////////////////////////////////////////////

const PRIME32_1 : W<u32> = W(2654435761);
const PRIME32_2 : W<u32> = W(2246822519);
const PRIME32_3 : W<u32> = W(3266489917);
const PRIME32_4 : W<u32> = W(668265263);
const PRIME32_5 : W<u32> = W(374761393);

////////////////////////////////////////////////////////////////////////////////

pub struct XXHash {
    seed: W<u32>,
}

#[inline]
fn u8_to_u32(buff:&[u8], index:usize) -> u32 {
    (buff[index] as u32) | ((buff[index+1] as u32) << 8)
        | ((buff[index+2] as u32) << 16) | ((buff[index+3] as u32) << 24)
}

#[inline]
fn subhash(value:W<u32>, read_value:W<u32>) -> W<u32> {
    let mut value = value;
    value += read_value * PRIME32_2;
    value = W(value.0.rotate_left(13));
    value *= PRIME32_1;
    return value;
}

#[inline]
fn subhash_arr(value:W<u32>, buff:&[u8], index:usize) -> W<u32> {
    let mut value = value;
    let read_value = W(u8_to_u32(buff, index));
    value += read_value * PRIME32_2;
    value = W(value.0.rotate_left(13));
    value *= PRIME32_1;
    return value;
}

impl XXHash {
    pub fn new(seed: u32) -> Self {
        let seed = W(seed);
        XXHash { seed }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct XXHash4 {
    sub: [XXHash; 4],
}

impl XXHash4 {
    pub fn new(seed:u32) -> Self {
        let h = XXHash::new(seed);
        let sub = [
            XXHash::new(h.hash32(0u32)),
            XXHash::new(h.hash32(1u32)),
            XXHash::new(h.hash32(2u32)),
            XXHash::new(h.hash32(3u32)),
            ];
        XXHash4 { sub }
    }
    /*
    pub fn new(seed:u128) -> Self {
        let sub = [
            XXHash::new((seed & 0xFFFFFFFF) as u32),
            XXHash::new(((seed>>32) & 0xFFFFFFFF) as u32),
            XXHash::new(((seed>>64) & 0xFFFFFFFF) as u32),
            XXHash::new(((seed>>96) & 0xFFFFFFFF) as u32),
            ];
        XXHash4 { sub }
    }
    */
}

////////////////////////////////////////////////////////////////////////////////

pub trait Hasher128<T> {
    fn hash128(&self, t:T) -> u128;
}

impl<T:Clone> Hasher128<T> for XXHash4
    where XXHash:Hasher32<T>
{
    fn hash128(&self, t:T) -> u128 {
        (self.sub[0].hash32(t.clone()) as u128) | ((self.sub[1].hash32(t.clone()) as u128)<<32) | ((self.sub[1].hash32(t.clone()) as u128)<<64) | ((self.sub[1].hash32(t.clone()) as u128)<<96)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Hasher32<T> {
    fn hash32(&self, t:T) -> u32;
}

////////////////////////////////////////

impl Hasher32<u32> for XXHash {
    fn hash32(&self, t:u32) -> u32 {
        let mut h32 = self.seed + PRIME32_5;
        h32 += W(4*1);

        h32 += W(t) * PRIME32_3;
        h32 = W(h32.0.rotate_left(17)) * PRIME32_4;

        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        return h32.0;
    }
}

impl Hasher32<(u32,u32)> for XXHash {
    fn hash32(&self, (t0,t1):(u32,u32)) -> u32 {
        let mut h32 = self.seed + PRIME32_5;
        h32 += W(4*2);

        h32 += W(t0) * PRIME32_3;
        h32 = W(h32.0.rotate_left(17)) * PRIME32_4;

        h32 += W(t1) * PRIME32_3;
        h32 = W(h32.0.rotate_left(17)) * PRIME32_4;

        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        return h32.0;
    }
}

impl Hasher32<(u32,u32,u32)> for XXHash {
    fn hash32(&self, (t0,t1,t2):(u32,u32,u32)) -> u32 {
        let mut h32 = self.seed + PRIME32_5;
        h32 += W(4*3);

        h32 += W(t0) * PRIME32_3;
        h32 = W(h32.0.rotate_left(17)) * PRIME32_4;

        h32 += W(t1) * PRIME32_3;
        h32 = W(h32.0.rotate_left(17)) * PRIME32_4;

        h32 += W(t2) * PRIME32_3;
        h32 = W(h32.0.rotate_left(17)) * PRIME32_4;

        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        return h32.0;
    }
}

impl Hasher32<(u32,u32,u32,u32)> for XXHash {
    fn hash32(&self, (t0,t1,t2,t3):(u32,u32,u32,u32)) -> u32 {
        let mut v1 = self.seed + PRIME32_1 + PRIME32_2;
        v1 = subhash(v1, W(t0));
        let mut v2 = self.seed + PRIME32_2;
        v2 = subhash(v2, W(t1));
        let mut v3 = self.seed;
        v3 = subhash(v3, W(t2));
        let mut v4 = self.seed - PRIME32_1;
        v4 = subhash(v4, W(t3));
        let mut h32 = W(v1.0.rotate_left(1)) + W(v2.0.rotate_left(7)) + W(v3.0.rotate_left(12)) + W(v4.0.rotate_left(18));

        h32 += W(4 * 4);

        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        return h32.0;
    }
}

impl Hasher32<u64> for XXHash {
    fn hash32(&self, t:u64) -> u32 {
        let t0 = (t & 0xFFFFFFFF) as u32;
        let t1 = ((t>>32) & 0xFFFFFFFF) as u32;
        self.hash32( (t0,t1) )
    }
}

////////////////////////////////////////

impl<'a> Hasher32<&'a [u32]> for XXHash {
    fn hash32(&self, t:&[u32]) -> u32 {
        let mut index = 0;
        let len = t.len();
        
        let mut h32 : W<u32>;
        if len >= 4 {
            let limit = len - 4;
            let mut v1 = self.seed + PRIME32_1 + PRIME32_2;
            let mut v2 = self.seed + PRIME32_2;
            let mut v3 = self.seed;
            let mut v4 = self.seed - PRIME32_1;
            
            loop {
                v1 = subhash(v1, W(t[index  ]));
                v2 = subhash(v2, W(t[index+1]));
                v3 = subhash(v3, W(t[index+2]));
                v4 = subhash(v4, W(t[index+3]));
                index += 4;

                if index > limit { break; }
            }
            h32 = W(v1.0.rotate_left(1)) + W(v2.0.rotate_left(7)) + W(v3.0.rotate_left(12)) + W(v4.0.rotate_left(18));
        } else {
            h32 = self.seed + PRIME32_5;
        }
        
        h32 += W((len * 4) as u32);
        
        while index < len {
            h32 += W(t[index]) * PRIME32_3;
            h32 = W(h32.0.rotate_left(17)) * PRIME32_4;
            index += 1;
        }
        
        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        
        return h32.0;
    }
}

impl<'a> Hasher32<&'a [u8]> for XXHash {
    fn hash32(&self, t:&[u8]) -> u32 {
        let mut index = 0;
        let len = t.len();
        
        let mut h32 : W<u32>;
        if len >= 16 {
            let limit = len - 16;
            let mut v1 = self.seed + PRIME32_1 + PRIME32_2;
            let mut v2 = self.seed + PRIME32_2;
            let mut v3 = self.seed;
            let mut v4 = self.seed - PRIME32_1;
            
            loop {
                v1 = subhash_arr(v1, t, index     );
                v2 = subhash_arr(v2, t, index +  4);
                v3 = subhash_arr(v3, t, index +  8);
                v4 = subhash_arr(v4, t, index + 12);
                index += 16;

                if index > limit { break; }
            }
            h32 = W(v1.0.rotate_left(1)) + W(v2.0.rotate_left(7)) + W(v3.0.rotate_left(12)) + W(v4.0.rotate_left(18));
        } else {
            h32 = self.seed + PRIME32_5;
        }
        
        h32 += W((len * 4) as u32);
        
        if len >= 4 {
            while index <= len - 4 {
                h32 += W(u8_to_u32(t, index)) * PRIME32_3;
                h32 = W(h32.0.rotate_left(17)) * PRIME32_4;
                index += 4;
            }
        }

        while index < len {
            h32 += W(t[index] as u32) * PRIME32_5;
            h32 = W(h32.0.rotate_left(17)) * PRIME32_1;
            index += 1;
        }
        
        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        
        return h32.0;
    }
}

////////////////////////////////////////

impl<'a> Hasher32<&'a [i32]> for XXHash {
    fn hash32(&self, t:&[i32]) -> u32 {
        let mut index = 0;
        let len = t.len();
        
        let mut h32 : W<u32>;
        if len >= 4 {
            let limit = len - 4;
            let mut v1 = self.seed + PRIME32_1 + PRIME32_2;
            let mut v2 = self.seed + PRIME32_2;
            let mut v3 = self.seed;
            let mut v4 = self.seed - PRIME32_1;
            
            loop {
                v1 = subhash(v1, W(unsafe{transmute(t[index  ])}));
                v2 = subhash(v2, W(unsafe{transmute(t[index+1])}));
                v3 = subhash(v3, W(unsafe{transmute(t[index+2])}));
                v4 = subhash(v4, W(unsafe{transmute(t[index+3])}));
                index += 4;

                if index > limit { break; }
            }
            h32 = W(v1.0.rotate_left(1)) + W(v2.0.rotate_left(7)) + W(v3.0.rotate_left(12)) + W(v4.0.rotate_left(18));
        } else {
            h32 = self.seed + PRIME32_5;
        }
        
        h32 += W((len * 4) as u32);
        
        while index < len {
            h32 += W(unsafe{transmute(t[index])}) * PRIME32_3;
            h32 = W(h32.0.rotate_left(17)) * PRIME32_4;
            index += 1;
        }
        
        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        
        return h32.0;
    }
}

impl Hasher32<i32> for XXHash {
    fn hash32(&self, t:i32) -> u32 {
        let t : u32 = unsafe { transmute(t) };
        self.hash32(t)
    }
}

impl Hasher32<i64> for XXHash {
    fn hash32(&self, t:i64) -> u32 {
        let t : u64 = unsafe { transmute(t) };
        self.hash32(t)
    }
}

////////////////////////////////////////

impl Hasher32<(i32,i32)> for XXHash {
    fn hash32(&self, (t0,t1):(i32,i32)) -> u32 {
        let t0 : u32 = unsafe { transmute(t0) };
        let t1 : u32 = unsafe { transmute(t1) };
        self.hash32((t0,t1))
    }
}

impl Hasher32<(i32,i32,i32)> for XXHash {
    fn hash32(&self, (t0,t1,t2):(i32,i32,i32)) -> u32 {
        let t0 : u32 = unsafe { transmute(t0) };
        let t1 : u32 = unsafe { transmute(t1) };
        let t2 : u32 = unsafe { transmute(t2) };
        self.hash32((t0,t1,t2))
    }
}

impl Hasher32<(i32,i32,i32,i32)> for XXHash {
    fn hash32(&self, (t0,t1,t2,t3):(i32,i32,i32,i32)) -> u32 {
        let t0 : u32 = unsafe { transmute(t0) };
        let t1 : u32 = unsafe { transmute(t1) };
        let t2 : u32 = unsafe { transmute(t2) };
        let t3 : u32 = unsafe { transmute(t3) };
        self.hash32((t0,t1,t2,t3))
    }
}

////////////////////////////////////////

impl Hasher32<Point2d> for XXHash {
    fn hash32(&self, t:Point2d) -> u32 {
        self.hash32((t.x,t.y))
    }
}

impl Hasher32<Rect2d> for XXHash {
    fn hash32(&self, t:Rect2d) -> u32 {
        self.hash32((t.bl.x,t.bl.y,t.tr.x,t.tr.y))
    }
}

impl<'a> Hasher32<&'a str> for XXHash {
    fn hash32(&self, t:&str) -> u32 {
        self.hash32(t.as_bytes())
    }
}

////////////////////////////////////////////////////////////////////////////////
