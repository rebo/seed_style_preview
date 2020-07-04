use crate::style::css_values::*;
use crate::style::ReturnBpScale;
use crate::style::ReturnBpTuple;
use crate::style::{CssValueTrait, Rule, Style, UpdateStyle};
use anymap::any::Any;
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style_macros::generate_froms;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
pub trait BorderTheme: Eq + Hash + Clone {}
pub trait BorderWidthTheme: Eq + Hash + Clone {}
pub trait BorderStyleTheme: Eq + Hash + Clone {}
pub trait SpaceTheme: Eq + Hash + Clone {}
pub trait LineHeightTheme: Eq + Hash + Clone {}
pub trait LetterSpacingTheme: Eq + Hash + Clone {}
pub trait BorderRadiusTheme: Eq + Hash + Clone {}
pub trait FontTheme: Eq + Hash + Clone {}
pub trait FontSizeTheme: Eq + Hash + Clone {}
pub trait SizeTheme: Eq + Hash + Clone {}
pub trait TransitionTheme: Eq + Hash + Clone {}
pub trait ZIndexTheme: Eq + Hash + Clone {}
pub trait DisplayTheme: Eq + Hash + Clone {}
pub trait ColorTheme: Eq + Hash + Clone {}
pub trait ShadowTheme: Eq + Hash + Clone {}
pub trait StyleTheme: Eq + Hash + Clone {}
pub trait BreakpointTheme: Eq + Hash + Clone {}

thread_local! {
    static THEMES_VEC : RefCell<Vec<Theme>> = RefCell::new(vec![]);
}


pub fn change_theme_with_name(name: &str, theme: Theme) {
    app_themes().update( |v|
        if let Some(existing_theme) = v.iter_mut().find(|t| &t.name == name) {
            let _old_theme = std::mem::replace(existing_theme, theme);
        } else {
            panic!("old theme doesnt exist");
        }
    )
}

impl From<CssSize> for CssWidth {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSize> for CssHeight {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSize> for CssMinHeight {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSize> for CssMinWidth {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSize> for CssMaxWidth {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSize> for CssMaxHeight {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSize> for CssFlexBasis {
    fn from(val: CssSize) -> Self {
        match val {
            CssSize::Auto => Self::Auto,
            CssSize::Length(val) => Self::Length(val),
            CssSize::Percentage(val) => Self::Percentage(val),
            CssSize::Initial => Self::Initial,
            CssSize::Inherit => Self::Inherit,
            CssSize::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderRadius> for CssBorderTopRightRadius {
    fn from(val: CssBorderRadius) -> Self {
        match val {
            CssBorderRadius::Length(val) => Self::Length(val),
            CssBorderRadius::Percentage(val) => Self::Percentage(val),
            CssBorderRadius::Initial => Self::Initial,
            CssBorderRadius::Inherit => Self::Inherit,
            CssBorderRadius::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderRadius> for CssBorderTopLeftRadius {
    fn from(val: CssBorderRadius) -> Self {
        match val {
            CssBorderRadius::Length(val) => Self::Length(val),
            CssBorderRadius::Percentage(val) => Self::Percentage(val),
            CssBorderRadius::Initial => Self::Initial,
            CssBorderRadius::Inherit => Self::Inherit,
            CssBorderRadius::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderRadius> for CssBorderBottomRightRadius {
    fn from(val: CssBorderRadius) -> Self {
        match val {
            CssBorderRadius::Length(val) => Self::Length(val),
            CssBorderRadius::Percentage(val) => Self::Percentage(val),
            CssBorderRadius::Initial => Self::Initial,
            CssBorderRadius::Inherit => Self::Inherit,
            CssBorderRadius::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderRadius> for CssBorderBottomLeftRadius {
    fn from(val: CssBorderRadius) -> Self {
        match val {
            CssBorderRadius::Length(val) => Self::Length(val),
            CssBorderRadius::Percentage(val) => Self::Percentage(val),
            CssBorderRadius::Initial => Self::Initial,
            CssBorderRadius::Inherit => Self::Inherit,
            CssBorderRadius::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssPadding {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssPaddingLeft {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssPaddingRight {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssPaddingTop {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssPaddingBottom {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssMargin {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssMarginLeft {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssMarginRight {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssMarginTop {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssMarginBottom {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

// impl From<CssSpace> for CssTop {
//     fn from(val: CssSpace) -> Self {
//         match val {
//             CssSpace::Auto => Self::Auto,
//             CssSpace::Length(val) => Self::Length(val),
//             CssSpace::Percentage(val) => Self::Percentage(val),
//             CssSpace::Inherit => Self::Inherit,
//             CssSpace::StringValue(val) => Self::StringValue(val),
//         }
//     }
// }

// impl From<CssSpace> for CssBottom {
//     fn from(val: CssSpace) -> Self {
//         match val {
//             CssSpace::Auto => Self::Auto,
//             CssSpace::Length(val) => Self::Length(val),
//             CssSpace::Percentage(val) => Self::Percentage(val),
//             CssSpace::Inherit => Self::Inherit,
//             CssSpace::StringValue(val) => Self::StringValue(val),
//         }
//     }
// }

// impl From<CssSpace> for CssLeft {
//     fn from(val: CssSpace) -> Self {
//         match val {
//             CssSpace::Auto => Self::Auto,
//             CssSpace::Length(val) => Self::Length(val),
//             CssSpace::Percentage(val) => Self::Percentage(val),
//             CssSpace::Inherit => Self::Inherit,
//             CssSpace::StringValue(val) => Self::StringValue(val),
//         }
//     }
// }

// impl From<CssSpace> for CssRight {
//     fn from(val: CssSpace) -> Self {
//         match val {
//             CssSpace::Auto => Self::Auto,
//             CssSpace::Length(val) => Self::Length(val),
//             CssSpace::Percentage(val) => Self::Percentage(val),
//             CssSpace::Inherit => Self::Inherit,
//             CssSpace::StringValue(val) => Self::StringValue(val),
//         }
//     }
// }

impl From<CssSpace> for CssGridGap {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssGridColumnGap {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssGridRowGap {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssGap {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssRowGap {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssSpace> for CssColumnGap {
    fn from(val: CssSpace) -> Self {
        match val {
            CssSpace::Auto => Self::Auto,
            CssSpace::Length(val) => Self::Length(val),
            CssSpace::Percentage(val) => Self::Percentage(val),
            CssSpace::Inherit => Self::Inherit,
            CssSpace::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssBorderBottomStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssBorderTopStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssBorderRightStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssBorderLeftStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssOutlineStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssOutlineBottomStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssOutlineTopStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssOutlineRightStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderStyle> for CssOutlineLeftStyle {
    fn from(val: CssBorderStyle) -> Self {
        match val {
            CssBorderStyle::None => Self::None,
            CssBorderStyle::Hidden => Self::Hidden,
            CssBorderStyle::Dotted => Self::Dotted,
            CssBorderStyle::Dashed => Self::Dashed,
            CssBorderStyle::Solid => Self::Solid,
            CssBorderStyle::Double => Self::Double,
            CssBorderStyle::Groove => Self::Groove,
            CssBorderStyle::Ridge => Self::Ridge,
            CssBorderStyle::Inset => Self::Inset,
            CssBorderStyle::Outset => Self::Outset,
            CssBorderStyle::Initial => Self::Initial,
            CssBorderStyle::Inherit => Self::Inherit,
            CssBorderStyle::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssColor> for CssBackgroundColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssFill {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssStroke {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssBorderColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssBorderLeftColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssBorderRightColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssBorderBottomColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssBorderTopColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssOutlineColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssOutlineLeftColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssOutlineRightColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssOutlineBottomColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssColor> for CssOutlineTopColor {
    fn from(val: CssColor) -> Self {
        match val {
            CssColor::Rgba(r, g, b, a) => Self::Rgba(r, g, b, a),
            CssColor::Hsl(h, s, l) => Self::Hsl(h, s, l),
            CssColor::Hex(h) => Self::Hex(h),
            CssColor::StringValue(val) => Self::StringValue(val),
            CssColor::Inherit => Self::Inherit,
        }
    }
}

impl From<CssBorder> for CssOutline {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Outline(a.into(), b.into(), c.into()),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssBorderLeft {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Border(a, b, c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssBorderRight {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Border(a, b, c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssBorderTop {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Border(a, b, c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssBorderBottom {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Border(a, b, c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorderWidth> for CssBorderBottomWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssBorderTopWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssBorderLeftWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssBorderRightWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorder> for CssOutlineLeft {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Outline(a.into(), b.into(), c.into()),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssOutlineRight {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Outline(a.into(), b.into(), c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssOutlineTop {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Outline(a.into(), b.into(), c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorder> for CssOutlineBottom {
    fn from(val: CssBorder) -> Self {
        match val {
            CssBorder::Border(a, b, c) => Self::Outline(a.into(), b.into(), c),
            CssBorder::Inherit => Self::Inherit,
            CssBorder::StringValue(value) => Self::StringValue(value),
        }
    }
}

impl From<CssBorderWidth> for CssOutlineBottomWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssOutlineTopWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssOutlineLeftWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssOutlineRightWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssBorderWidth> for CssOutlineWidth {
    fn from(val: CssBorderWidth) -> Self {
        match val {
            CssBorderWidth::Medium => Self::Medium,
            CssBorderWidth::Thin => Self::Thin,
            CssBorderWidth::Thick => Self::Thick,
            CssBorderWidth::Length(val) => Self::Length(val),
            CssBorderWidth::Initial => Self::Initial,
            CssBorderWidth::Inherit => Self::Inherit,
            CssBorderWidth::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssShadow> for CssTextShadow {
    fn from(val: CssShadow) -> Self {
        match val {
            CssShadow::Shadow(h, v, b, color) => Self::Shadow(h, v, b, color),
            CssShadow::None => Self::None,
            CssShadow::Initial => Self::Initial,
            CssShadow::Inherit => Self::Inherit,
            CssShadow::StringValue(val) => Self::StringValue(val),
        }
    }
}

impl From<CssShadow> for CssBoxShadow {
    fn from(val: CssShadow) -> Self {
        match val {
            CssShadow::Shadow(h, v, b, color) => Self::Shadow(h, v, b, color),
            CssShadow::None => Self::None,
            CssShadow::Initial => Self::Initial,
            CssShadow::Inherit => Self::Inherit,
            CssShadow::StringValue(val) => Self::StringValue(val),
        }
    }
}

struct ReturnThemeValFromUsize<T: CssValueTrait>(usize, PhantomData<T>);

generate_froms!([
    (
        "FontSizeTheme",
        "CssFontSize",
        "CssFontSize",
        "font_sizes_scale"
    ),
    ("SizeTheme", "CssSize", "CssWidth", "sizes_scale"),
    ("SizeTheme", "CssSize", "CssMinWidth", "sizes_scale"),
    ("SizeTheme", "CssSize", "CssMaxWidth", "sizes_scale"),
    ("SizeTheme", "CssSize", "CssHeight", "sizes_scale"),
    ("SizeTheme", "CssSize", "CssMinHeight", "sizes_scale"),
    ("SizeTheme", "CssSize", "CssMaxHeight", "sizes_scale"),
    ("SpaceTheme", "CssSpace", "CssPadding", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssPaddingLeft", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssPaddingRight", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssPaddingTop", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssPaddingBottom", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssMargin", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssMarginLeft", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssMarginRight", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssMarginTop", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssMarginBottom", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssLeft", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssRight", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssTop", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssBottom", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssGridGap", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssGridColumnGap", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssGridRowGap", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssGap", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssColumnGap", "spaces_scale"),
    ("SpaceTheme", "CssSpace", "CssRowGap", "spaces_scale"),
    ("BorderTheme", "CssBorder", "CssBorder", "borders_scale"),
    ("BorderTheme", "CssBorder", "CssBorderLeft", "borders_scale"),
    (
        "BorderTheme",
        "CssBorder",
        "CssBorderRight",
        "borders_scale"
    ),
    ("BorderTheme", "CssBorder", "CssBorderTop", "borders_scale"),
    (
        "BorderTheme",
        "CssBorder",
        "CssBorderBottom",
        "borders_scale"
    ),
    ("BorderTheme", "CssBorder", "CssOutline", "borders_scale"),
    (
        "BorderTheme",
        "CssBorder",
        "CssOutlineLeft",
        "borders_scale"
    ),
    (
        "BorderTheme",
        "CssBorder",
        "CssOutlineRight",
        "borders_scale"
    ),
    ("BorderTheme", "CssBorder", "CssOutlineTop", "borders_scale"),
    (
        "BorderTheme",
        "CssBorder",
        "CssOutlineBottom",
        "borders_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssBorderLeftStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssBorderRightStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssBorderTopStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssBorderBottomStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssOutlineStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssOutlineLeftStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssOutlineRightStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssOutlineTopStyle",
        "border_styles_scale"
    ),
    (
        "BorderStyleTheme",
        "CssBorderStyle",
        "CssOutlineBottomStyle",
        "border_styles_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssBorderLeftWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssBorderRightWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssBorderTopWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssBorderBottomWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssOutlineWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssOutlineLeftWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssBorderWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssOutlineRightWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssOutlineTopWidth",
        "border_widths_scale"
    ),
    (
        "BorderWidthTheme",
        "CssBorderWidth",
        "CssOutlineBottomWidth",
        "border_widths_scale"
    ),
    (
        "BorderRadiusTheme",
        "CssBorderRadius",
        "CssBorderTopRightRadius",
        "radii_scale"
    ),
    (
        "BorderRadiusTheme",
        "CssBorderRadius",
        "CssBorderTopLeftRadius",
        "radii_scale"
    ),
    (
        "BorderRadiusTheme",
        "CssBorderRadius",
        "CssBorderBottomRightRadius",
        "radii_scale"
    ),
    (
        "BorderRadiusTheme",
        "CssBorderRadius",
        "CssBorderBottomLeftRadius",
        "radii_scale"
    ),
    ("ColorTheme", "CssColor", "CssColor", "colors_scale"),
    (
        "ColorTheme",
        "CssColor",
        "CssBackgroundColor",
        "colors_scale"
    ),
    ("ColorTheme", "CssColor", "CssBorderColor", "colors_scale"),
    (
        "ColorTheme",
        "CssColor",
        "CssBorderTopColor",
        "colors_scale"
    ),
    (
        "ColorTheme",
        "CssColor",
        "CssBorderBottomColor",
        "colors_scale"
    ),
    (
        "ColorTheme",
        "CssColor",
        "CssBorderRightColor",
        "colors_scale"
    ),
    (
        "ColorTheme",
        "CssColor",
        "CssBorderLeftColor",
        "colors_scale"
    ),
    ("ColorTheme", "CssColor", "CssOutlineColor", "colors_scale"),
    (
        "ColorTheme",
        "CssColor",
        "CssOutlineTopColor",
        "colors_scale"
    ),
    (
        "ColorTheme",
        "CssColor",
        "CssOutlineBottomColor",
        "colors_scale"
    ),
    (
        "ColorTheme",
        "CssColor",
        "CssOutlineRightColor",
        "colors_scale"
    ),
    (
        "ColorTheme",
        "CssColor",
        "CssOutlineLeftColor",
        "colors_scale"
    ),
    ("ColorTheme", "CssColor", "CssFill", "colors_scale"),
    ("ColorTheme", "CssColor", "CssStroke", "colors_scale",),
    ("ShadowTheme", "CssShadow", "CssBoxShadow", "shadows_scale"),
    ("ShadowTheme", "CssShadow", "CssTextShadow", "shadows_scale"),
]);

pub trait OverloadedGeneralStyleLookUp<T, R> {
    fn overloaded_general_lookup(&self, alias: T) -> Option<R>;
}

impl<Q, R> OverloadedGeneralStyleLookUp<Q, R> for Theme
where
    Q: 'static + Clone + Hash + Eq,
    R: 'static + Clone,
{
    fn overloaded_general_lookup(&self, alias: Q) -> Option<R> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, R>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

pub fn only_and_below<T, F, Ms>(bp: T, content: F) -> Node<Ms>
where
    T: BreakpointTheme + 'static,
    F: FnOnce() -> Node<Ms>,
{
    let bp_pair = with_themes(ReturnBpTuple(bp));
    match bp_pair {
        (_lower, Some(higher)) => {
            if window()
                .match_media(&format!("(max-width: {}px)", higher))
                .unwrap()
                .unwrap()
                .matches()
            {
                content()
            } else {
                empty![]
            }
        }
        (_lower, None) => content(),
    }
}

pub fn at_breakpoint_and_above<T>(bp: T) -> bool 
where
    T: BreakpointTheme + 'static,{
    let bp_pair = with_themes(ReturnBpTuple(bp));
    match bp_pair {
        (lower, Some(_higher)) => {
            window()
                .match_media(&format!("(min-width: {}px)", lower))
                .unwrap()
                .unwrap()
                .matches()
          
        }
        (lower, None) => {
            window()
                .match_media(&format!("(min-width: {}px)", lower))
                .unwrap()
                .unwrap()
                .matches()
          
        }
    }
}

pub fn only_and_above<T, F, Ms>(bp: T, content: F) -> Node<Ms>
where
    T: BreakpointTheme + 'static,
    F: FnOnce() -> Node<Ms>,
{
    let bp_pair = with_themes(ReturnBpTuple(bp));
    match bp_pair {
        (lower, Some(_higher)) => {
            if window()
                .match_media(&format!("(min-width: {}px)", lower))
                .unwrap()
                .unwrap()
                .matches()
            {
                content()
            } else {
                empty![]
            }
        }
        (lower, None) => {
            if window()
                .match_media(&format!("(min-width: {}px)", lower))
                .unwrap()
                .unwrap()
                .matches()
            {
                content()
            } else {
                empty![]
            }
        }
    }
}

pub fn only<T, F, Ms>(bp: T, content: F) -> Node<Ms>
where
    T: BreakpointTheme + 'static,
    F: FnOnce() -> Node<Ms>,
{
    let bp_pair = with_themes(ReturnBpTuple(bp));
    match bp_pair {
        (lower, Some(higher)) => {
            if window()
                .match_media(&format!(
                    "(min-width: {}px) and (max-width: {}px)",
                    lower, higher
                ))
                .unwrap()
                .unwrap()
                .matches()
            {
                content()
            } else {
                empty![]
            }
        }
        (lower, None) => {
            if window()
                .match_media(&format!("(min-width: {}px)", lower))
                .unwrap()
                .unwrap()
                .matches()
            {
                content()
            } else {
                empty![]
            }
        }
    }
}

pub fn except<T, F, Ms>(bp: T, content: F) -> Node<Ms>
where
    T: BreakpointTheme + 'static,
    F: FnOnce() -> Node<Ms>,
{
    let bp_pair = with_themes(ReturnBpTuple(bp));

    match bp_pair {
        (lower, Some(higher)) => {
            if window()
                .match_media(&format!(
                    "(max-width: {}px) and (max-width: {}px)",
                    lower, higher
                ))
                .unwrap()
                .unwrap()
                .matches()
                || window()
                    .match_media(&format!("(min-width: {}px)", higher))
                    .unwrap()
                    .unwrap()
                    .matches()
            {
                content()
            } else {
                empty![]
            }
        }
        (_lower, None) => content(),
    }
}

pub trait ActOnIteratorOfThemes<R> {
    fn call<'a, It>(&self, it: It) -> R
    where
        It: DoubleEndedIterator<Item = &'a Theme>;
}

// pub fn with_themes<Q, R>(with: Q) -> R
// where
//     Q: ActOnIteratorOfThemes<R>,
// {
//     if let Some(themes_idxs) = illicit::Env::get::<StateAccess<Vec<usize>>>() {
//         themes_idxs.get_with(|theme_idxs| {
//             THEMES_VEC.with(|global_v_t| {
//                 with.call(global_v_t.borrow().iter().enumerate().filter_map(|(x, b)| {
//                     if theme_idxs.contains(&x) {
//                         Some(b)
//                     } else {
//                         None
//                     }
//                 }))
//             })
//         })
//     } else {
//         with.call(std::iter::empty::<&Theme>())
//     }
// }


#[atom]
pub fn app_themes() -> Vec<Theme> {
    vec![]
}

pub fn with_themes<Q, R>(with: Q) -> R
where
    Q: ActOnIteratorOfThemes<R>,
{
    app_themes().observe_with (|v|    
        with.call(v.iter())
    )
}


pub fn load_app_themes(themes:&[fn()->Theme]) {
    for theme in themes {
        app_themes().update(|t| t.push(theme()))
    }
}

#[derive(Debug)]
pub struct Theme {
    pub name: String,
    pub anymap: anymap::Map<dyn Any>,
    pub spaces_scale: Vec<CssSpace>,
    pub font_sizes_scale: Vec<CssFontSize>,
    pub fonts_scale: Vec<CssFont>,
    pub font_weights_scale: Vec<CssFontWeight>,
    pub line_heights_scale: Vec<CssLineHeight>,
    pub letter_spacings_scale: Vec<CssLetterSpacing>,
    pub sizes_scale: Vec<CssSize>,
    pub borders_scale: Vec<CssBorder>,
    pub border_styles_scale: Vec<CssBorderStyle>,
    pub border_widths_scale: Vec<CssBorderWidth>,
    pub breakpoints_scale: Vec<u32>,
    pub media_bp_scale: Vec<CssMedia>,
    pub media_bp_pairs: Vec<(u32, Option<u32>)>,
    pub radii_scale: Vec<CssBorderRadius>,
    pub colors_scale: Vec<CssColor>,
    pub shadows_scale: Vec<CssShadow>,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: "".to_string(),
            anymap: anymap::Map::<dyn Any>::new(),
            spaces_scale: vec![],
            font_sizes_scale: vec![],
            fonts_scale: vec![],
            font_weights_scale: vec![],
            line_heights_scale: vec![],
            letter_spacings_scale: vec![],
            sizes_scale: vec![],
            borders_scale: vec![],
            border_widths_scale: vec![],
            breakpoints_scale: vec![],
            media_bp_scale: vec![],
            media_bp_pairs: vec![],
            border_styles_scale: vec![],
            colors_scale: vec![],
            shadows_scale: vec![],
            radii_scale: vec![],
        }
    }
}

pub trait OverloadedStyleLookUp<T, R> {
    fn overloaded_lookup(&self, alias: T) -> Option<R>;
}

// fn th_color<T,R>(alias:T) -> Option<CssColor> where T: 'static + ColorTheme + Clone,{
//     observe_with(my_theme(), |t| t.get(alias))
// }

impl<Q: 'static + SizeTheme> OverloadedStyleLookUp<Q, CssSize> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssSize> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssSize>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + ShadowTheme> OverloadedStyleLookUp<Q, CssShadow> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssShadow> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssShadow>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + SpaceTheme> OverloadedStyleLookUp<Q, CssSpace> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssSpace> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssSpace>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + FontSizeTheme> OverloadedStyleLookUp<Q, CssFontSize> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssFontSize> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssFontSize>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + ColorTheme> OverloadedStyleLookUp<Q, CssColor> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssColor> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssColor>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + BorderTheme> OverloadedStyleLookUp<Q, CssBorder> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorder> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssBorder>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + BorderWidthTheme> OverloadedStyleLookUp<Q, CssBorderWidth> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorderWidth> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssBorderWidth>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + BorderStyleTheme> OverloadedStyleLookUp<Q, CssBorderStyle> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorderStyle> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssBorderStyle>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + StyleTheme> OverloadedStyleLookUp<Q, Style> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<Style> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, Style>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + BorderRadiusTheme> OverloadedStyleLookUp<Q, CssBorderRadius> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssBorderRadius> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssBorderRadius>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + TransitionTheme> OverloadedStyleLookUp<Q, CssTransition> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssTransition> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssTransition>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl<Q: 'static + ZIndexTheme> OverloadedStyleLookUp<Q, CssZIndex> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<CssZIndex> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, CssZIndex>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}
impl<Q: 'static + BreakpointTheme> OverloadedStyleLookUp<Q, (u32, Option<u32>)> for Theme {
    fn overloaded_lookup(&self, alias: Q) -> Option<(u32, Option<u32>)> {
        if let Some(hm) = self.anymap.get::<HashMap<Q, (u32, Option<u32>)>>() {
            hm.get(&alias).cloned()
        } else {
            None
        }
    }
}

impl Theme {
    pub fn new(name: &str) -> Theme {
        Theme {
            name: name.into(),
            ..Theme::default()
        }
    }

    pub fn space_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssSpace> + Clone,
    {
        self.spaces_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    pub fn border_width_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssBorderWidth> + Clone,
    {
        self.border_widths_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    pub fn font_size_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssFontSize> + Clone,
    {
        self.font_sizes_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    pub fn font_weight_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssFontWeight> + Clone,
    {
        self.font_weights_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    pub fn size_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssSize> + Clone,
    {
        self.sizes_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    pub fn line_height_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssLineHeight> + Clone,
    {
        self.line_heights_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    pub fn letter_spacing_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssLetterSpacing> + Clone,
    {
        self.letter_spacings_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }

    //
    pub fn border_scale<S>(mut self, scale: &[S]) -> Theme
    where
        S: Into<CssBorder> + Clone,
    {
        self.borders_scale = scale.iter().cloned().map(|s| s.into()).collect::<_>();
        self
    }
    //
    pub fn breakpoint_scale<S: Into<Vec<u32>>>(mut self, scale: S) -> Theme {
        self.breakpoints_scale = scale.into();
        let mut lower = 0;
        let mut bp_pairs = vec![];

        for bp in &self.breakpoints_scale {
            bp_pairs.push((lower, Some(*bp - 1)));
            lower = *bp;
        }
        bp_pairs.push((lower, None));

        self.media_bp_scale = bp_pairs
            .iter()
            .map(|pair| match pair {
                (lower, Some(higher)) => CssMedia(format!(
                    "@media (min-width: {}px) and (max-width: {}px)",
                    lower, higher
                )),
                (lower, None) => CssMedia(format!("@media (min-width: {}px)", lower)),
            })
            .collect::<Vec<CssMedia>>();

        self.media_bp_pairs = bp_pairs;

        self
    }

    //

    pub fn general_get<T, R>(&self, alias: T) -> Option<R>
    where
        Self: OverloadedGeneralStyleLookUp<T, R>,
        T: Clone,
    {
        self.overloaded_general_lookup(alias)
    }

    

    pub fn get<T, R>(&self, alias: T) -> Option<R>
    where
        Self: OverloadedStyleLookUp<T, R>,
        T: Clone,
    {
        self.overloaded_lookup(alias)
    }

    pub fn set_size<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssSize>,
        Q: 'static + SizeTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssSize>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssSize>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_shadow<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssShadow>,
        Q: 'static + ShadowTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssShadow>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssShadow>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_color<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssColor>,
        Q: 'static + ColorTheme,
    {
        let value = value.into();
        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssColor>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssColor>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_space<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssSpace>,
        Q: 'static + SpaceTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssSpace>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssSpace>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_font_size<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssFontSize>,
        Q: 'static + FontSizeTheme,
    {
        let value = value.into();
        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssFontSize>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssFontSize>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_border<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssBorder>,
        Q: 'static + BorderTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssBorder>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssBorder>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_border_width<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssBorderWidth>,
        Q: 'static + BorderWidthTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssBorderWidth>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssBorderWidth>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_border_style<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssBorderStyle>,
        Q: 'static + BorderStyleTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssBorderStyle>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssBorderStyle>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_border_radius<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssBorderRadius>,
        Q: 'static + BorderRadiusTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssBorderRadius>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssBorderRadius>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_transition<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssTransition>,
        Q: 'static + TransitionTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssTransition>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssTransition>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_style<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<Style>,
        Q: 'static + StyleTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, Style>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, Style>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_breakpoint<Q>(mut self, alias: Q, value: (u32, Option<u32>)) -> Theme
    where
        Q: 'static + BreakpointTheme,
    {
        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, (u32, Option<u32>)>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, (u32, Option<u32>)>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_line_height<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssLineHeight>,
        Q: 'static + LineHeightTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssLineHeight>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssLineHeight>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }

    pub fn set_letter_spacing<T, Q>(mut self, alias: Q, value: T) -> Theme
    where
        T: Into<CssLetterSpacing>,
        Q: 'static + LetterSpacingTheme,
    {
        let value = value.into();

        if let Some(hm) = self.anymap.get_mut::<HashMap<Q, CssLetterSpacing>>() {
            hm.insert(alias, value);
        } else {
            let mut hm = HashMap::<Q, CssLetterSpacing>::new();
            hm.insert(alias, value);
            self.anymap.insert(hm);
        }
        self
    }
}

impl<T> From<T> for Style
where
    T: StyleTheme + 'static,
{
    fn from(v: T) -> Self {
        with_themes(ReturnStyle(v))
    }
}

struct ReturnStyle<T>(T)
where
    T: StyleTheme + 'static;

impl<T> ActOnIteratorOfThemes<Style> for ReturnStyle<T>
where
    T: StyleTheme + 'static,
{
    fn call<'a, It>(&self, mut it: It) -> Style
    where
        It: DoubleEndedIterator<Item = &'a Theme>,
    {
        it.find_map(|theme| theme.get::<T, Style>(self.0.clone()))
            .expect("Cannot find a theme that defines that CSS Value, are you sure you have provided access to that theme using `use_themes(|| THEME_NAME, ||..`")
    }
}
