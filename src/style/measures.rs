use crate::style::css_values::*;
use ordered_float::NotNan;

#[derive(Clone, Debug)]
pub struct ExactLength {
    pub unit: Unit,
    pub value: NotNan<f64>,
}

#[derive(Clone, Debug)]
pub enum Unit {
    Px,
    Rem,
    Em,
    Cm,
    Vw,
    Vh,
}

pub fn px<T: Into<f64>>(val: T) -> ExactLength {
    ExactLength {
        value: NotNan::new(val.into()).unwrap(),
        unit: crate::style::measures::Unit::Px,
    }
}

pub fn vw<T: Into<f64>>(val: T) -> ExactLength {
    ExactLength {
        value: NotNan::new(val.into()).unwrap(),
        unit: Unit::Vw,
    }
}

pub fn vh<T: Into<f64>>(val: T) -> ExactLength {
    ExactLength {
        value: NotNan::new(val.into()).unwrap(),
        unit: Unit::Vh,
    }
}

pub fn cm<T: Into<f64>>(val: T) -> ExactLength {
    ExactLength {
        value: NotNan::new(val.into()).unwrap(),
        unit: Unit::Cm,
    }
}

pub fn rem<T: Into<f64>>(val: T) -> ExactLength {
    ExactLength {
        value: NotNan::new(val.into()).unwrap(),
        unit: Unit::Rem,
    }
}

pub fn em<T: Into<f64>>(val: T) -> ExactLength {
    ExactLength {
        value: NotNan::new(val.into()).unwrap(),
        unit: Unit::Em,
    }
}

pub fn hsl<H: Into<f64>,S: Into<f64>,L: Into<f64>>(h: H, s: S, l: L) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    CssColor::Hsl(h, s, l)
}

pub fn rgb<R: Into<f64>,G: Into<f64>,B: Into<f64>>(r: R, g: G, b: B) -> CssColor {
    let r = r.into();
    let g = g.into();
    let b = b.into();

    CssColor::Rgba(r, g, b, 1.0)
}

pub fn rgba<R: Into<f64>,G: Into<f64>,B: Into<f64>,A: Into<f64>>(r: R ,g: G, b: B, a: A) -> CssColor {
    let r = r.into();
    let g = g.into();
    let b = b.into();
    let a = a.into();
    CssColor::Rgba(r, g, b, a)
}

impl std::fmt::Display for ExactLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unit {
            Unit::Px => write!(f, "{}px", self.value),
            Unit::Rem => write!(f, "{}rem", self.value),
            Unit::Em => write!(f, "{}em", self.value),
            Unit::Cm => write!(f, "{}cm", self.value),
            Unit::Vw => write!(f, "{}vw", self.value),
            Unit::Vh => write!(f, "{}vh", self.value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Percent(pub f64);

pub fn pc<T: Into<f64>>(val: T) -> Percent {
    Percent(val.into())
}

impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}
