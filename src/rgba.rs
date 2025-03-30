#![allow(dead_code)]  // Just for now

#[derive(Debug,Clone)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b:f32,
    pub a:f32
}

impl RGBA {
    pub fn red()   -> RGBA { return RGBA{ r: 1.0, g: 0.0, b: 0.0, a: 1.0 }; }
    pub fn green() -> RGBA { return RGBA{ r: 0.0, g: 1.0, b: 0.0, a: 1.0 }; }
    pub fn blue()  -> RGBA { return RGBA{ r: 0.0, g: 0.0, b: 1.0, a: 1.0 }; }
    pub fn black() -> RGBA { return RGBA{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 }; }
    pub fn white() -> RGBA { return RGBA{ r: 1.0, g: 1.0, b: 1.0, a: 1.0 }; }

    pub fn lerp(one: &RGBA, two: &RGBA, coeff: f32) -> RGBA {
        return coeff * one + (1.0 - coeff) * two;
    }

    pub fn gamma_correct(&mut self) {
        let gamma = 2.2f32;
        self.r = self.r.powf(1.0 / gamma);
        self.g = self.g.powf(1.0 / gamma);
        self.b = self.b.powf(1.0 / gamma);
    }
}

impl std::ops::Add<&RGBA> for &RGBA {
    type Output = RGBA;
    fn add(self, other: &RGBA) -> RGBA {
        return RGBA {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        };
    }
}
impl std::ops::Add<RGBA> for RGBA {
    type Output = RGBA;
    fn add(self, other: RGBA) -> RGBA {
        return RGBA {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        };
    }
}
impl std::ops::Mul<&RGBA> for f32 {
    type Output = RGBA;
    fn mul(self, val: &RGBA) -> RGBA {
        return RGBA {
            r: self * val.r,
            g: self * val.g,
            b: self * val.b,
            a: self * val.a,
        };
    }
}
