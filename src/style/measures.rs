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

pub fn hsl<T: Into<f64>>(h: T, s: T, l: T) -> CssColor {
    let h = h.into();
    let s = s.into();
    let l = l.into();
    CssColor::Hsl(h, s, l)
}

pub fn rgb(r: f64, g: f64, b: f64) -> CssColor {
    CssColor::Rgba(r, g, b, 1.0)
}

pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> CssColor {
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
