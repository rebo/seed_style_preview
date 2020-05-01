use derive_more::Display;
use harsh::{Harsh, HarshBuilder};
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::panic::Location;
use wasm_bindgen::JsCast;

use lazy_static::*;
use seed_style_macros::{create_pseudos, generate_short_f_names};
mod css_values;
pub use css_values::*;
pub mod measures;
pub use measures::*;
mod theme;
pub use theme::*;

pub mod presets;
pub use presets::style_presets;
pub use presets::*;

mod from_traits;
pub use from_traits::*;

use std::collections::HashMap;

lazy_static! {
    pub static ref S: Style = Style { ..Style::default() };
}

thread_local! {
    pub static STYLES_IN_ELEM: RefCell<Vec<Style>> = RefCell::new(vec![]);
    pub static STYLES_USED : RefCell<HashSet<u64>> = RefCell::new(HashSet::<u64>::new());
    pub static HASH_IDS_GENERATOR: RefCell<Harsh> = RefCell::new(HarshBuilder::new().init().unwrap());
}

fn short_uniq_id(id: u64) -> String {
    HASH_IDS_GENERATOR.with(|h| h.borrow().encode(&[id]).unwrap())
}

#[derive(Clone, Debug)]
pub struct Rule {
    value: CssValue,
}

impl Rule {
    fn render(&self) -> String {
        format!("{};\n", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    media_rules: HashMap<String, Vec<Rule>>,
    rules: Vec<Rule>,
    updated_at: Vec<String>,
    pseudo: Pseudo,
    media: Option<String>,
    name: String,
    keyframes: Keyframes,
    combinator: Option<Combinator>,
}

impl Default for Style {
    #[track_caller]
    fn default() -> Self {
        Style {
            media_rules: HashMap::new(),
            pseudo: Pseudo::None,
            updated_at: vec![format!("{}", Location::caller())],
            rules: vec![],
            name: "".to_string(),
            media: None,
            keyframes: Keyframes::default(),
            combinator: None,
        }
    }
}

pub trait UpdateStyle<T> {
    fn update_style(self, style: &Style) -> Style;
}

// pub trait OverloadUpdateStyle<T>{
//     fn update_st<Q>(self, val:Q) -> Style;
// }

impl Style {
    //Accessor Shorthand

    #[track_caller]
    pub fn padding_x<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pl: CssPaddingLeft = space.clone().into();
        let pr: CssPaddingRight = space.into();

        self.padding_left(pl).padding_right(pr)
    }
    #[track_caller]
    pub fn padding_y<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pt: CssPaddingTop = space.clone().into();
        let pb: CssPaddingBottom = space.into();

        self.padding_top(pt).padding_bottom(pb)
    }
    #[track_caller]
    pub fn margin_x<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pl: CssMarginLeft = space.clone().into();
        let pr: CssMarginRight = space.into();

        self.margin_left(pl).margin_right(pr)
    }
    #[track_caller]
    pub fn margin_y<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pt: CssMarginTop = space.clone().into();
        let pb: CssMarginBottom = space.into();

        self.margin_top(pt).margin_bottom(pb)
    }

    #[track_caller]
    pub fn px<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pl: CssPaddingLeft = space.clone().into();
        let pr: CssPaddingRight = space.into();

        self.padding_left(pl).padding_right(pr)
    }
    #[track_caller]
    pub fn py<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pt: CssPaddingTop = space.clone().into();
        let pb: CssPaddingBottom = space.into();

        self.padding_top(pt).padding_bottom(pb)
    }
    #[track_caller]
    pub fn mx<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pl: CssMarginLeft = space.clone().into();
        let pr: CssMarginRight = space.into();

        self.margin_left(pl).margin_right(pr)
    }
    #[track_caller]
    pub fn my<T>(&self, val: T) -> Style
    where
        T: Clone + Into<CssSpace>,
    {
        let space = val.into();

        let pt: CssMarginTop = space.clone().into();
        let pb: CssMarginBottom = space.into();

        self.margin_top(pt).margin_bottom(pb)
    }

    generate_short_f_names!([
        ("m", "Margin"),
        ("ml", "MarginLeft"),
        ("mr", "MarginRight"),
        ("mt", "MarginTop"),
        ("mb", "MarginBottom"),
        ("p", "Padding"),
        ("pl", "PaddingLeft"),
        ("pr", "PaddingRight"),
        ("pt", "PaddingTop"),
        ("pb", "PaddingBottom"),
        ("b_width", "BorderWidth"),
        ("bl_width", "BorderLeftWidth"),
        ("br_width", "BorderRightWidth"),
        ("bt_width", "BorderTopWidth"),
        ("bb_width", "BorderBottomWidth"),
        ("b_color", "BorderColor"),
        ("bl_color", "BorderLeftColor"),
        ("br_color", "BorderRightColor"),
        ("bt_color", "BorderTopColor"),
        ("bb_color", "BorderBottomColor"),
        ("b_style", "BorderStyle"),
        ("bl_style", "BorderLeftStyle"),
        ("br_style", "BorderRightStyle"),
        ("bt_style", "BorderTopStyle"),
        ("bb_style", "BorderBottomStyle"),
        ("bg_color", "BackgroundColor"),
        ("bg_image", "BackgroundImage"),
        ("bg_position", "BackgroundPosition"),
        ("bg_repeat", "BackgroundRepeat"),
        ("bg_size", "BackgroundSize"),
        ("w", "Width"),
        ("h", "Height"),
        ("min_h", "MinHeight"),
        ("min_w", "MinWidth"),
        ("max_h", "MaxHeight"),
        ("max_w", "MaxWidth"),
        ("pos", "Position"),
        ("radius", "BorderRadius"),
    ]);

    #[track_caller]
    pub fn css(&self, val: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.add_rule(CssValue::StringValue(val.to_string()));
        new_style
    }

    //
    // Display
    //

    #[track_caller]
    pub fn add_style<T>(&self, val: T) -> Style
    where
        T: Into<Style>,
    {
        let val = val.into();

        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        for rule in &val.rules {
            new_style.add_rule(rule.value.clone())
        }
        new_style
    }

    fn add_rule(&mut self, value: CssValue) {
        self.rules.push(Rule {
            // property,
            value,
        });
    }

    // The below macro creates psuedo matods like this one...
    // #[track_caller]
    // pub fn hover(&self) -> Style {
    //     let mut new_style = self.clone();
    //     new_style.updated_at.push(format!("{}", Location::caller()));
    //     new_style.pseudo = Pseudo::Hover;
    //     new_style
    // }
    create_pseudos!([
        "None",
        "Active",
        "Checked",
        "Disabled",
        "Empty",
        "Enabled",
        "FirstChild",
        "FirstOfType",
        "Focus",
        "Hover",
        "InRange",
        "Invalid",
        "LastChild",
        "LastOfType",
        "Link",
        "OnlyOfType",
        "OnlyChild",
        "Optional",
        "OutOfRange",
        "ReadOnly",
        "ReadWrite",
        "Required",
        "Root",
        "Target",
        "Valid",
        "Visited",
    ]);

    #[track_caller]
    pub fn name(&self, name: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.name = name.to_string();
        new_style
    }

    #[track_caller]
    pub fn keyframe(&self, key: i32, to: Style) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.keyframes.frames.push((key, Box::new(to)));
        new_style
    }

    #[track_caller]
    pub fn media(&self, val: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.media = Some(val.to_string());
        new_style
    }

    #[track_caller]
    pub fn adjacent_follows(&self, val: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.combinator = Some(Combinator::Post(PostCombinator::AdjacentSiblingFollows(
            val.to_string(),
        )));
        new_style
    }

    #[track_caller]
    pub fn general_follows(&self, val: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.combinator = Some(Combinator::Post(PostCombinator::GeneralSibingFollows(
            val.to_string(),
        )));
        new_style
    }

    #[track_caller]
    pub fn is_direct_child_of(&self, val: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.combinator = Some(Combinator::Post(PostCombinator::IsDirectChildOf(
            val.to_string(),
        )));
        new_style
    }

    #[track_caller]
    pub fn is_child_of(&self, val: &str) -> Style {
        let mut new_style = self.clone();
        new_style.updated_at.push(format!("{}", Location::caller()));
        new_style.combinator = Some(Combinator::Post(PostCombinator::IsChildOf(val.to_string())));
        new_style
    }

    pub fn only_and_below<T>(&self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(|borrowed_themes| {
            borrowed_themes
                .iter()
                .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp.clone()))
                .unwrap()
        });

        match bp_pair {
            (_lower, Some(higher)) => self.media(&format!("@media (max-width: {}px)", higher - 1)),
            (_lower, None) => self.clone(),
        }
    }

    pub fn only<T>(&self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(|borrowed_themes| {
            borrowed_themes
                .iter()
                .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp.clone()))
                .unwrap()
        });

        match bp_pair {
            (lower, Some(higher)) => self.media(&format!(
                "@media (min-width:{}px) and (max-width: {}px)",
                lower,
                higher - 1
            )),
            (lower, None) => self.media(&format!("@media (min-width:{}px)", lower)),
        }
    }

    pub fn only_and_above<T>(&self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(|borrowed_themes| {
            borrowed_themes
                .iter()
                .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp.clone()))
                .unwrap()
        });

        match bp_pair {
            (lower, Some(_higher)) => self.media(&format!("@media (min-width:{}px)", lower)),
            (lower, None) => self.media(&format!("@media (min-width:{}px)", lower)),
        }
    }

    pub fn except<T>(&self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(|borrowed_themes| {
            borrowed_themes
                .iter()
                .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp.clone()))
                .unwrap()
        });

        match bp_pair {
            (lower, Some(higher)) => self.media(&format!(
                "@media (max-width:{}px) or (min-width: {}px)",
                lower - 1,
                higher
            )),
            (lower, None) => self.media(&format!("@media (max-width:{}px)", lower - 1)),
        }
    }

    pub fn render(&self) -> String {
        let mut style = "".to_string();

        style.push_str(&format!(
            "    /* Defined at {} */\n",
            self.updated_at.last().unwrap()
        ));

        for rule in &self.rules {
            match &rule.value {
                CssValue::StringValue(val) => style.push_str(&format!("{}\n", val)),
                _ => style.push_str(&rule.render()),
            }
        }

        style
    }
}

#[derive(Display, Clone, Debug)]
pub enum Combinator {
    Pre(PreCombinator),
    Post(PostCombinator),
}

#[derive(Display, Clone, Debug)]
pub enum PostCombinator {
    #[display(fmt = "{} + ", _0)]
    AdjacentSiblingFollows(String),
    #[display(fmt = "{} ~ ", _0)]
    GeneralSibingFollows(String),
    #[display(fmt = "{} > ", _0)]
    IsDirectChildOf(String),
    #[display(fmt = "{} ", _0)]
    IsChildOf(String),
}

#[derive(Display, Clone, Debug)]
pub enum PreCombinator {
    #[display(fmt = " + {}", _0)]
    AdjacentSiblingPreceeds(String),
    #[display(fmt = " ~ {}", _0)]
    GeneralSibingPreceeds(String),
    #[display(fmt = " > {}", _0)]
    IsDirectParentOf(String),
    #[display(fmt = " {}", _0)]
    IsParentOf(String),
}

#[derive(Clone, Debug)]
pub enum Pseudo {
    None,
    Active,
    Checked,
    Disabled,
    Empty,
    Enabled,
    FirstChild,
    FirstOfType,
    Focus,
    Hover,
    InRange,
    Invalid,
    Lang(String),
    LastChild,
    LastOfType,
    Link,
    Not(String),
    NthChild(usize),
    NthLastChild(usize),
    NthLastOfType(usize),
    NthOfType(usize),
    OnlyOfType,
    OnlyChild,
    Optional,
    OutOfRange,
    ReadOnly,
    ReadWrite,
    Required,
    Root,
    Target,
    Valid,
    Visited,
}

impl Pseudo {
    fn render(&self) -> String {
        match self {
            Pseudo::None => "",
            Pseudo::Active => ":active",
            Pseudo::Checked => ":checked",
            Pseudo::Disabled => ":disabled",
            Pseudo::Empty => ":empty",
            Pseudo::Enabled => ":enabled",
            Pseudo::FirstChild => ":first-child",
            Pseudo::FirstOfType => ":first-of-type",
            Pseudo::Focus => ":focus",
            Pseudo::Hover => ":hover",
            Pseudo::InRange => ":in-range",
            Pseudo::Invalid => ":invalid",
            Pseudo::Lang(_xxx) => ":langXXX",
            Pseudo::LastChild => ":last-child",
            Pseudo::LastOfType => ":last-of-type",
            Pseudo::Link => ":link",
            Pseudo::Not(_xxx) => ":notXXX",
            Pseudo::NthChild(_xxx) => ":nth-childXXX",
            Pseudo::NthLastChild(_xxx) => ":nth-last-childXXX",
            Pseudo::NthLastOfType(_xxx) => ":nth-last-of-typeXXX",
            Pseudo::NthOfType(_xxx) => ":nth-of-typeXXX",
            Pseudo::OnlyOfType => ":only-of-type",
            Pseudo::OnlyChild => ":only-child",
            Pseudo::Optional => ":optional",
            Pseudo::OutOfRange => ":out-of-range",
            Pseudo::ReadOnly => ":read-only",
            Pseudo::ReadWrite => ":read-write",
            Pseudo::Required => ":required",
            Pseudo::Root => ":root",
            Pseudo::Target => ":target",
            Pseudo::Valid => ":valid",
            Pseudo::Visited => ":visited",
        }
        .to_string()
    }
}

#[derive(Default, Clone, Debug)]
pub struct Keyframes {
    frames: Vec<(i32, Box<Style>)>,
}

impl Keyframes {
    fn render(&self) -> String {
        let mut rendered = "".to_string();

        for frame in &self.frames {
            rendered.push_str(&format!("{}% {{\n{}}}", frame.0, frame.1.render()));
        }
        rendered
    }
}

pub trait LocalUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms> LocalUpdateEl<El<Ms>> for Style {
    fn update_el(self, el: &mut El<Ms>) {
        // log!(el.attrs);
        let rendered_css = self.render();

        let do_it = if let Some(AtValue::Some(class_string)) = el.attrs.vals.get(&At::Class) {
            class_string.find("seedstyle-").is_some()
        } else {
            false
        };

        if do_it {
            let styles_in_elem = STYLES_IN_ELEM.with(|styles| {
                styles.borrow_mut().push(self.clone());
                styles.borrow().clone()
            });

            let vec_of_rendered_css = styles_in_elem
                .iter()
                .map(|s| s.render())
                .collect::<Vec<String>>();

            let mut s = DefaultHasher::new();
            vec_of_rendered_css.hash(&mut s);
            let revised_variant_hash = s.finish();

            let css_aleady_created = STYLES_USED
                .with(|css_set_ref| css_set_ref.borrow().contains(&revised_variant_hash));

            if !css_aleady_created {
                add_css_to_head_unchecked(&rendered_css, revised_variant_hash, &self, &self.name);
            }
            let short_hash = format!("{}{}", &self.name, short_uniq_id(revised_variant_hash));
            let class_name = format!("seedstyle-{}", short_hash);
            C![class_name].update_el(el);
        } else {
            STYLES_IN_ELEM.with(|styles| {
                styles.borrow_mut().clear();
                styles.borrow_mut().push(self.clone());
            });

            let variant_hash = hash_64(&rendered_css, &self.updated_at);

            let class_name = format!(
                "seedstyle-{}",
                add_css_to_head(&rendered_css, variant_hash, &self)
            );

            C![class_name].update_el(el);
        }
    }
}

impl<Ms> LocalUpdateEl<El<Ms>> for &[Style] {
    fn update_el(self, el: &mut El<Ms>) {
        let vec_of_rendered_css = self.iter().map(|s| s.render()).collect::<Vec<String>>();

        let mut s = DefaultHasher::new();
        vec_of_rendered_css.hash(&mut s);
        let variant_hash = s.finish();

        let css_aleady_created =
            STYLES_USED.with(|css_set_ref| css_set_ref.borrow().contains(&variant_hash));

        let mut name = "".to_string();
        for (_, style) in vec_of_rendered_css.iter().zip(self.iter()) {
            name.push_str(&style.name);
        }

        if !css_aleady_created {
            for (rendered_css, style) in vec_of_rendered_css.iter().zip(self.iter()) {
                add_css_to_head_unchecked(&rendered_css, variant_hash, style, &name);
            }
        }
        let short_hash = format!("{}{}", name, short_uniq_id(variant_hash));
        let class_name = format!("seedstyle-{}", short_hash);

        C![class_name].update_el(el);
    }
}

fn hash_64<T: AsRef<str> + Hash>(css: &str, locations: &[T]) -> u64 {
    let mut s = DefaultHasher::new();
    (css, locations).hash(&mut s);
    s.finish()
}

fn add_css_to_head(css: &str, variant_hash: u64, style: &Style) -> String {
    let css_aleady_created =
        STYLES_USED.with(|css_set_ref| css_set_ref.borrow().contains(&variant_hash));
    let short_hash = format!("{}{}", style.name, short_uniq_id(variant_hash));

    if !css_aleady_created {
        add_css_to_head_unchecked(css, variant_hash, style, &style.name);
    }
    short_hash
}

fn add_css_to_head_unchecked(css: &str, variant_hash: u64, style: &Style, name: &str) -> String {
    let short_hash = format!("{}{}", name, short_uniq_id(variant_hash));

    let head_elem = document().get_elements_by_tag_name("head").item(0).unwrap();

    let css = if !style.keyframes.frames.is_empty() {
        format!("{}    animation-name: anim-{};\n", css, short_hash)
    } else {
        css.to_string()
    };

    let mut full_css = match (&style.media, &style.combinator) {
        (Some(media), Some(Combinator::Pre(c))) => format!(
            "{}{{\n.seedstyle-{}{}{}{{\n{}}}}}\n",
            media,
            short_hash,
            c,
            style.pseudo.render(),
            css
        ),
        (Some(media), Some(Combinator::Post(c))) => format!(
            "{}{{\n{}.seedstyle-{}{}{{\n{}}}}}\n",
            media,
            c,
            short_hash,
            style.pseudo.render(),
            css
        ),
        (Some(media), None) => format!(
            "{}{{\n.seedstyle-{}{}{{\n{}}}}}\n",
            media,
            short_hash,
            style.pseudo.render(),
            css
        ),
        (None, Some(Combinator::Pre(c))) => format!(
            "\n.seedstyle-{}{}{}{{\n{}}}\n",
            short_hash,
            c,
            style.pseudo.render(),
            css
        ),
        (None, Some(Combinator::Post(c))) => format!(
            "\n{}.seedstyle-{}{}{{\n{}}}\n",
            c,
            short_hash,
            style.pseudo.render(),
            css
        ),

        (None, None) => format!(
            "\n.seedstyle-{}{}{{\n{}}}\n",
            short_hash,
            style.pseudo.render(),
            css
        ),
    };

    for (media_breakpoint, rule_vec) in &style.media_rules {
        full_css.push_str(&format!(
            "{}{{\n.seedstyle-{}{{\n",
            media_breakpoint, short_hash
        ));

        for rule in rule_vec {
            match &rule.value {
                CssValue::StringValue(val) => full_css.push_str(&format!("{}\n", val)),
                _ => full_css.push_str(&rule.render()),
            }
        }

        full_css.push_str("}\n}\n");
    }

    if !style.keyframes.frames.is_empty() {
        full_css.push_str(&format!(
            "\n\n@keyframes anim-{}{{ \n  {}  \n}}\n",
            short_hash,
            style.keyframes.render()
        ));
    }

    if let Some(style_elem) = head_elem.get_elements_by_tag_name("style").item(0) {
        let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
        let existing_style_content = style_elem.inner_html();

        let new_style_content = format!("{}\n{}", existing_style_content, full_css);
        style_elem.set_inner_html(&new_style_content);
    } else {
        let style_elem = document()
            .create_element("style")
            .unwrap()
            .dyn_into::<web_sys::HtmlStyleElement>()
            .unwrap();
        style_elem.set_inner_html(&full_css);
        let _ = head_elem.append_child(&style_elem);
    }
    STYLES_USED.with(|css_set_ref| css_set_ref.borrow_mut().insert(variant_hash));

    short_hash
}
