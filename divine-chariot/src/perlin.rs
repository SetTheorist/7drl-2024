////////////////////////////////////////////////////////////////////////////////

use crate::rng::{Generator, Rng};

////////////////////////////////////////////////////////////////////////////////

// Hash lookup table as defined by Ken Perlin.  This is a randomly
// arranged array of all numbers from 0-255 inclusive.
// Doubled permutation to avoid overflow
static STATIC_P : [usize; 512] = [
    151,160,137,91,90,15,
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
    //
    151,160,137,91,90,15,
    131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
    190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
    88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
    77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
    102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
    135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
    5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
    223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
    129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
    251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
    49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
    138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
];

/*
#[inline]
fn constrain(x:f64, a:f64, b:f64) -> f64 { x.max(a).min(b) }
*/

#[inline]
fn lerp(x:f64, a:f64, b:f64) -> f64 { a + x*(b - a) }

// Fade function as defined by Ken Perlin.  This eases coordinate values
// so that they will "ease" towards integral values.  This ends up smoothing
// the final output.
// 6t^5 - 15t^4 + 10t^3
#[inline]
fn fade(t:f64) -> f64 { t * t * t * (t * (t * 6.0 - 15.0) + 10.0) }

// Take the hashed value and take the first 4 bits of it (15 == 0b1111)
// If the most significant bit (MSB) of the hash is 0 then set u = x.  Otherwise y.
// In Ken Perlin's original implementation this was another conditional operator (?:).  I
// expanded it for readability.
// If the first and second significant bits are 0 set v = y
// If the first and second significant bits are 1 set v = x
// If the first and second significant bits are not equal (0/1, 1/0) set v = z
// Use the last 2 bits to decide if u and v are positive or negative.  Then return their addition.
fn grad(hash:usize, x:f64, y:f64, z:f64) -> f64 {
    let h = hash & 15;
    let u = if h<8 {x} else {y};
    let v =
        if h < 4 {
            y
        } else if (h == 12) || (h == 14) {
            x
        } else {
            z
        };
    (if (h&1)==0 {u} else {-u}) + (if (h&2)==0 {v} else {-v})
}

 

pub struct Perlin {
    repeat: Option<usize>,
    p: [usize; 512],
}

impl Perlin {
    pub fn new() -> Self {
        let repeat = None;
        let p = STATIC_P.clone();
        Self { repeat, p }
    }

    pub fn permute(&mut self, rng:&mut Rng) {
        for i in 0..255 {
            let x = rng.gen_range(i, 255);
            self.p.swap(i,     x);
            self.p.swap(i+256, x+256);
        }
    }

    pub fn set_repeat(&mut self, repeat:usize) {
        self.repeat = if repeat==0 {None} else {Some(repeat)};
    }

    pub fn octave_perlin(&self, x:f64, y:f64, z:f64, octaves:isize, persistence:f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        for _ in 0..octaves {
            total += self.perlin(x*frequency, y*frequency, z*frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= 2.0;

        }
        (total / max_value)
    }

    pub fn perlin(&self, x:f64, y:f64, z:f64) -> f64 {
        // TODO: this implementation is broken for negative coordinates.
        let x = x.abs(); let y = y.abs(); let z = z.abs();

        // If we have any repeat on, change the coordinates to their "local" repetitions
        let (x,y,z) = match self.repeat {
            Some(repeat) => { let r=repeat as f64; (x%r, y%r, z%r) }
            None => { (x,y,z) }
        };

        // Calculate the "unit cube" that the point asked will be located in
        // The left bound is ( |_x_|,|_y_|,|_z_| ) and the right bound is that
        // plus 1.  Next we calculate the location (from 0.0 to 1.0) in that cube.
        // We also fade the location to smooth the result.
        let calc = |v|{
            let vi = (v as usize) & 255;
            let vi2 = if let Some(repeat) = self.repeat {(vi+1)%repeat} else {vi+1};
            let vf = v - (v as isize as f64);
            let vv = fade(vf);
            (vi, vi2, vf, vv)
        };
        let (xi, xi2, xf, u) = calc(x);
        let (yi, yi2, yf, v) = calc(y);
        let (zi, zi2, zf, w) = calc(z);

        let p = &self.p;
        let ppp = |x,y,z|{p[p[p[x]+y]+z]};
        let aaa = ppp(xi , yi , zi );
        let aab = ppp(xi , yi , zi2);
        let aba = ppp(xi , yi2, zi );
        let abb = ppp(xi , yi2, zi2);
        let baa = ppp(xi2, yi , zi );
        let bab = ppp(xi2, yi , zi2);
        let bba = ppp(xi2, yi2, zi );
        let bbb = ppp(xi2, yi2, zi2);

        // The gradient function calculates the dot product between a pseudorandom
        // gradient vector and the vector from the input coordinate to the 8
        // surrounding points in its unit cube.
        // This is all then lerped together as a sort of weighted average based on the faded (u,v,w)
        // values we made earlier.
        let (xfm, yfm, zfm) = (xf-1.0, yf-1.0, zf-1.0);
        let x1 = lerp(u, grad(aaa, xf, yf , zf ), grad(baa, xfm, yf , zf ));
        let x2 = lerp(u, grad(aba, xf, yfm, zf ), grad(bba, xfm, yfm, zf ));
        let y1 = lerp(v, x1, x2);

        let x1 = lerp(u, grad(aab, xf, yf , zfm), grad(bab, xfm, yf , zfm));
        let x2 = lerp(u, grad(abb, xf, yfm, zfm), grad(bbb, xfm, yfm, zfm));
        let y2 = lerp(v, x1, x2);

        // For convenience we bound it to 0 - 1 (theoretical min/max before is -1 - 1)
        (lerp(w, y1, y2) + 1.0) / 2.0
    }
}

////////////////////////////////////////////////////////////////////////////////
