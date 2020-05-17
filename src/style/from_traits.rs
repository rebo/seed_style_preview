use super::css_values::*;
use super::measures::*;

// Sizes
use crate::style::{Style, UpdateStyle};

impl<T> UpdateStyle<T> for ExactLength
where
    ExactLength: Into<T>,
    T: UpdateStyle<T>,
{
    fn update_style(self, style: &mut Style) {
        self.into().update_style(style)
    }
}

impl<T> UpdateStyle<T> for Percent
where
    Percent: Into<T>,
    T: UpdateStyle<T>,
{
    fn update_style(self, style: &mut Style) {
        self.into().update_style(style)
    }
}

impl From<ExactLength> for CssLetterSpacing {
    fn from(length: ExactLength) -> Self {
        CssLetterSpacing::Length(length)
    }
}

impl From<ExactLength> for CssLineHeight {
    fn from(length: ExactLength) -> Self {
        CssLineHeight::Length(length)
    }
}

impl From<ExactLength> for CssSize {
    fn from(length: ExactLength) -> Self {
        CssSize::Length(length)
    }
}

impl From<ExactLength> for CssWidth {
    fn from(length: ExactLength) -> Self {
        CssWidth::Length(length)
    }
}

impl From<ExactLength> for CssHeight {
    fn from(length: ExactLength) -> Self {
        CssHeight::Length(length)
    }
}

impl From<ExactLength> for CssMinHeight {
    fn from(length: ExactLength) -> Self {
        CssMinHeight::Length(length)
    }
}

impl From<ExactLength> for CssMaxHeight {
    fn from(length: ExactLength) -> Self {
        CssMaxHeight::Length(length)
    }
}

impl From<ExactLength> for CssMinWidth {
    fn from(length: ExactLength) -> Self {
        CssMinWidth::Length(length)
    }
}

impl From<ExactLength> for CssMaxWidth {
    fn from(length: ExactLength) -> Self {
        CssMaxWidth::Length(length)
    }
}

impl From<ExactLength> for CssGridGap {
    fn from(length: ExactLength) -> Self {
        CssGridGap::Length(length)
    }
}

impl From<ExactLength> for CssGridRowGap {
    fn from(length: ExactLength) -> Self {
        CssGridRowGap::Length(length)
    }
}

impl From<ExactLength> for CssGridColumnGap {
    fn from(length: ExactLength) -> Self {
        CssGridColumnGap::Length(length)
    }
}

impl From<ExactLength> for CssGap {
    fn from(length: ExactLength) -> Self {
        CssGap::Length(length)
    }
}

impl From<ExactLength> for CssRowGap {
    fn from(length: ExactLength) -> Self {
        CssRowGap::Length(length)
    }
}

impl From<ExactLength> for CssColumnGap {
    fn from(length: ExactLength) -> Self {
        CssColumnGap::Length(length)
    }
}

impl From<Percent> for CssSize {
    fn from(percentage: Percent) -> Self {
        CssSize::Percentage(percentage)
    }
}

impl From<Percent> for CssWidth {
    fn from(percentage: Percent) -> Self {
        CssWidth::Percentage(percentage)
    }
}

impl From<Percent> for CssHeight {
    fn from(percentage: Percent) -> Self {
        CssHeight::Percentage(percentage)
    }
}

impl From<Percent> for CssMinHeight {
    fn from(percentage: Percent) -> Self {
        CssMinHeight::Percentage(percentage)
    }
}

impl From<Percent> for CssMaxHeight {
    fn from(percentage: Percent) -> Self {
        CssMaxHeight::Percentage(percentage)
    }
}

impl From<Percent> for CssMinWidth {
    fn from(percentage: Percent) -> Self {
        CssMinWidth::Percentage(percentage)
    }
}

impl From<Percent> for CssMaxWidth {
    fn from(percentage: Percent) -> Self {
        CssMaxWidth::Percentage(percentage)
    }
}

impl From<Percent> for CssGridGap {
    fn from(percentage: Percent) -> Self {
        CssGridGap::Percentage(percentage)
    }
}

impl From<Percent> for CssGridRowGap {
    fn from(percentage: Percent) -> Self {
        CssGridRowGap::Percentage(percentage)
    }
}

impl From<Percent> for CssGridColumnGap {
    fn from(percentage: Percent) -> Self {
        CssGridColumnGap::Percentage(percentage)
    }
}

impl From<Percent> for CssGap {
    fn from(percentage: Percent) -> Self {
        CssGap::Percentage(percentage)
    }
}

impl From<Percent> for CssRowGap {
    fn from(percentage: Percent) -> Self {
        CssRowGap::Percentage(percentage)
    }
}

impl From<Percent> for CssColumnGap {
    fn from(percentage: Percent) -> Self {
        CssColumnGap::Percentage(percentage)
    }
}

/// Space

impl From<ExactLength> for CssSpace {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssMargin {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssMarginLeft {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssMarginRight {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssMarginBottom {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssMarginTop {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssPadding {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssPaddingLeft {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssPaddingRight {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssPaddingBottom {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssTop {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}


impl From<ExactLength> for CssColumnWidth {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}


impl From<ExactLength> for CssBottom {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssLeft {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssRight {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssPaddingTop {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}
impl From<Percent> for CssBorderRadius {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssSpace {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssMargin {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssMarginLeft {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssMarginRight {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssMarginBottom {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssMarginTop {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssPadding {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssPaddingLeft {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssPaddingRight {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssPaddingBottom {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<Percent> for CssPaddingTop {
    fn from(v: Percent) -> Self {
        Self::Percentage(v)
    }
}

impl From<ExactLength> for CssBorderWidth {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssFontSize {
    fn from(v: ExactLength) -> Self {
        Self::Size(v)
    }
}

impl From<ExactLength> for CssBorderLeftWidth {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssBorderRadius {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssBorderRightWidth {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssBorderTopWidth {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<ExactLength> for CssBorderBottomWidth {
    fn from(v: ExactLength) -> Self {
        Self::Length(v)
    }
}

impl From<Percent> for CssLineHeight {
    fn from(pc: Percent) -> Self {
        CssLineHeight::Percentage(pc)
    }
}
