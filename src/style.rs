use derive_more::Display;
use harsh::{Harsh, HarshBuilder};
use seed::{prelude::*, *};
use seed_hooks::*;

use std::cell::{Cell, RefCell};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::panic::Location;
use wasm_bindgen::JsCast;

pub mod css_values;
pub use css_values::*;


pub mod row_col_layout;
pub use row_col_layout::*;

pub mod measures;

pub mod theme;
use theme::*;

pub mod composition;

pub mod layout;

pub mod helpers;

pub mod presets;
use seed_style_macros::CssPseudoMacro;

use presets::*;

mod from_traits;

use std::collections::HashMap;

pub fn s() -> Style {

    Style::default()
}

thread_local! {
    pub static GLOBAL_STYLES_COUNT: Cell<u32> = Cell::new(0);
    
    pub static STYLES_USED : RefCell<HashSet<u64>> = RefCell::new(HashSet::<u64>::new());
    pub static HASH_IDS_GENERATOR: RefCell<Harsh> = RefCell::new(HarshBuilder::new().init().unwrap());
}

fn short_uniq_id(id: u64) -> String {
    HASH_IDS_GENERATOR.with(|h| h.borrow().encode(&[id]).unwrap())
}

use objekt_clonable::*;
#[clonable]
pub trait CssValueTrait: std::fmt::Display + Clone + Sync + Send + std::fmt::Debug {
    fn prefixes(&self) -> Option<Vec<String>> {
        None
    }

    fn value_only(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub value: Box<dyn CssValueTrait>,
}

impl Rule {
    fn render(&self) -> String {
        if let Some(prefixes) = &self.value.prefixes() {
            let mut rendered = format!("{}\n", self.value);
            for prefix in prefixes {
                rendered.push_str(&format!("{}{}\n", prefix, self.value));
            }
            rendered
        } else {
            format!("{}\n", self.value)
        }
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
    pre_combinators: Vec<Combinator>,
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
            pre_combinators: vec![],
        }
    }
}

pub trait UpdateStyle<T> {
    fn update_style(self, style: &mut Style);
}

impl<'a, P> UpdateStyle<P> for &'a str
where
    P: 'static + Clone + CssValueTrait,
    &'a str: Into<P>,
{
    fn update_style(self, style: &mut Style) {
        let val: P = self.into();
        style.add_rule(Box::new(val));
    }
}

impl<P> UpdateStyle<P> for P
where
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        let val: P = self.into();
        style.add_rule(Box::new(val));
    }
}

struct ReturnBpScale;

impl ActOnIteratorOfThemes<Option<Vec<CssMedia>>> for ReturnBpScale {
    fn call<'a, It>(&self,  it: It) -> Option<Vec<CssMedia>>
    where
        It: DoubleEndedIterator<Item = &'a Theme>,
    {
        it.rev().find_map(|theme| {
            if !theme.media_bp_scale.is_empty() {
                Some(theme.media_bp_scale.clone())
            } else {
                None
            }
        })
    }
}

impl<R, P> UpdateStyle<P> for &[R]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        if let Some(bp_scale) = with_themes(ReturnBpScale) {
            let mut old_style = None;

            for (style_idx, bp) in bp_scale.iter().enumerate() {
                if let Some(item) = self.get(style_idx) {
                    let specific_value: P = item.clone().into();

                    let rules = style.media_rules.entry(bp.clone().0).or_insert(vec![]);
                    rules.push(Rule {
                        value: Box::new(specific_value.clone()),
                    });

                    old_style = Some(specific_value);
                } else {
                    let rules = style.media_rules.entry(bp.clone().0).or_insert(vec![]);
                    rules.push(Rule {
                        value: Box::new(old_style.clone().unwrap()),
                    });
                }
            }
        } else {
            panic!("No breakpoints have been defined!")
        };
    }
}
impl<R, P> UpdateStyle<P> for &[R; 2]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}

impl<R, P> UpdateStyle<P> for &[R; 3]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}

impl<R, P> UpdateStyle<P> for &[R; 4]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}
impl<R, P> UpdateStyle<P> for &[R; 5]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}

impl<R, P> UpdateStyle<P> for &[R; 6]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}
impl<R, P> UpdateStyle<P> for &[R; 7]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}

impl<R, P> UpdateStyle<P> for &[R; 8]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}
impl<R, P> UpdateStyle<P> for &[R; 9]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}
impl<R, P> UpdateStyle<P> for &[R; 10]
where
    R: Into<P> + Clone,
    P: 'static + Clone + CssValueTrait,
{
    fn update_style(self, style: &mut Style) {
        self.as_ref().update_style(style)
    }
}
// pub trait OverloadUpdateStyle<T>{
//     fn update_st<Q>(self, val:Q) -> Style;
// }

impl Style {
    //Accessor Shorthand

    #[track_caller]
    pub fn padding_x<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssPaddingLeft> + UpdateStyle<CssPaddingRight>,
    {
        self.padding_left(val.clone()).padding_right(val)
    }
    #[track_caller]
    pub fn padding_y<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssPaddingTop> + UpdateStyle<CssPaddingBottom>,
    {
        self.padding_top(val.clone()).padding_bottom(val)
    }
    #[track_caller]
    pub fn margin_x<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssMarginLeft> + UpdateStyle<CssMarginRight>,
    {
        // let space = val.into();

        // let pl: CssMarginLeft = space.clone().into();
        // let pr: CssMarginRight = space.into();

        self.margin_left(val.clone()).margin_right(val)
    }
    #[track_caller]
    pub fn margin_y<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssMarginTop> + UpdateStyle<CssMarginBottom>,
    {
        // let space = val.into();

        // let pt: CssMarginTop = space.clone().into();
        // let pb: CssMarginBottom = space.into();

        self.margin_top(val.clone()).margin_bottom(val)
    }

    #[track_caller]
    pub fn px<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssPaddingLeft> + UpdateStyle<CssPaddingRight>,
    {
        // let space = val.into();

        // let pl: CssPaddingLeft = space.clone().into();
        // let pr: CssPaddingRight = space.into();

        self.padding_left(val.clone()).padding_right(val)
    }
    #[track_caller]
    pub fn py<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssPaddingTop> + UpdateStyle<CssPaddingBottom>,
    {
        // let space = val.into();

        // let pt: CssPaddingTop = space.clone().into();
        // let pb: CssPaddingBottom = space.into();

        self.padding_top(val.clone()).padding_bottom(val)
    }
    #[track_caller]
    pub fn mx<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssMarginLeft> + UpdateStyle<CssMarginRight>,
    {
        // let space = val.into();

        // let pl: CssMarginLeft = space.clone().into();
        // let pr: CssMarginRight = space.into();

        self.margin_left(val.clone()).margin_right(val)
    }

    #[track_caller]
    pub fn mx_auto(self) -> Style {
        // let space = val.into();

        // let pl: CssMarginLeft = space.clone().into();
        // let pr: CssMarginRight = space.into();

        self.margin_left_auto().margin_right_auto()
    }

    #[track_caller]
    pub fn my_auto(self) -> Style {
        // let space = val.into();

        // let pl: CssMarginLeft = space.clone().into();
        // let pr: CssMarginRight = space.into();

        self.margin_top_auto().margin_bottom_auto()
    }

    #[track_caller]
    pub fn margin_x_auto(self) -> Style {
        // let space = val.into();

        // let pl: CssMarginLeft = space.clone().into();
        // let pr: CssMarginRight = space.into();

        self.margin_left_auto().margin_right_auto()
    }

    #[track_caller]
    pub fn margin_y_auto(self) -> Style {
        // let space = val.into();

        // let pl: CssMarginLeft = space.clone().into();
        // let pr: CssMarginRight = space.into();

        self.margin_top_auto().margin_bottom_auto()
    }

    #[track_caller]
    pub fn my<T>(self, val: T) -> Style
    where
        T: Clone + UpdateStyle<CssMarginTop> + UpdateStyle<CssMarginBottom>,
    {
        // let space = val.into();

        // let pt: CssMarginTop = space.clone().into();
        // let pb: CssMarginBottom = space.into();

        self.margin_top(val.clone()).margin_bottom(val)
    }

    // #[track_caller]
    // pub fn add_style<T>(mut self, val: T) -> Style
    // where
    //     T: Into<Style>,
    // {
    //     let val = val.into();

    //     self.updated_at.push(format!("{}", Location::caller()));
    //     for rule in &val.rules {
    //         self.add_rule(rule.value.clone())
    //     }
    //     self
    // }

    fn add_rule(&mut self, value: Box<dyn CssValueTrait>) {
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
    // create_pseudos!([
    //     "None",
    //     "Active",
    //     "Checked",
    //     "Disabled",
    //     "Empty",
    //     "Enabled",
    //     "FirstChild",
    //     "FirstOfType",
    //     "Focus",
    //     "Hover",
    //     "InRange",
    //     "Invalid",
    //     "LastChild",
    //     "LastOfType",
    //     "Link",
    //     "OnlyOfType",
    //     "OnlyChild",
    //     "Optional",
    //     "OutOfRange",
    //     "ReadOnly",
    //     "ReadWrite",
    //     "Required",
    //     "Root",
    //     "Target",
    //     "Valid",
    //     "Visited",
    // ]);

    #[track_caller]
    pub fn name(mut self, name: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.name = name.to_string();
        self
    }

    #[track_caller]
    pub fn keyframe(mut self, key: i32, to: Style) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.keyframes.frames.push((key, Box::new(to)));
        self
    }

    #[track_caller]
    pub fn media(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.media = Some(val.to_string());
        self
    }

    #[track_caller]
    pub fn follows(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.combinator = Some(Combinator::Post(PostCombinator::AdjacentSiblingFollows(
            val.to_string(),
        )));
        self
    }

    #[track_caller]
    pub fn sibling_of(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.combinator = Some(Combinator::Post(PostCombinator::GeneralSibingFollows(
            val.to_string(),
        )));
        self
    }

    #[track_caller]
    pub fn child_of(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.combinator = Some(Combinator::Post(PostCombinator::IsDirectChildOf(
            val.to_string(),
        )));
        self
    }

    #[track_caller]
    pub fn descendant_of(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.combinator = Some(Combinator::Post(PostCombinator::IsChildOf(val.to_string())));
        self
    }

    #[track_caller]
    pub fn style_following(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![Combinator::Pre(PreCombinator::AdjacentSiblingPreceeds(
            val.to_string(),
        ))];
        self
    }

    #[track_caller]
    pub fn style_sibling(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![Combinator::Pre(PreCombinator::GeneralSibingPreceeds(
            val.to_string(),
        ))];
        self
    }

    #[track_caller]
    pub fn style_child(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![Combinator::Pre(PreCombinator::IsDirectParentOf(
            val.to_string(),
        ))];
        self
    }

    #[track_caller]
    pub fn style_other(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![Combinator::Pre(PreCombinator::CombinatorForOther(
            val.to_string(),
        ))];
        self
    }

    #[track_caller]
    pub fn style_descendant(mut self, val: &str) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![Combinator::Pre(PreCombinator::IsParentOf(val.to_string()))];
        self
    }

    #[track_caller]
    pub fn style_siblings(mut self, siblings: &[&str]) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![];
        for sibling in siblings.iter() {
            self.pre_combinators
                .push(Combinator::Pre(PreCombinator::GeneralSibingPreceeds(
                    sibling.to_string(),
                )));
        }
        self
    }

    #[track_caller]
    pub fn style_children(mut self, children: &[&str]) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));

        self.pre_combinators = vec![];
        for child in children.iter() {
            self.pre_combinators
                .push(Combinator::Pre(PreCombinator::IsDirectParentOf(
                    child.to_string(),
                )));
        }

        self
    }

    #[track_caller]
    pub fn style_descendants(mut self, descendants: &[&str]) -> Style {
        self.updated_at.push(format!("{}", Location::caller()));
        self.pre_combinators = vec![];
        for descendant in descendants.iter() {
            self.pre_combinators
                .push(Combinator::Pre(PreCombinator::IsParentOf(
                    descendant.to_string(),
                )));
        }
        self
    }

    pub fn only_and_below<T>(self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(ReturnBpTuple(bp));

        match bp_pair {
            (_lower, Some(higher)) => self.media(&format!("@media (max-width: {}px)", higher - 1)),
            (_lower, None) => self.clone(),
        }
    }

    pub fn only<T>(self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(ReturnBpTuple(bp));

        match bp_pair {
            (lower, Some(higher)) => self.media(&format!(
                "@media (min-width:{}px) and (max-width: {}px)",
                lower,
                higher - 1
            )),
            (lower, None) => self.media(&format!("@media (min-width:{}px)", lower)),
        }
    }

    pub fn only_and_above<T>(self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(ReturnBpTuple(bp));

        match bp_pair {
            (lower, Some(_higher)) => self.media(&format!("@media (min-width:{}px)", lower)),
            (lower, None) => self.media(&format!("@media (min-width:{}px)", lower)),
        }
    }

    pub fn except<T>(self, bp: T) -> Style
    where
        T: BreakpointTheme + 'static,
    {
        let bp_pair = with_themes(ReturnBpTuple(bp));

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

        // style.push_str(&format!(
        //     "    /* Defined at {} */\n",
        //     self.updated_at.last().unwrap()
        // ));

        for rule in &self.rules {
            style.push_str(&rule.render());
        }

        style
    }

    #[track_caller]
    pub fn custom_style<T>(mut self, val: T) -> Style
    where
        T: UpdateCustomStyle,
    {
        self.updated_at.push(format!("{}", Location::caller()));
        val.update_style(&mut self);
        self
    }
}

pub trait UpdateCustomStyle {
    fn update_style(self, style: &mut Style);
}

impl UpdateCustomStyle for Style {
    fn update_style(self, style: &mut Style) {
        for rule in &self.rules {
            style.add_rule(rule.value.clone());
        }

        for (key, value) in &self.media_rules {
            style.media_rules.insert(key.clone(), value.clone());
        }

        style.pseudo = self.pseudo.clone();

        style.media = self.media.clone();

        style.name = self.name.clone();

        style.keyframes = self.keyframes.clone();

        style.combinator = self.combinator.clone();

        style.pre_combinators = self.pre_combinators.clone();
    }
}

struct ReturnSpecificStyleFromStyleTheme<T: StyleTheme + 'static>(T);

impl<Th> UpdateCustomStyle for Th
where
    Th: StyleTheme + 'static,
{
    fn update_style(self, style: &mut Style)
    where
        Th: StyleTheme + 'static,
    {
        let theme_style = with_themes(ReturnSpecificStyleFromStyleTheme(self)).unwrap();
        theme_style.update_style(style);
    }
}

impl<T> ActOnIteratorOfThemes<Option<Style>> for ReturnSpecificStyleFromStyleTheme<T>
where
    T: StyleTheme + 'static,
{
    fn call<'a, It>(&self,  it: It) -> Option<Style>
    where
        It: DoubleEndedIterator<Item = &'a Theme>,
    {
        it.rev().find_map(|theme| theme.get::<T, Style>(self.0.clone()))
    }
}

struct ReturnBpTuple<T>(T)
where
    T: BreakpointTheme + 'static;

impl<T> ActOnIteratorOfThemes<(u32, Option<u32>)> for ReturnBpTuple<T>
where
    T: BreakpointTheme + 'static,
{
    fn call<'a, It>(&self,  it: It) -> (u32, Option<u32>)
    where
        It: DoubleEndedIterator<Item = &'a Theme>,
    {
        it.rev().find_map(|theme| theme.get::<T, (u32, Option<u32>)>(self.0.clone()))
            .unwrap()
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
    #[display(fmt = "{}", _0)]
    CombinatorForOther(String),
}

#[derive(Clone, Debug, CssPseudoMacro)]
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
    LastChild,
    LastOfType,
    Link,
    Lang(String),
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
    Before,
    After,
    Custom(String),
}

impl Pseudo {
    fn render(&self) -> String {
        match self {
            Pseudo::None => "".to_string(),
            Pseudo::Active => ":active".to_string(),
            Pseudo::Checked => ":checked".to_string(),
            Pseudo::Disabled => ":disabled".to_string(),
            Pseudo::Empty => ":empty".to_string(),
            Pseudo::Enabled => ":enabled".to_string(),
            Pseudo::FirstChild => ":first-child".to_string(),
            Pseudo::FirstOfType => ":first-of-type".to_string(),
            Pseudo::Focus => ":focus".to_string(),
            Pseudo::Hover => ":hover".to_string(),
            Pseudo::InRange => ":in-range".to_string(),
            Pseudo::Invalid => ":invalid".to_string(),
            Pseudo::LastChild => ":last-child".to_string(),
            Pseudo::LastOfType => ":last-of-type".to_string(),
            Pseudo::Link => ":link".to_string(),
            Pseudo::OnlyOfType => ":only-of-type".to_string(),
            Pseudo::OnlyChild => ":only-child".to_string(),
            Pseudo::Optional => ":optional".to_string(),
            Pseudo::OutOfRange => ":out-of-range".to_string(),
            Pseudo::ReadOnly => ":read-only".to_string(),
            Pseudo::ReadWrite => ":read-write".to_string(),
            Pseudo::Required => ":required".to_string(),
            Pseudo::Root => ":root".to_string(),
            Pseudo::Target => ":target".to_string(),
            Pseudo::Valid => ":valid".to_string(),
            Pseudo::Visited => ":visited".to_string(),
            Pseudo::Before  => "::before".to_string(),
            Pseudo::After => "::after".to_string(),
            Pseudo::Lang(val) => format!(":lang({})", val),
            Pseudo::Not(val) => format!(":not({})", val),
            Pseudo::NthChild(val) => format!(":nth-child({})", val),
            Pseudo::NthLastChild(val) => format!(":nth-last-chid({})", val),
            Pseudo::NthLastOfType(val) => format!(":nth-last-of-type({})", val),
            Pseudo::NthOfType(val) => format!(":nth-of-type({})", val),
            Pseudo::Custom(val) => format!("{}", val),
        }
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

// #[derive(Hash)]
//     struct GlobalStyle {
//         style: Style,
//         global: String,
//     }

fn add_global_init_css_to_head(
    css: &str,
    short_hash: &str,
    style: &Style,
    global_classname: &str,
    selector: &str,
) {
    let head_elem = document().get_elements_by_tag_name("head").item(0).unwrap();

    let css = if !style.keyframes.frames.is_empty() {
        format!("{}    animation-name: anim-{};\n", css, short_hash)
    } else {
        css.to_string()
    };

    let pre_combinators_str = style
        .pre_combinators
        .iter()
        .map(|c| {
            if let Combinator::Pre(c) = c {
                format!(
                    ".{}{}{}{}",
                    global_classname,
                    selector,
                    c,
                    style.pseudo.render()
                )
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(",");

    let full_css = if style.pre_combinators.len() > 0 {
        match &style.media {
            Some(media) => format!("{}{{\n{}{{\n{}}}}}\n", media, pre_combinators_str, css),

            None => format!("\n{}{{\n{}}}\n", pre_combinators_str, css),
        }
    } else {
        match (&style.media, &style.combinator) {
            (Some(media), Some(Combinator::Pre(c))) => format!(
                "{}{{\n.{}{}{}{}{{\n{}}}}}\n",
                media,
                global_classname,
                selector,
                c,
                style.pseudo.render(),
                css
            ),
            (Some(media), Some(Combinator::Post(c))) => format!(
                "{}{{\n.{}{}{}{}{{\n{}}}}}\n",
                media,
                global_classname,
                c,
                selector,
                style.pseudo.render(),
                css
            ),
            (Some(media), None) => format!(
                "{}{{\n.{}{}{}{{\n{}}}}}\n",
                media,
                global_classname,
                selector,
                style.pseudo.render(),
                css
            ),
            (None, Some(Combinator::Pre(c))) => format!(
                "\n.{}{}{}{}{{\n{}}}\n",
                global_classname,
                selector,
                c,
                style.pseudo.render(),
                css
            ),
            (None, Some(Combinator::Post(c))) => format!(
                "\n.{}{}{}{}{{\n{}}}\n",
                global_classname,
                c,
                selector,
                style.pseudo.render(),
                css
            ),

            (None, None) => format!(
                "\n.{}{}{}{{\n{}}}\n",
                global_classname,
                selector,
                style.pseudo.render(),
                css
            ),
        }
    };

    // let full_css = match (&style.media, &style.combinator) {
    //     (Some(media), Some(Combinator::Pre(c))) => format!(
    //         "{}{{\n{}{}{}{{\n{}}}}}\n",
    //         media,
    //         selector,
    //         c,
    //         style.pseudo.render(),
    //         css
    //     ),
    //     (Some(media), Some(Combinator::Post(c))) => format!(
    //         "{}{{\n{}{}{}{{\n{}}}}}\n",
    //         media,
    //         c,
    //         selector,
    //         style.pseudo.render(),
    //         css
    //     ),
    //     (Some(media), None) => format!(
    //         "{}{{\n{}{}{{\n{}}}}}\n",
    //         media,
    //         selector,
    //         style.pseudo.render(),
    //         css
    //     ),
    //     (None, Some(Combinator::Pre(c))) => format!(
    //         "\n{}{}{}{{\n{}}}\n",
    //         selector,
    //         c,
    //         style.pseudo.render(),
    //         css
    //     ),
    //     (None, Some(Combinator::Post(c))) => format!(
    //         "\n{}{}{}{{\n{}}}\n",
    //         c,
    //         selector,
    //         style.pseudo.render(),
    //         css
    //     ),

    //     (None, None) => format!("\n{}{}{{\n{}}}\n", selector, style.pseudo.render(), css),
    // };

    let css_stylesheet =
        if let Some(style_elem) = head_elem.get_elements_by_tag_name("style").item(0) {
            let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
            style_elem
                .sheet()
                .unwrap()
                .dyn_into::<web_sys::CssStyleSheet>()
                .unwrap()

        // let existing_style_content = style_elem.inner_html();

        // let new_style_content = format!("{}\n{}", existing_style_content, full_css);
        // style_elem.set_inner_html(&new_style_content);
        } else {
            let style_elem = document()
                .create_element("style")
                .unwrap()
                .dyn_into::<web_sys::HtmlStyleElement>()
                .unwrap();

            // style_elem.set_inner_html(&full_css);

            let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
            let _ = head_elem.append_child(&style_elem);
            style_elem
                .sheet()
                .unwrap()
                .dyn_into::<web_sys::CssStyleSheet>()
                .unwrap()
        };

    let rules_length = GLOBAL_STYLES_COUNT.with(|count| count.get());
    
    // log!(full_css);
    let res = css_stylesheet.insert_rule_with_index(&full_css, rules_length);
    if let Err(err) = res {
        log!("error inserting style: " ,err, full_css);
    }

    GLOBAL_STYLES_COUNT.with(|count| {
        let mut c = count.get();
        c += 1;
        count.set(c);
    });

    for (media_breakpoint, rule_vec) in &style.media_rules {
        let mut media_string = String::new();
        media_string.push_str(&format!(
            "{}{{\n.{}{}{{\n",
            media_breakpoint, global_classname, selector
        ));

        for rule in rule_vec {
            media_string.push_str(&rule.render());
        }

        let rules_length = GLOBAL_STYLES_COUNT.with(|count| count.get());
        let res = css_stylesheet.insert_rule_with_index(&media_string, rules_length);
        if let Err(err) = res {
            log!("error inserting style: " ,err, media_string);
        }
        GLOBAL_STYLES_COUNT.with(|count| {
            let mut c = count.get();
            c += 1;
            count.set(c);
        });
    }

    if !style.keyframes.frames.is_empty() {
        let rules_length = GLOBAL_STYLES_COUNT.with(|count| count.get());
        let res = css_stylesheet.insert_rule_with_index(
            &format!(
                "\n\n@keyframes anim-{}{{ \n  {}  \n}}\n",
                short_hash,
                style.keyframes.render()
            ),
            rules_length,
        );
        if let Err(err) = res {
            log!("error inserting keyframes style: " ,err );
        }
        GLOBAL_STYLES_COUNT.with(|count| {
            let mut c = count.get();
            c += 1;
            count.set(c);
        });
    }
}

// fn add_global_css_to_head(css: &str, variant_hash: u64, style: &Style, name: &str) {

//     let short_hash = format!("{}{}", name, short_uniq_id(variant_hash));

//     let head_elem = document().get_elements_by_tag_name("head").item(0).unwrap();

//     let css = if !style.keyframes.frames.is_empty() {
//         format!("{}    animation-name: anim-{};\n", css, short_hash)
//     } else {
//         css.to_string()
//     };

//     let full_css = match (&style.media, &style.combinator) {
//         (Some(media), Some(Combinator::Pre(c))) => format!(
//             "{}{{\n.seed-global-style-{} {}{}{}{{\n{}}}}}\n",
//             media,
//             short_hash,
//             selector,
//             c,
//             style.pseudo.render(),
//             css
//         ),
//         (Some(media), Some(Combinator::Post(c))) => format!(
//             "{}{{\n.seed-global-style-{} {}{}{}{{\n{}}}}}\n",
//             media,
//             c,
//             short_hash,
//             selector,
//             style.pseudo.render(),
//             css
//         ),
//         (Some(media), None) => format!(
//             "{}{{\n.seed-global-style-{} {}{}{{\n{}}}}}\n",
//             media,
//             short_hash,
//             selector,
//             style.pseudo.render(),
//             css
//         ),
//         (None, Some(Combinator::Pre(c))) => format!(
//             "\n.seed-global-style-{} {}{}{}{{\n{}}}\n",
//             short_hash,
//             selector,
//             c,
//             style.pseudo.render(),
//             css
//         ),
//         (None, Some(Combinator::Post(c))) => format!(
//             "\n.seed-global-style-{} {}{}{}{{\n{}}}\n",
//             c,
//             short_hash,
//             selector,
//             style.pseudo.render(),
//             css
//         ),

//         (None, None) => format!(
//             "\n.seed-global-style-{} {}{}{{\n{}}}\n",
//             short_hash,
//             selector,
//             style.pseudo.render(),
//             css
//         ),
//     };
//     if let Some(html_root_elem) = head_elem.get_elements_by_tag_name("html").item(0) {
//         // let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
//         // style_elem
//         //     .sheet()
//         //     .unwrap()
//         //     .dyn_into::<web_sys::CssStyleSheet>()
//         //     .unwrap()

//         let html_root_class = format!(" seed-global-style-{} ", short_hash);
//         if let Some(mut class_name) = html_root_elem.get_attribute("class") {
//             if let Some(existing_seed_style_position) = class_name.find("seedstyle-") {
//                 let final_position = existing_seed_style_position + 10 + short_hash.len();
//                 class_name.replace_range(
//                     existing_seed_style_position..final_position,
//                     &html_root_class,
//                 )
//             } else {
//                 class_name.push_str(&html_root_class);
//             }
//             let _ = html_root_elem.set_attribute("class", &class_name);
//         } else {
//             let _ = html_root_elem.set_attribute("class", &html_root_class);
//         }
//     }

//     let css_stylesheet =
//         if let Some(style_elem) = head_elem.get_elements_by_tag_name("style").item(0) {
//             let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
//             style_elem
//                 .sheet()
//                 .unwrap()
//                 .dyn_into::<web_sys::CssStyleSheet>()
//                 .unwrap()

//         // let existing_style_content = style_elem.inner_html();

//         // let new_style_content = format!("{}\n{}", existing_style_content, full_css);
//         // style_elem.set_inner_html(&new_style_content);
//         } else {
//             let style_elem = document()
//                 .create_element("style")
//                 .unwrap()
//                 .dyn_into::<web_sys::HtmlStyleElement>()
//                 .unwrap();

//             // style_elem.set_inner_html(&full_css);

//             let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
//             let _ = head_elem.append_child(&style_elem);
//             style_elem
//                 .sheet()
//                 .unwrap()
//                 .dyn_into::<web_sys::CssStyleSheet>()
//                 .unwrap()
//         };

//     let rules_length = GLOBAL_STYLES_COUNT.with(|count| count.get());
//     let _ = css_stylesheet.insert_rule_with_index(&full_css, rules_length);
//     GLOBAL_STYLES_COUNT.with(|count| {
//         let mut c = count.get();
//         c += 1;
//         count.set(c);
//     });

//     for (media_breakpoint, rule_vec) in &style.media_rules {
//         let mut media_string = String::new();
//         media_string.push_str(&format!("{}{{\n{}{{\n", media_breakpoint, selector));

//         for rule in rule_vec {
//             media_string.push_str(&rule.render());
//         }

//         let rules_length = GLOBAL_STYLES_COUNT.with(|count| count.get());
//         let _ = css_stylesheet.insert_rule_with_index(&media_string, rules_length);
//         GLOBAL_STYLES_COUNT.with(|count| {
//             let mut c = count.get();
//             c += 1;
//             count.set(c);
//         });
//     }

//     if !style.keyframes.frames.is_empty() {
//         let rules_length = GLOBAL_STYLES_COUNT.with(|count| count.get());
//         let _ = css_stylesheet.insert_rule_with_index(
//             &format!(
//                 "\n\n@keyframes anim-{}{{ \n  {}  \n}}\n",
//                 short_hash,
//                 style.keyframes.render()
//             ),
//             rules_length,
//         );
//         GLOBAL_STYLES_COUNT.with(|count| {
//             let mut c = count.get();
//             c += 1;
//         });
//     }

//     STYLES_USED.with(|css_set_ref| css_set_ref.borrow_mut().insert(variant_hash));
// }

pub trait LocalUpdateEl<T> {
    fn update_el(self, el: &mut T);
}

impl<Ms> LocalUpdateEl<El<Ms>> for Style {
    fn update_el(self, el: &mut El<Ms>) {
        let rendered_css = self.render();
        let existing_style_hashes = if let Some(AtValue::Some(class_string)) = el.attrs.vals.get(&At::Class) {
            let existing_style_hashes = class_string.split(" ").filter_map(|cls|
                if cls.starts_with("seedstyle-") {

                    let harsh_code = cls.split("-").last().unwrap();

                    let hash = HASH_IDS_GENERATOR.with(|builder_refcell| builder_refcell.borrow().decode(harsh_code).unwrap());

                    let hash = &hash.first().unwrap();
                    let existing_style = STYLES_USED
                    .with(|css_set_ref| css_set_ref.borrow().contains(hash));

                    if existing_style {
                        Some(**hash)
                    } else {
                        None
                    }
                } else {
                    None
                }
            ).collect::<Vec<u64>>();
            
            Some(existing_style_hashes)
        } else {
            None
        };

        if let Some(styles_in_elem) = existing_style_hashes {

            let mut s = DefaultHasher::new();
            (styles_in_elem, self.render()).hash(&mut s);
            let revised_variant_hash = s.finish();

            let css_aleady_created = STYLES_USED
                .with(|css_set_ref| css_set_ref.borrow().contains(&revised_variant_hash));

            if !css_aleady_created {
                
                add_css_to_head_unchecked(&rendered_css, revised_variant_hash, &self, &self.name);
            } 
            let short_hash = format!("{}-{}", &self.name, short_uniq_id(revised_variant_hash));
            let class_name = format!("seedstyle-{}", short_hash);
            C![class_name].update_el(el);
        } else {

            let variant_hash = hash_64(&rendered_css, &self.updated_at);

            let class_name = format!(
                "seedstyle-{}",
                add_css_to_head(&rendered_css, variant_hash, &self)
            );

            // sst- style added to raise specificity so has equal(and therefore more) specificity than global styles.
            C!["sst-class",class_name].update_el(el);  
        }
    }
}

// this function adds blocks of css as rules
// however it ensures only one classname is needed.
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
        let short_hash = format!("{}-{}", name, short_uniq_id(variant_hash));
        let class_name = format!("seedstyle-{}", short_hash);

        C!["sst-class",class_name].update_el(el);
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
    let short_hash = format!("{}-{}", style.name, short_uniq_id(variant_hash));

    if !css_aleady_created {
        add_css_to_head_unchecked(css, variant_hash, style, &style.name);
    }

    short_hash
}

fn add_css_to_head_unchecked(css: &str, variant_hash: u64, style: &Style, name: &str) -> String {
    let short_hash = format!("{}-{}", name, short_uniq_id(variant_hash));

    // log!(short_hash);

    let head_elem = document().get_elements_by_tag_name("head").item(0).unwrap();

    let css = if !style.keyframes.frames.is_empty() {
        format!("{}    animation-name: anim-{};\n", css, short_hash)
    } else {
        css.to_string()
    };

    let pre_combinators_str = style
        .pre_combinators
        .iter()
        .map(|c| {
            if let Combinator::Pre(c) = c {
                format!(".sst-class.seedstyle-{}{}{}", short_hash, c, style.pseudo.render())
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join(",");

    let full_css = if style.pre_combinators.len() > 0 {
        match &style.media {
            Some(media) => format!("{}{{\n{}{{\n{}}}}}\n", media, pre_combinators_str, css),

            None => format!("\n{}{{\n{}}}\n", pre_combinators_str, css),
        }
    } else {
        match (&style.media, &style.combinator) {
            (Some(media), Some(Combinator::Pre(c))) => format!(
                "{}{{\n.sst-class.seedstyle-{}{}{}{{\n{}}}}}\n",
                media,
                short_hash,
                c,
                style.pseudo.render(),
                css
            ),
            (Some(media), Some(Combinator::Post(c))) => format!(
                "{}{{\n{}.sst-class.seedstyle-{}{}{{\n{}}}}}\n",
                media,
                c,
                short_hash,
                style.pseudo.render(),
                css
            ),
            (Some(media), None) => format!(
                "{}{{\n.sst-class.seedstyle-{}{}{{\n{}}}}}\n",
                media,
                short_hash,
                style.pseudo.render(),
                css
            ),
            (None, Some(Combinator::Pre(c))) => format!(
                "\n.sst-class.seedstyle-{}{}{}{{\n{}}}\n",
                short_hash,
                c,
                style.pseudo.render(),
                css
            ),
            (None, Some(Combinator::Post(c))) => format!(
                "\n{}.sst-class.seedstyle-{}{}{{\n{}}}\n",
                c,
                short_hash,
                style.pseudo.render(),
                css
            ),

            (None, None) => format!(
                "\n.sst-class.seedstyle-{}{}{{\n{}}}\n",
                short_hash,
                style.pseudo.render(),
                css
            ),
        }
    };
    // if !name.is_empty() {
    //   log!(full_css);
    // }

    let css_stylesheet =
        if let Some(style_elem) = head_elem.get_elements_by_tag_name("style").item(0) {
            let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
            style_elem
                .sheet()
                .unwrap()
                .dyn_into::<web_sys::CssStyleSheet>()
                .unwrap()

        // let existing_style_content = style_elem.inner_html();

        // let new_style_content = format!("{}\n{}", existing_style_content, full_css);
        // style_elem.set_inner_html(&new_style_content);
        } else {
            let style_elem = document()
                .create_element("style")
                .unwrap()
                .dyn_into::<web_sys::HtmlStyleElement>()
                .unwrap();

            // style_elem.set_inner_html(&full_css);

            let style_elem = style_elem.dyn_into::<web_sys::HtmlStyleElement>().unwrap();
            let _ = head_elem.append_child(&style_elem);
            style_elem
                .sheet()
                .unwrap()
                .dyn_into::<web_sys::CssStyleSheet>()
                .unwrap()
        };

    let rules_length = css_stylesheet.css_rules().unwrap().length();
    // log!(full_css);
    let res = css_stylesheet.insert_rule_with_index(&full_css, rules_length);
    if let Err(err) = res {
        log!("error inserting style: " ,err, full_css);
    }

    for (media_breakpoint, rule_vec) in &style.media_rules {
        if style.pre_combinators.len() > 0 {
            let mut media_string = String::new();
            media_string.push_str(&format!(
                "{}{{\n{}{{\n",
                media_breakpoint, pre_combinators_str
            ));

            for rule in rule_vec {
                media_string.push_str(&rule.render());
            }
            let rules_length = css_stylesheet.css_rules().unwrap().length();
            // log!(media_string);
            let res = css_stylesheet.insert_rule_with_index(&media_string, rules_length);
            if let Err(err) = res {
                log!("error inserting style: " ,err , media_string);
            }
        } else {
            let mut media_string = String::new();
            media_string.push_str(&format!(
                "{}{{\n.sst-class.seedstyle-{}{{\n",
                media_breakpoint, short_hash
            ));

            for rule in rule_vec {
                media_string.push_str(&rule.render());
            }
            let rules_length = css_stylesheet.css_rules().unwrap().length();
            // log!(media_string);
            let res = css_stylesheet.insert_rule_with_index(&media_string, rules_length);
            if let Err(err) = res {
                log!("error inserting style: " ,err, media_string);
            }
        }
    }

    if !style.keyframes.frames.is_empty() {
        let rules_length = css_stylesheet.css_rules().unwrap().length();
        let res = css_stylesheet.insert_rule_with_index(
            &format!(
                "\n\n@keyframes anim-{}{{ \n  {}  \n}}\n",
                short_hash,
                style.keyframes.render()
            ),
            rules_length,
        );

        if let Err(err) = res {
            log!("error inserting keyframes style: " ,err);
        }
    }

    STYLES_USED.with(|css_set_ref| css_set_ref.borrow_mut().insert(variant_hash));

    short_hash
}

pub trait AddStyleToNode {
    fn style(&mut self, style: Style);
}

impl<Ms> AddStyleToNode for Node<Ms> {
    fn style(&mut self, style: Style) {

        //  when styling we interrupt the  usual assigning of style to 
        // a node, therefore cannot guarantee still updating the same node
        
        // really need to improve this
        // the best would be to have a hash of existing styles
        // toa actual style and then use this to update.

     

        match self {
            seed::virtual_dom::Node::Element(ref mut elx) => style.update_el(elx),
            _ => {}
        }
    }
}

#[derive(Default, Debug)]
pub struct GlobalStyle {
    pub styles: Vec<(String, Style)>,
}

impl GlobalStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style(mut self, selector: &str, style: Style) -> GlobalStyle {
        if selector == "html" {
            panic!("Sorry for now , You can only set html style directly in css.")
        }
        
        self.styles.push((selector.to_string(), style));
        self
    }

    pub fn activate_init_styles(&self) {
        do_once(|| {
            let html_root_class = "seed-init-style ".to_string();

            if let Some(html_root_elem) = document().get_elements_by_tag_name("html").item(0) {
                if let Some(mut class_name) = html_root_elem.get_attribute("class") {
                    class_name.push_str(" ");
                    class_name.push_str(&html_root_class);
                    let _ = html_root_elem.set_attribute("class", &class_name);
                } else {
                    let _ = html_root_elem.set_attribute("class", &format!(" {}", html_root_class));
                }
            }

            for (selector, style) in &self.styles {
                let rendered_css = style.render();

                add_global_init_css_to_head(
                    &rendered_css,
                    "global_init",
                    &style,
                    &html_root_class,
                    &selector,
                );
            }
        });
    }

    pub fn activate_styles(&self) {
        let mut style_string = "global".to_string();

        for (selector, style) in &self.styles {
            let rendered_css = style.render();
            style_string.push_str(&selector);
            style_string.push_str(&rendered_css);
        }

        let mut s = DefaultHasher::new();
        style_string.hash(&mut s);

        let revised_variant_hash = s.finish();
        let short_hash = short_uniq_id(revised_variant_hash);

        let html_root_class = format!("seed-global-style-{} ", short_hash);

        if let Some(html_root_elem) = document().get_elements_by_tag_name("html").item(0) {
            if let Some(mut class_name) = html_root_elem.get_attribute("class") {
                if let Some(existing_seed_style_position) = class_name.find("seed-global-style-") {
                    if let Some(terminating_position) =
                        class_name[existing_seed_style_position..].find(" ")
                    {
                        let final_term_pos =
                            existing_seed_style_position + terminating_position + 1;
                        class_name.replace_range(
                            existing_seed_style_position..final_term_pos,
                            &html_root_class,
                        )
                    } else {
                        class_name.replace_range(existing_seed_style_position.., &html_root_class)
                    }
                } else {
                    class_name.push_str(" ");
                    class_name.push_str(&html_root_class);
                }
                let _ = html_root_elem.set_attribute("class", &format!("{}", &class_name));
            } else {
                let _ = html_root_elem.set_attribute("class", &format!(" {}", &html_root_class));
            }
        }

        let css_aleady_created =
            STYLES_USED.with(|css_set_ref| css_set_ref.borrow().contains(&revised_variant_hash));

        if !css_aleady_created {
            for (selector, style) in &self.styles {
                let rendered_css = style.render();

                add_global_init_css_to_head(
                    &rendered_css,
                    &short_hash,
                    &style,
                    &html_root_class,
                    &selector,
                );
                STYLES_USED.with(|css_set_ref| {
                    css_set_ref
                        .borrow_mut()
                        .insert(revised_variant_hash.clone())
                });
            }
        }
    }
}
