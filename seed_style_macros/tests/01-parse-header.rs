// use tw_seed_hooks::flip;
// use tw_seed_hooks::flop;
use seed_style_macros::generate_froms;
// use std::panic::Location;

// generate_display_for_values!([
//     "BackgroundTest"
// ]);

// #[derive(CssStyleMacro)]
// #[prefix = "nng_"]
// // #[derive(Display, Clone, Debug)]
// // #[display(fmt = "counter-increment: {}")]
// pub enum CssCounterIncrement{
//     Identifier(String),
//     Integer(i32),
//     // #[display(fmt = "none")]
//     None,
//     // #[display(fmt = "inherit")]
//     Inherit,
//     StringValue(String),
// }
generate_froms!([
    (
        "FontSizeTheme",
        "CssFontSize",
        "CssFontSize",
        "font_sizes_scale"
    ),
    // ("SizeTheme", "CssSize", "CssWidth", "sizes_scale"),
    // ("SizeTheme", "CssSize", "CssMinWidth", "sizes_scale"),
    // ("SizeTheme", "CssSize", "CssMaxWidth", "sizes_scale"),
    // ("SizeTheme", "CssSize", "CssHeight", "sizes_scale"),
    // ("SizeTheme", "CssSize", "CssMinHeight", "sizes_scale"),
    // ("SizeTheme", "CssSize", "CssMaxHeight", "sizes_scale"),
    // ("SpaceTheme", "CssSpace", "CssPadding", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssPaddingLeft", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssPaddingRight", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssPaddingTop", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssPaddingBottom", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssMargin", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssMarginLeft", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssMarginRight", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssMarginTop", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssMarginBottom", "spaces_scale"),
    // // ("SpaceTheme", "CssSpace", "CssLeft", "spaces_scale"),
    // // ("SpaceTheme", "CssSpace", "CssRight", "spaces_scale"),
    // // ("SpaceTheme", "CssSpace", "CssTop", "spaces_scale"),
    // // ("SpaceTheme", "CssSpace", "CssBottom", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssGridGap", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssGridColumnGap", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssGridRowGap", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssGap", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssColumnGap", "spaces_scale"),
    // ("SpaceTheme", "CssSpace", "CssRowGap", "spaces_scale"),
    // ("BorderTheme", "CssBorder", "CssBorder", "borders_scale"),
    // ("BorderTheme", "CssBorder", "CssBorderLeft", "borders_scale"),
    // (
    //     "BorderTheme",
    //     "CssBorder",
    //     "CssBorderRight",
    //     "borders_scale"
    // ),
    // ("BorderTheme", "CssBorder", "CssBorderTop", "borders_scale"),
    // (
    //     "BorderTheme",
    //     "CssBorder",
    //     "CssBorderBottom",
    //     "borders_scale"
    // ),
    // ("BorderTheme", "CssBorder", "CssOutline", "borders_scale"),
    // (
    //     "BorderTheme",
    //     "CssBorder",
    //     "CssOutlineLeft",
    //     "borders_scale"
    // ),
    // (
    //     "BorderTheme",
    //     "CssBorder",
    //     "CssOutlineRight",
    //     "borders_scale"
    // ),
    // ("BorderTheme", "CssBorder", "CssOutlineTop", "borders_scale"),
    // (
    //     "BorderTheme",
    //     "CssBorder",
    //     "CssOutlineBottom",
    //     "borders_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssBorderLeftStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssBorderRightStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssBorderTopStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssBorderBottomStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssOutlineStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssOutlineLeftStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssOutlineRightStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssOutlineTopStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderStyleTheme",
    //     "CssBorderStyle",
    //     "CssOutlineBottomStyle",
    //     "border_styles_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssBorderLeftWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssBorderRightWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssBorderTopWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssBorderBottomWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssOutlineWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssOutlineLeftWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssBorderWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssOutlineRightWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssOutlineTopWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderWidthTheme",
    //     "CssBorderWidth",
    //     "CssOutlineBottomWidth",
    //     "border_widths_scale"
    // ),
    // (
    //     "BorderRadiusTheme",
    //     "CssBorderRadius",
    //     "CssBorderTopRightRadius",
    //     "radii_scale"
    // ),
    // (
    //     "BorderRadiusTheme",
    //     "CssBorderRadius",
    //     "CssBorderTopLeftRadius",
    //     "radii_scale"
    // ),
    // (
    //     "BorderRadiusTheme",
    //     "CssBorderRadius",
    //     "CssBorderBottomRightRadius",
    //     "radii_scale"
    // ),
    // (
    //     "BorderRadiusTheme",
    //     "CssBorderRadius",
    //     "CssBorderBottomLeftRadius",
    //     "radii_scale"
    // ),
    // ("ColorTheme", "CssColor", "CssColor", "colors_scale"),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssBackgroundColor",
    //     "colors_scale"
    // ),
    // ("ColorTheme", "CssColor", "CssBorderColor", "colors_scale"),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssBorderTopColor",
    //     "colors_scale"
    // ),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssBorderBottomColor",
    //     "colors_scale"
    // ),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssBorderRightColor",
    //     "colors_scale"
    // ),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssBorderLeftColor",
    //     "colors_scale"
    // ),
    // ("ColorTheme", "CssColor", "CssOutlineColor", "colors_scale"),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssOutlineTopColor",
    //     "colors_scale"
    // ),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssOutlineBottomColor",
    //     "colors_scale"
    // ),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssOutlineRightColor",
    //     "colors_scale"
    // ),
    // (
    //     "ColorTheme",
    //     "CssColor",
    //     "CssOutlineLeftColor",
    //     "colors_scale"
    // ),
    // ("ColorTheme", "CssColor", "CssFill", "colors_scale"),
    // ("ColorTheme", "CssColor", "CssStroke", "colors_scale",)(
    //     "ShadowTheme",
    //     "CssShadow",
    //     "CssBoxShadow",
    //     "shadows_scale"
    // ),
    // ("ShadowTheme", "CssShadow", "CssTextShadow", "shadows_scale"),
]);
fn main() {}
