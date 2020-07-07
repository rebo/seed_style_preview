use super::measures::*;
use crate::style::{CssValueTrait, Style, UpdateStyle};
use derive_more::Display;

use seed_style_macros::{create_enums, CssStyleMacro};
use std::panic::Location;
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bg_attachment"]
#[display(fmt = "background-attachment: {};")]
pub enum CssBackgroundAttachment {
    #[display(fmt = "scroll")]
    Scroll,
    #[display(fmt = "fixed")]
    Fixed,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "column-span: {};")]
pub enum CssColumnSpan {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "all")]
    All,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "column-fill: {};")]
pub enum CssColumnsFill {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "balance")]
    Balance,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "column-rule: {};")]
pub enum CssColumnRule {
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "column-count: {};")]
pub enum CssColumnCount {
    #[display(fmt = "auto")]
    Auto,
    Number(u32),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "column-width: {};")]
pub enum CssColumnWidth {
    Length(ExactLength),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
pub enum CssRaw {
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bg_image"]
#[display(fmt = "background-image: {};")]
pub enum CssBackgroundImage {
    Uri(String),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bg_position"]
#[display(fmt = "background-position: {};")]
pub enum CssBackgroundPosition {
    PercentX(Percent),
    PosX(ExactLength),
    PercentY(Percent),
    PosY(ExactLength),
    #[display(fmt = "top-left")]
    TopLeft,
    #[display(fmt = "top-cebter")]
    TopCenter,
    #[display(fmt = "top-right")]
    TopRight,
    #[display(fmt = "center-left")]
    CenterLeft,
    #[display(fmt = "center-center")]
    Center,
    #[display(fmt = "center-right")]
    CenterRight,
    #[display(fmt = "bottom-left")]
    BottomLeft,
    #[display(fmt = "bottom-center")]
    BottomCenter,
    #[display(fmt = "bottom-right")]
    BottomRight,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bg_repeat"]
#[display(fmt = "background-repeat: {};")]
pub enum CssBackgroundRepeat {
    #[display(fmt = "repeat")]
    Repeat,
    #[display(fmt = "repeat-x")]
    RepeatX,
    #[display(fmt = "repeat-y")]
    RepeatY,
    #[display(fmt = "no-repeat")]
    NoRepeat,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-collapse: {};")]
pub enum CssBorderCollapse {
    #[display(fmt = "collapse")]
    Collapse,
    #[display(fmt = "separate")]
    Separate,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug)]
#[display(fmt = "border-spacing: {};")]
pub enum CssBorderSpacing {
    #[display(fmt = "length-x")]
    LengthX,
    #[display(fmt = "length-y")]
    LengthY,
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "capition-side: {};")]
pub enum CssCaptionSide {
    #[display(fmt = "top")]
    Top,
    #[display(fmt = "bottom")]
    Bottom,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "clear: {};")]
pub enum CssClear {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "both")]
    Both,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "clip: {};")]
pub enum CssClip {
    Shape(String),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "content: {};")]
pub enum CssContent {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "none")]
    None,
    #[display(fmt = "open-quote")]
    OpenQuote,
    #[display(fmt = "close-quote")]
    CloseQuote,
    #[display(fmt = "no-open-quote")]
    NoOpenQuote,
    #[display(fmt = "no-close-quote")]
    NoCloseQuote,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "counter-increment: {};")]
pub enum CssCounterIncrement {
    Identifier(String),
    Integer(i32),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "cursor: {};")]
pub enum CssCursor {
    Uri(String),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "crosshair")]
    Crosshair,
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "pointer")]
    Pointer,
    #[display(fmt = "move")]
    Move,
    #[display(fmt = "e-resize")]
    EResize,
    #[display(fmt = "ne-resize")]
    NeResize,
    #[display(fmt = "nw-resize")]
    NwResize,
    #[display(fmt = "n-resize")]
    NResize,
    #[display(fmt = "se-resize")]
    SeResize,
    #[display(fmt = "sw-resize")]
    SwResize,
    #[display(fmt = "s-resize")]
    SResize,
    #[display(fmt = "w-resize")]
    WResize,
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "wait")]
    Wait,
    #[display(fmt = "help")]
    Help,
    #[display(fmt = "progress")]
    Progress,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "direction: {};")]
pub enum CssDirection {
    #[display(fmt = "ltr")]
    Ltr,
    #[display(fmt = "rtl")]
    Rtl,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "empty-cells: {};")]
pub enum CssEmptyCells {
    #[display(fmt = "show")]
    Show,
    #[display(fmt = "hide")]
    Hide,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "float: {};")]
pub enum CssFloat {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "font-style: {};")]
pub enum CssFontStyle {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "italic")]
    Italic,
    #[display(fmt = "oblique")]
    Oblique,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "font-variant: {};")]
pub enum CssFontVariant {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "small-caps")]
    SmallCaps,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "font-weight: {};")]
pub enum CssFontWeight {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "bold")]
    Bold,
    #[display(fmt = "bolder")]
    Bolder,
    #[display(fmt = "lighter")]
    Lighter,
    #[display(fmt = "100")]
    V100,
    #[display(fmt = "200")]
    V200,
    #[display(fmt = "300")]
    V300,
    #[display(fmt = "400")]
    V400,
    #[display(fmt = "500")]
    V500,
    #[display(fmt = "600")]
    V600,
    #[display(fmt = "700")]
    V700,
    #[display(fmt = "800")]
    V800,
    #[display(fmt = "900")]
    V900,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "list-style-image: {};")]
pub enum CssListStyleImage {
    Url(String),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "list-style-position: {};")]
pub enum CssListStylePosition {
    #[display(fmt = "inside")]
    Inside,
    #[display(fmt = "outside")]
    Outside,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "list-style-type: {};")]
pub enum CssListStyleType {
    #[display(fmt = "disc")]
    Disc,
    #[display(fmt = "circle")]
    Circle,
    #[display(fmt = "decimal")]
    Decimal,
    #[display(fmt = "decimal-leading-zero")]
    DecimalLeadingZero,
    #[display(fmt = "lower-roman")]
    LowerRoman,
    #[display(fmt = "upper-roman")]
    UpperRoman,
    #[display(fmt = "lower-greek")]
    LowerGreek,
    #[display(fmt = "lower-latin")]
    LowerLatin,
    #[display(fmt = "upper-latin")]
    UpperLatin,
    #[display(fmt = "armenian")]
    Armenian,
    #[display(fmt = "georgian")]
    Georgian,
    #[display(fmt = "lower-alpha")]
    LowerAlpha,
    #[display(fmt = "upper-alpha")]
    UpperAlpha,
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "list-style: {};")]
pub enum CssListStyle {
    #[display(fmt = "{} {} {}", _0, _1, _2)]
    ListStyle(CssListStyleType, CssListStylePosition, CssListStyleImage),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "orphans: {};")]
pub enum CssOrphans {
    Integer(i32),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "overflow: {};")]
pub enum CssOverflow {
    #[display(fmt = "visible")]
    Visible,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "scroll")]
    Scroll,
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "overflow-x: {};")]
pub enum CssOverflowX {
    #[display(fmt = "visible")]
    Visible,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "scroll")]
    Scroll,
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "overflow-y: {};")]
pub enum CssOverflowY {
    #[display(fmt = "visible")]
    Visible,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "scroll")]
    Scroll,
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "page-break: {};")]
pub enum CssPageBreak {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "always")]
    Always,
    #[display(fmt = "avoid")]
    Avoid,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "pos"]
#[display(fmt = "position: {};")]
pub enum CssPosition {
    #[display(fmt = "sticky")]
    Sticky,
    #[display(fmt = "static")]
    Static,
    #[display(fmt = "relative")]
    Relative,
    #[display(fmt = "absolute")]
    Absolute,
    #[display(fmt = "fixed")]
    Fixed,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "quotes: {};")]
pub enum CssQuotes {
    #[display(fmt = "{} {}", _0, _1)]
    Quotes(String, String),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "table-layout: {};")]
pub enum CssTableLayout {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "fixed")]
    Fixed,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "text-align: {};")]
pub enum CssTextAlign {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "justify")]
    Justify,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "text-decoration: {};")]
pub enum CssTextDecoration {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "underline")]
    Underline,
    #[display(fmt = "overline")]
    Overline,
    #[display(fmt = "line-through")]
    LineThrough,
    #[display(fmt = "blink")]
    Blink,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "text-indent: {};")]
pub enum CssTextIndent {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "text-transform: {};")]
pub enum CssTextTransform {
    #[display(fmt = "capitalize")]
    Capitalize,
    #[display(fmt = "uppercase")]
    Uppercase,
    #[display(fmt = "lowercase")]
    Lowercase,
    #[display(fmt = "none")]
    None,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "unicode-bidi: {};")]
pub enum CssUnicodeBidi {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "embed")]
    Embed,
    #[display(fmt = "bidi-override")]
    BidiOverride,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "vertical-align: {};")]
pub enum CssVerticalAlign {
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "sub")]
    Sub,
    #[display(fmt = "super")]
    Super,
    #[display(fmt = "top")]
    Top,
    #[display(fmt = "text-top")]
    TextTop,
    #[display(fmt = "middle")]
    Middle,
    #[display(fmt = "bottom")]
    Bottom,
    Percentage(Percent),
    Length(ExactLength),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "visibility: {};")]
pub enum CssVisibility {
    #[display(fmt = "visible")]
    Visible,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "collapse")]
    Collapse,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "white-space: {};")]
pub enum CssWhiteSpace {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "pre")]
    Pre,
    #[display(fmt = "no-wrap")]
    NoWrap,
    #[display(fmt = "pre-wrap")]
    PreWrap,
    #[display(fmt = "pre-line")]
    PreLine,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "widows: {};")]
pub enum CssWidows {
    Integer(i32),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "word-spacing: {};")]
pub enum CssWordSpacing {
    #[display(fmt = "normal")]
    Normal,
    Length(ExactLength),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "z-index: {};")]
pub enum CssZIndex {
    #[display(fmt = "auto")]
    Auto,
    Integer(i32),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "m"]
#[display(fmt = "margin: {};")]
pub enum CssMargin {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "mt"]
#[display(fmt = "margin-top: {};")]
pub enum CssMarginTop {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "mb"]
#[display(fmt = "margin-bottom: {};")]
pub enum CssMarginBottom {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "ml"]
#[display(fmt = "margin-left: {};")]
pub enum CssMarginLeft {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "mr"]
#[display(fmt = "margin-right: {};")]
pub enum CssMarginRight {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "generic-space: {};")]
pub enum CssSpace {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "top: {};")]
pub enum CssTop {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "bottom: {};")]
pub enum CssBottom {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "left: {};")]
pub enum CssLeft {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "right: {};")]
pub enum CssRight {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-gap: {};")]
pub enum CssGridGap {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-column-gap: {};")]
pub enum CssGridColumnGap {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-row-gap: {};")]
pub enum CssGridRowGap {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "gap: {};")]
pub enum CssGap {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "column-gap: {};")]
pub enum CssColumnGap {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "row-gap: {};")]
pub enum CssRowGap {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "p"]
#[display(fmt = "padding: {};")]
pub enum CssPadding {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "pt"]
#[display(fmt = "padding-top: {};")]
pub enum CssPaddingTop {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "pr"]
#[display(fmt = "padding-right: {};")]
pub enum CssPaddingRight {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "pl"]
#[display(fmt = "padding-left: {};")]
pub enum CssPaddingLeft {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "pb"]
#[display(fmt = "padding-bottom: {};")]
pub enum CssPaddingBottom {
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "b_style"]
#[display(fmt = "border-style: {};")]
pub enum CssBorderStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bl_style"]
#[display(fmt = "border-left-style: {};")]
pub enum CssBorderLeftStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "br_style"]
#[display(fmt = "border-right-style: {};")]
pub enum CssBorderRightStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bt_style"]
#[display(fmt = "border-top-style: {};")]
pub enum CssBorderTopStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bb_style"]
#[display(fmt = "border-bottom-style: {};")]
pub enum CssBorderBottomStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-style: {};")]
pub enum CssOutlineStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-left-style: {};")]
pub enum CssOutlineLeftStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-right-style: {};")]
pub enum CssOutlineRightStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-top-style: {};")]
pub enum CssOutlineTopStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-bottom-style: {};")]
pub enum CssOutlineBottomStyle {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "dotted")]
    Dotted,
    #[display(fmt = "dashed")]
    Dashed,
    #[display(fmt = "solid")]
    Solid,
    #[display(fmt = "double")]
    Double,
    #[display(fmt = "groove")]
    Groove,
    #[display(fmt = "ridge")]
    Ridge,
    #[display(fmt = "inset")]
    Inset,
    #[display(fmt = "outset")]
    Outset,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "b_width"]
#[display(fmt = "border-width: {};")]
pub enum CssBorderWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bl_width"]
#[display(fmt = "border-left-width: {};")]
pub enum CssBorderLeftWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "br_width"]
#[display(fmt = "border-right-width: {};")]
pub enum CssBorderRightWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bt_width"]
#[display(fmt = "border-top-width: {};")]
pub enum CssBorderTopWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bb_width"]
#[display(fmt = "border-bottom-width: {};")]
pub enum CssBorderBottomWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-width: {};")]
pub enum CssOutlineWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-left-width: {};")]
pub enum CssOutlineLeftWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-right-width: {};")]
pub enum CssOutlineRightWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-top-width: {};")]
pub enum CssOutlineTopWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-bottom-width: {};")]
pub enum CssOutlineBottomWidth {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "thin")]
    Thin,
    #[display(fmt = "thick")]
    Thick,
    Length(ExactLength),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "generic-size: {};")]
pub enum CssSize {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "flex-basis: {};")]
pub enum CssFlexBasis {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "w"]
#[display(fmt = "width: {};")]
pub enum CssWidth {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "h"]
#[display(fmt = "height: {};")]
pub enum CssHeight {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "min_w"]
#[display(fmt = "min-width: {};")]
pub enum CssMinWidth {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "max_w"]
#[display(fmt = "max-width: {};")]
pub enum CssMaxWidth {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "min_h"]
#[display(fmt = "min-height: {};")]
pub enum CssMinHeight {
    #[display(fmt = "auto")]
    Auto,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "max_h"]
#[display(fmt = "max-height: {};")]
pub enum CssMaxHeight {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "none")]
    None,
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border: {};")]
pub enum CssBorder {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Border(CssBorderWidth, CssBorderStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-left: {};")]
pub enum CssBorderLeft {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Border(CssBorderWidth, CssBorderStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-right: {};")]
pub enum CssBorderRight {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Border(CssBorderWidth, CssBorderStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-top: {};")]
pub enum CssBorderTop {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Border(CssBorderWidth, CssBorderStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-bottom: {};")]
pub enum CssBorderBottom {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Border(CssBorderWidth, CssBorderStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline: {};")]
pub enum CssOutline {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Outline(CssOutlineWidth, CssOutlineStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-left: {};")]
pub enum CssOutlineLeft {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Outline(CssOutlineWidth, CssOutlineStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-right: {};")]
pub enum CssOutlineRight {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Outline(CssOutlineWidth, CssOutlineStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-top: {};")]
pub enum CssOutlineTop {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Outline(CssOutlineWidth, CssOutlineStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-bottom: {};")]
pub enum CssOutlineBottom {
    #[display(fmt = "{} {} {}", "_0.value_only()", "_1.value_only()", "_2.value_only()",)]
    Outline(CssOutlineWidth, CssOutlineStyle, CssColor),
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "transition: {};")]
pub enum CssTransition {
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "letter-spacing: {};")]
pub enum CssLetterSpacing {
    Normal,
    Length(ExactLength),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "line-height: {};")]
pub enum CssLineHeight {
    Normal,
    Number(f64),
    Length(ExactLength),
    Percentage(Percent),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "radius"]
#[display(fmt = "border-radius: {};")]
pub enum CssBorderRadius {
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-top-right-radius: {};")]
pub enum CssBorderTopRightRadius {
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-top-left-radius: {};")]
pub enum CssBorderTopLeftRadius {
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-bottom-right-radius: {};")]
pub enum CssBorderBottomRightRadius {
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "border-bottom-left-radius: {};")]
pub enum CssBorderBottomLeftRadius {
    Length(ExactLength),
    Percentage(Percent),
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "font: {};")]
pub enum CssFont {
    #[display(fmt = "{} {} {} {}/{} {}", _0, _1, _2, _3, _4, _5)]
    Font(
        CssFontStyle,
        CssFontVariant,
        CssFontWeight,
        CssFontSize,
        CssLineHeight,
        CssFontFamily,
    ),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "font-size: {};")]
pub enum CssFontSize {
    Size(ExactLength),
    Percentage(Percent),
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "color: {};")]
pub enum CssColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), 
    #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "stroke: {};")]
pub enum CssStroke {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bg_color"]
#[display(fmt = "background-color: {};")]
pub enum CssBackgroundColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "fill: {};")]
pub enum CssFill {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "b_color"]
#[display(fmt = "border-color: {};")]
pub enum CssBorderColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bl_color"]
#[display(fmt = "border-left-color: {};")]
pub enum CssBorderLeftColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "br_color"]
#[display(fmt = "border-right-color: {};")]
pub enum CssBorderRightColor {
    #[display(fmt = "{} {} {} {})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "{} {} {}", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), 
    #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bt_color"]
#[display(fmt = "border-top-color: {};")]
pub enum CssBorderTopColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[short_prop = "bb_color"]
#[display(fmt = "border-bottom-color: {};")]
pub enum CssBorderBottomColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-color: {};")]
pub enum CssOutlineColor {
    #[display(fmt = "rgba({},{},{},{}", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-left-color: {};")]
pub enum CssOutlineLeftColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-right-color: {};")]
pub enum CssOutlineRightColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-top-color: {};")]
pub enum CssOutlineTopColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "outline-bottom-color: {};")]
pub enum CssOutlineBottomColor {
    #[display(fmt = "rgba({},{},{},{})", _0, _1, _2, _3)]
    Rgba(f64, f64, f64, f64),
    #[display(fmt = "hsl({},{}%,{}%)", _0, _1, _2)]
    Hsl(f64, f64, f64),
    #[display(fmt = "hsla({},{}%,{}%,{})", _0, _1, _2, _3)]
    Hsla(f64, f64, f64, f64), #[display(fmt = "#{:06x}", _0)]
    Hex(i32),
    StringValue(String),
    #[display(fmt = "inherit")]
    Inherit,
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "shadow: {};")]
pub enum CssShadow {
    #[display(fmt = "{},{},{},{}", _0, _1, _2, _3)]
    Shadow(ExactLength, ExactLength, ExactLength, String),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

// impl CssShadow {
//     fn new(ExactLength, ExactLength) {
//         CssShadow::
//     }
// }

// S.box_shadow_build( px(3), px(2), rgb(30,40,50), |v| v.inset().blur(px(2)))

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "text-shadow: {};")]
pub enum CssTextShadow {
    #[display(fmt = "{} {} {} {}", _0, _1, _2, _3)]
    Shadow(ExactLength, ExactLength, ExactLength, String),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "box-shadow: {};")]
pub enum CssBoxShadow {
    #[display(fmt = "{} {} {} {}", _0, _1, _2, _3)]
    Shadow(ExactLength, ExactLength, ExactLength, String),
    #[display(fmt = "none")]
    None,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-template-columns: {};")]
pub enum CssGridTemplateColumns {
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "unset")]
    Unset,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-template-rows: {};")]
pub enum CssGridTemplateRows {
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "unset")]
    Unset,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-area: {};")]
pub enum CssGridArea {
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "unset")]
    Unset,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-auto-columns: {};")]
pub enum CssGridAutoColumns {
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "unset")]
    Unset,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-auto-rows: {};")]
pub enum CssGridAutoRows {
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "unset")]
    Unset,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "grid-auto-flow: {};")]
pub enum CssGridAutoFlow {
    #[display(fmt = "row")]
    Row,
    #[display(fmt = "column")]
    Column,
    #[display(fmt = "dense")]
    Dense,
    #[display(fmt = "row dense")]
    RowDense,
    #[display(fmt = "column dense")]
    ColumnDense,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "unset")]
    Unset,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "flex: {};")]
pub enum CssFlex {
    #[display(fmt = "0 1 auto")]
    Initial,
    #[display(fmt = "1 1 0%")]
    One,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "flex-direction: {};")]
pub enum CssFlexDirection {
    #[display(fmt = "row")]
    Row,
    #[display(fmt = "column")]
    Column,
    #[display(fmt = "row-reverse")]
    RowReverse,
    #[display(fmt = "column-reverse")]
    ColumnReverse,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "flex-wrap: {};")]
pub enum CssFlexWrap {
    #[display(fmt = "nowrap")]
    NoWrap,
    #[display(fmt = "wrap-reverse")]
    WrapReverse,
    #[display(fmt = "wrap")]
    Wrap,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "align-items: {};")]
pub enum CssAlignItems {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "align-self: {};")]
pub enum CssAlignSelf {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "justify-items: {};")]
pub enum CssJustifyItems {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "self-start")]
    SelfStart,
    #[display(fmt = "self-end")]
    SelfEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "safe")]
    Safe,
    #[display(fmt = "unsafe")]
    Unsafe,
    #[display(fmt = "legacy")]
    Legacy,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "justify-self: {};")]
pub enum CssJustifySelf {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "self-start")]
    SelfStart,
    #[display(fmt = "self-end")]
    SelfEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "safe")]
    Safe,
    #[display(fmt = "unsafe")]
    Unsafe,
    #[display(fmt = "legacy")]
    Legacy,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "justify-content: {};")]
pub enum CssJustifyContent {
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "space-between")]
    SpaceBetween,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "align-content: {};")]
pub enum CssAlignContent {
    #[display(fmt = "stretcj")]
    Stretch,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "space-between")]
    SpaceBetween,
    #[display(fmt = "space-around")]
    SpaceAround,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    #[display(fmt = "none")]
    None,
    StringValue(String),
}
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "box-sizing: {};")]
pub enum CssBoxSizing {
    #[display(fmt = "border-box")]
    BorderBox,
    #[display(fmt = "content-box")]
    ContentBox,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

// vendor specific not ideal hack but works for now.
// need to implement display directly
#[derive(Display, Clone, Debug, CssStyleMacro)]
#[vendor_prefixes = "-webkit-"]
#[display(fmt = "backface-visibility: {};")]
pub enum CssBackfaceVisibility {
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "visible")]
    Visible,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "-webkit-font-smoothing: {};")]
pub enum CssWebkitFontSmoothing {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "antialiased")]
    Antialiased,
    #[display(fmt = "subpixel-antialiased")]
    SubpixelAntialiased,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

#[derive(Display, Clone, Debug, CssStyleMacro)]
#[display(fmt = "display: {};")]
pub enum CssDisplay {
    #[display(fmt = "inline")]
    Inline,
    #[display(fmt = "block")]
    Block,
    #[display(fmt = "contents")]
    Contents,
    #[display(fmt = "flex")]
    Flex,
    #[display(fmt = "grid")]
    Grid,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "inline-block")]
    InlineBlock,
    #[display(fmt = "inline=flex")]
    InlineFlex,
    #[display(fmt = "inline-grid")]
    InlineGrid,
    #[display(fmt = "inline-table")]
    InlineTable,
    #[display(fmt = "list-tem")]
    ListItem,
    #[display(fmt = "run-in")]
    RunIn,
    #[display(fmt = "table")]
    Table,
    #[display(fmt = "table-caption")]
    TableCaption,
    #[display(fmt = "table-column-group")]
    TableColumnGroup,
    #[display(fmt = "table-header-group")]
    TableHeaderGroup,
    #[display(fmt = "table-footer-group")]
    TableFooterGroup,
    #[display(fmt = "table-row-group")]
    TableRowGroup,
    #[display(fmt = "table-cell")]
    TableCell,
    #[display(fmt = "table-column")]
    TableColumn,
    #[display(fmt = "table-row")]
    TableRow,
    #[display(fmt = "none")]
    None,
    #[display(fmt = "initial")]
    Initial,
    #[display(fmt = "inherit")]
    Inherit,
    StringValue(String),
}

create_enums!([
    "AnimationDelay",
    "AnimationDirection",
    "AnimationDuration",
    "AnimationFillMode",
    "AnimationIterationCount",
    "AnimationName",
    "AnimationPlayState",
    "AnimationTimingFunction",
    "Animation",
    "Background",
    "BackgroundBlendMode",
    "BackgroundClip",
    "BackgroundOrigin",
    "BackgroundSize",
    "BorderImage",
    "BorderImageOutset",
    "BorderImageRepeat",
    "BorderImageSlice",
    "BorderImageSource",
    "BorderImageWidth",
    "BoxDecorationBreak",
    "BreakAfter",
    "BreakBefore",
    "BreakInside",
    "CaretColor",
    "ClipPath",
    "ColumnRuleColor",
    "ColumnRuleStyle",
    "ColumnRuleWidth",
    "Columns",
    "CounterReset",
    "Filter",
    "FlexFlow",
    "FlexGrow",
    "FlexShrink",
    "FontFamily",
    "FontFeatureSettings",
    "FontKerning",
    "FontLanguageOverride",
    "FontSizeAdjust",
    "FontStretch",
    "FontSynthesis",
    "FontVariantAlternates",
    "FontVariantCaps",
    "FontVariantEastAsian",
    "FontVariantLigatures",
    "FontVariantNumeric",
    "FontVariantPosition",
    "Grid",
    "GridColumn",
    "GridColumnEnd",
    "GridColumnStart",
    "GridRow",
    "GridRowEnd",
    "GridRowStart",
    "GridTemplate",
    "GridTemplateAreas",
    "Hyphens",
    "ImageRendering",
    "Isolation",
    "LineBreak",
    "Mask",
    "MaskType",
    "MixBlendMode",
    "ObjectFit",
    "ObjectPosition",
    "Opacity",
    "Order",
    "OverflowWrap",
    "PageBreakAfter",
    "PageBreakBefore",
    "PageBreakInside",
    "Perspective",
    "PerspectiveOrigin",
    "PlaceContent",
    "PointerEvents",
    "Resize",
    "ScrollBehavior",
    "ShapeImageThreshold",
    "ShapeMargin",
    "TabSize",
    "TextAlignLast",
    "TextCombineUpright",
    "TextDecorationColor",
    "TextDecorationLine",
    "TextDecorationStyle",
    "TextEmphasis",
    "TextEmphasisColor",
    "TextEmphasisPosition",
    "TextEmphasisStyle",
    "TextJustify",
    "TextOrientation",
    "TextOverflow",
    "TextUnderlinePosition",
    "TouchAction",
    "Transform",
    "TransformOrigin",
    "TransformStyle",
    "TransitionDelay",
    "TransitionDuration",
    "TransitionProperty",
    "TransitionTimingFunction",
    "UserSelect",
    "WillChange",
    "WordBreak",
    "WordWrap",
    "WritingMode",
]);

pub fn trim_css_ends<T:ToString>(property:T, name: &str) -> String {
    property.to_string().trim_start_matches(name).trim_end_matches(";").to_string()
}


#[derive(Debug, Clone)]
pub struct CssMedia(pub String);

impl CssMedia {
    pub fn render(&self) -> String {
        format!("@media {}", self.0)
    }
}
