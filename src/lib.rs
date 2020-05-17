#![feature(track_caller)]
mod style;

// exports
pub use style::CssValueTrait;

// style builder
pub use style::s;

// style property argument trait
pub use style::{
    // exports for Seed Layout
    composition::{Composition,SeedBreakpoint,WithLayoutComposition },
    // Css Values
    css_values::*,
    // resizing
    helpers::conditionally_skip_rendering,
    layout::{Layout, LayoutArea, NoArea, WithGridLayout},
    // measures
    measures::{cm, em, hsl, pc, px, rem, rgb, rgba, vh, vw},
    // presets
    presets::{seed_colors, style_presets},
    theme::change_theme_with_name,
    theme::use_themes,
    // themes
    theme::Theme,
    // themes, conditional rendering
    theme::{except, only, only_and_above, only_and_below},
    //theme alias keys
    theme::{
        BorderRadiusTheme, BorderStyleTheme, BorderTheme, BorderWidthTheme, BreakpointTheme,
        ColorTheme, DisplayTheme, FontSizeTheme, FontTheme, LetterSpacingTheme, LineHeightTheme,
        ShadowTheme, SizeTheme, SpaceTheme, StyleTheme, TransitionTheme, ZIndexTheme,
    },
    // global style api
    GlobalStyle,
    // extension trait to allow Style structs to be update_el processed by seed.
    LocalUpdateEl,
    // Style struct, technically user shouldn't really need to access this directly
    Style,
    // style property argument trait
    UpdateStyle,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
