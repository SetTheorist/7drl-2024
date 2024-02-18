////////////////////////////////////////////////////////////////////////////////
#![allow(unused_variables)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct Color(pub u32);

#[allow(non_upper_case_globals)]
impl Color {
    #[inline]
    pub fn rgba(r:u8, g:u8, b:u8, a:u8) -> Self {
        Color(((a as u32)<<24)|((r as u32)<<16)|((g as u32)<< 8)|((b as u32)<< 0))
    }
    #[inline]
    pub fn rgb(r:u8, g:u8, b:u8) -> Self { Color::rgba(r,g,b,0xFF) }
    #[inline]
    pub fn a(&self) -> u8 { ((self.0>>24)&0xFF) as u8 }
    #[inline]
    pub fn r(&self) -> u8 { ((self.0>>16)&0xFF) as u8 }
    #[inline]
    pub fn g(&self) -> u8 { ((self.0>> 8)&0xFF) as u8 }
    #[inline]
    pub fn b(&self) -> u8 { ((self.0>> 0)&0xFF) as u8 }

    #[inline]
    pub fn lint(&self, c:&Color, w:f64) -> Color {
        let w = w.min(1.0).max(0.0);
        Color::rgba(
            (((self.r() as f64)*w + (c.r() as f64)*(1.0-w)).round().min(255.0).max(0.0) as u8),
            (((self.g() as f64)*w + (c.g() as f64)*(1.0-w)).round().min(255.0).max(0.0) as u8),
            (((self.b() as f64)*w + (c.b() as f64)*(1.0-w)).round().min(255.0).max(0.0) as u8),
            (((self.a() as f64)*w + (c.a() as f64)*(1.0-w)).round().min(255.0).max(0.0) as u8))
    }

    #[inline]
    pub fn scale(&self, w:f64) -> Color {
        Color::rgba(
            (((self.r() as f64)*w).round().min(255.0).max(0.0) as u8),
            (((self.g() as f64)*w).round().min(255.0).max(0.0) as u8),
            (((self.b() as f64)*w).round().min(255.0).max(0.0) as u8),
            (((self.a() as f64)*w).round().min(255.0).max(0.0) as u8))
    }

    #[inline]
    pub fn blt(&self) -> bear_lib_terminal::Color {
        bear_lib_terminal::Color::from_rgba(self.r(), self.g(), self.b(), self.a())
    }

    pub const alice_blue : Color = Color(0xFFF0F8FF);
    #[inline] pub fn antique_white() -> Color { Color::rgb(0xFA,0xEB,0xD7) }
    #[inline] pub fn azure_mist() -> Color { Color::rgb(0xF0,0xFF,0xFF) }
    #[inline] pub fn black() -> Color { Color::rgb(0x00,0x00,0x00) }
    #[inline] pub fn blood() -> Color { Color::rgb(0x8A,0x03,0x03 ) }
    #[inline] pub fn blue() -> Color { Color::rgb(0x00,0x00,0xDF) }
    #[inline] pub fn blue_sapphire() -> Color { Color::rgb(0x12,0x61,0x80) }
    #[inline] pub fn bronze_metallic() -> Color { Color::rgb(0xB0,0x8D,0x57) }
    #[inline] pub fn brown() -> Color { Color::rgb(0x80,0x40,0x40) }
    #[inline] pub fn cadmium_blue() -> Color { Color::rgb(0x0A,0x11,0x95) }
    #[inline] pub fn carnation_pink() -> Color { Color::rgb(0xFF,0xA6,0xC9) }
    #[inline] pub fn cerulean_frost() -> Color { Color::rgb(0x6D,0x9B,0xC3) }
    #[inline] pub fn classic_rose() -> Color { Color::rgb(0xFB,0xCC,0xE7) }
    #[inline] pub fn cobalt_blue() -> Color { Color::rgb(0x00,0x47,0xAB) }
    #[inline] pub fn dark_blue() -> Color { Color::rgb(0x00,0x00,0x1F) }
    #[inline] pub fn dark_brown() -> Color { Color::rgb(0x20,0x10,0x10) }
    #[inline] pub fn dark_green() -> Color { Color::rgb(0x00,0x1F,0x00) }
    #[inline] pub fn dark_grey() -> Color { Color::rgb(0x1F,0x1F,0x1F) }
    #[inline] pub fn dark_saffron() -> Color { Color::saffron().scale(0.5) }
    #[inline] pub fn desert_sand() -> Color { Color::rgb(0xED,0xC9,0xAF) }
    #[inline] pub fn emerald_green() -> Color { Color::rgb(0x04,0x63,0x07) }
    #[inline] pub fn english_lavender() -> Color { Color::rgb(0xB4,0x83,0x95) }
    #[inline] pub fn forest_green() -> Color { Color::rgb(0x5F,0xA7,0x77) }
    #[inline] pub fn gold() -> Color { Color::rgb(0xA5,0x7C,0x00) }
    #[inline] pub fn granite_gray() -> Color { Color::rgb(0x67,0x67,0x67) }
    #[inline] pub fn green() -> Color { Color::rgb(0x00,0xDF,0x00) }
    #[inline] pub fn grey() -> Color { Color::rgb(0x6F,0x6F,0x6F) }
    #[inline] pub fn honolulu_blue() -> Color { Color::rgb(0x00,0x6D,0xB0) }
    #[inline] pub fn iron() -> Color { Color::rgb(0xA1,0x9D,0x94) }
    #[inline] pub fn jade() -> Color { Color::rgb(0x00,0xA8,0x6B) }
    #[inline] pub fn lapis_lazuli() -> Color { Color::rgb(0x26,0x61,0x9C) }
    #[inline] pub fn lavender_blue() -> Color { Color::rgb(0xCC,0xCC,0xFF) }
    #[inline] pub fn lavender_blush() -> Color { Color::rgb(0xFF,0xF0,0xF5) }
    #[inline] pub fn lavender_mist() -> Color { Color::rgb(0xE6,0xE6,0xFA) }
    #[inline] pub fn lilac() -> Color { Color::rgb(0xC8,0xA2,0xC8) }
    #[inline] pub fn malachite() -> Color { Color::rgb(0x0B,0xDA,0x51) }
    #[inline] pub fn mauve() -> Color { Color::rgb(0xE0,0xB0,0xFF) }
    #[inline] pub fn maximum_blue() -> Color { Color::rgb(0x47,0xAB,0xCC) }
    #[inline] pub fn maximum_green() -> Color { Color::rgb(0x5E,0x8C,0x31) }
    #[inline] pub fn maximum_red() -> Color { Color::rgb(0xD9,0x21,0x21) }
    #[inline] pub fn milk() -> Color { Color::rgb(0xFD,0xFF,0xF5) }
    #[inline] pub fn mint() -> Color { Color::rgb(0x3E,0xB4,0x89) }
    #[inline] pub fn misty_rose() -> Color { Color::rgb(0xFF,0xE4,0xE1) }
    #[inline] pub fn mistyrose() -> Color { Color::rgb(0xFF,0xE4,0xE1) }
    #[inline] pub fn navajo_white() -> Color { Color::rgb(0xFF,0xDE,0xAD) }
    #[inline] pub fn onyx() -> Color { Color::rgb(0x35,0x38,0x39) }
    #[inline] pub fn opal() -> Color { Color::rgb(0xA8,0xC3,0xBC) }
    #[inline] pub fn pale_lavender() -> Color { Color::rgb(0xDC,0xD0,0xFF) }
    #[inline] pub fn pale_pink() -> Color { Color::rgb(0xFA,0xDA,0xDD) }
    #[inline] pub fn pale_plum() -> Color { Color::rgb(0xDD,0xA0,0xDD) }
    #[inline] pub fn papaya_whip() -> Color { Color::rgb(0xFF,0xEF,0xD5) }
    #[inline] pub fn parakeet_blue() -> Color { Color::rgb(0x7E,0xB6,0xFF) }
    #[inline] pub fn pearl() -> Color { Color::rgb(0xEA,0xE0,0xC8) }
    #[inline] pub fn persian_blue() -> Color { Color::rgb(0x1C,0x39,0xBB) }
    #[inline] pub fn pistachio() -> Color { Color::rgb(0x93,0xC5,0x72) }
    #[inline] pub fn prussian_blue() -> Color { Color::rgb(0x00,0x31,0x53) }
    #[inline] pub fn pure_blue() -> Color { Color::rgb(0x00,0x00,0xFF) }
    #[inline] pub fn pure_green() -> Color { Color::rgb(0x00,0xFF,0x00) }
    #[inline] pub fn pure_red() -> Color { Color::rgb(0xFF,0x00,0x00) }
    #[inline] pub fn quartz() -> Color { Color::rgb(0x51,0x48,0x4F) }
    #[inline] pub fn rose_garnet() -> Color { Color::rgb(0x96,0x01,0x45) }
    #[inline] pub fn rose_quartz() -> Color { Color::rgb(0xAA,0x98,0xA9) }
    #[inline] pub fn royal_purple() -> Color { Color::rgb(0x78,0x51,0xA9) }
    #[inline] pub fn saffron() -> Color { Color::rgb(0xF4,0xC4,0x30) }
    #[inline] pub fn sage() -> Color { Color::rgb(0xBC,0xB8,0x8A) }
    #[inline] pub fn sandstorm() -> Color { Color::rgb(0xEC,0xD5,0x40) }
    #[inline] pub fn sapphire() -> Color { Color::rgb(0x0F,0x52,0xBA) }
    #[inline] pub fn sea_blue() -> Color { Color::rgb(0x00,0x69,0x94) }
    #[inline] pub fn smoke() -> Color { Color::rgb(0x73,0x82,0x76) }
    #[inline] pub fn smokey_topaz() -> Color { Color::rgb(0x83,0x2A,0x0D) }
    #[inline] pub fn smoky_topaz() -> Color { Color::rgb(0x93,0x3D,0x41) }
    #[inline] pub fn snow() -> Color { Color::rgb(0xFF,0xFA,0xFA) }
    #[inline] pub fn tangelo() -> Color { Color::rgb(0xF9,0x4D,0x00) }
    #[inline] pub fn teal_blue() -> Color { Color::rgb(0x36,0x75,0x88) }
    #[inline] pub fn tea_rose() -> Color { Color::rgb(0xF4,0xC2,0xC2) }
    #[inline] pub fn tea_rose2() -> Color { Color::rgb(0xF8,0x83,0x79) }
    #[inline] pub fn thistle() -> Color { Color::rgb(0xD8,0xBF,0xD8) }
    #[inline] pub fn thulian_pink() -> Color { Color::rgb(0xDE,0x6F,0xA1) }
    #[inline] pub fn titanium() -> Color { Color::rgb(0x87,0x86,0x81) }
    #[inline] pub fn tuscan() -> Color { Color::rgb(0xFA,0xD6,0xA5) }
    #[inline] pub fn twilight_lavender() -> Color { Color::rgb(0x8A,0x49,0x6B) }
    #[inline] pub fn vanilla() -> Color { Color::rgb(0xF3,0xE5,0xAB) }
    #[inline] pub fn vodka() -> Color { Color::rgb(0xBF,0xC0,0xEE) }
    #[inline] pub fn white_chocolate() -> Color { Color::rgb(0xED,0xE6,0xD6) }
    #[inline] pub fn white() -> Color { Color::rgb(0xFF,0xFF,0xFF) }
}

impl ::std::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Cr{:02X}g{:02X}b{:02X}a{:02X}",
            self.r(), self.g(), self.b(), self.a())
    }
}

#[derive(Clone,Copy,Default,Eq,PartialEq)]
pub struct Glyph {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Glyph {
    pub fn new(ch:char, fg:Color, bg:Color) -> Self {
        Glyph { ch, fg, bg }
    }
}

impl ::std::fmt::Debug for Glyph {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Glyph[{:?}/{:?}/{:?}]", self.ch, self.fg, self.bg)
    }
}

////////////////////////////////////////////////////////////////////////////////
