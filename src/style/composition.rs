use super::layout::*;
use super::measures::{pc, px};
use super::theme::*;
use super::*;
use crate::style::s;
use crate::style::Style;
use std::collections::HashMap;

pub struct Composition<A, Mdl, Ms, T>
where
    T: BreakpointTheme,
    A: LayoutArea,
{
    pub container_styles: Vec<Style>,
    pub layouts: Vec<Layout<A>>,
    pub mocked_children: Option<(String, u32, CssWidth, CssHeight)>,
    pub children: Vec<Box<dyn Fn(&Mdl) -> Node<Ms> + 'static>>,
    pub default_idx: Option<usize>,
    pub areas_hm: HashMap<A, Box<dyn Fn(&Mdl) -> Node<Ms> + 'static>>,
    pub layouts_hm: HashMap<T, usize>,
}

impl<A, Mdl, Ms, T> Default for Composition<A, Mdl, Ms, T>
where
    T: BreakpointTheme,
    A: LayoutArea,
{
    fn default() -> Self {
        Self {
            container_styles: vec![],
            layouts: vec![],
            default_idx: None,
            layouts_hm: HashMap::new(),
            mocked_children: None,
            children: vec![],
            areas_hm: HashMap::new(),
        }
    }
}

// Named Breakpoints Keys allow you to refer to a named breakpoint in layout helpers and css media queries.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum SeedBreakpoint {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl BreakpointTheme for SeedBreakpoint {} // Enable `Breakpoint` as a Breakpoint alias.

pub fn default_bp_theme() -> Theme {
    use SeedBreakpoint::*;

    // I generally set the named aliases seperately from the theme scales:
    Theme::new("default_bp_theme")
        .set_breakpoint(ExtraSmall, (0, Some(600))) // Breakpoints are upper bound exclusive lower bound inclusive.
        .set_breakpoint(Small, (600, Some(960)))
        .set_breakpoint(Medium, (960, Some(1280)))
        .set_breakpoint(Large, (1280, Some(1920)))
        .set_breakpoint(ExtraLarge, (1920, None))
}

pub trait WithLayoutComposition<A, Mdl, Ms>
where
    // T: BreakpointTheme + 'static,
    Ms: 'static,
    Mdl: 'static,
    A: LayoutArea,
{
    fn with_layout(layout: Layout<A>) -> Composition<A, Mdl, Ms, SeedBreakpoint>;
}

impl<A, Mdl, Ms> WithLayoutComposition<A, Mdl, Ms> for Composition<A, Mdl, Ms, SeedBreakpoint>
where
    // T: BreakpointTheme + 'static,
    Ms: 'static,
    Mdl: 'static,
    A: LayoutArea,
{
    fn with_layout(layout: Layout<A>) -> Composition<A, Mdl, Ms, SeedBreakpoint> {
        let mut c = Composition::default();
        c.layouts.push(layout);
        let idx = c.layouts.len() - 1;
        c.layouts_hm.insert(SeedBreakpoint::ExtraSmall, idx);
        c
    }
}

impl<A, Mdl, Ms, T> Composition<A, Mdl, Ms, T>
where
    T: BreakpointTheme + 'static,
    Ms: 'static,
    Mdl: 'static,
    A: LayoutArea,
{
    pub fn with_layouts(bp_layouts: &[(T, Layout<A>)]) -> Self {
        let mut c = Composition::default();
        for (bp, layout) in bp_layouts {
            let layout = layout.clone();
            c.layouts.push(layout);
            let idx = c.layouts.len() - 1;
            c.layouts_hm.insert(bp.clone(), idx);
        }
        c
    }

    pub fn add(&mut self, bp: T, layout: Layout<A>) -> &mut Self {
        self.layouts.push(layout);
        let idx = self.layouts.len() - 1;
        // for bp in bps {
        self.layouts_hm.insert(bp.clone(), idx);
        // }
        self
    }

    pub fn set_default(&mut self, layout: Layout<A>) -> &mut Self {
        self.layouts.push(layout);
        self.default_idx = Some(self.layouts.len() - 1);
        self
    }

    pub fn mock_children<W, H>(&mut self, name: &str, count: u32, width: W, height: H) -> &mut Self
    where
        W: Into<CssWidth>,
        H: Into<CssHeight>,
    {
        let width = width.into();
        let height = height.into();
        self.mocked_children = Some((name.to_string(), count, width, height));
        self
    }

    pub fn render(&self, model: &Mdl) -> Node<Ms> {
        use_themes(
            || vec![default_bp_theme()],
            || {
                let mut sorted_bps = self.layouts_hm.keys().cloned().collect::<Vec<T>>();
                sorted_bps.sort_unstable_by_key(|bp_key| {
                    let bp_pair = with_themes(ReturnBpTuple(bp_key.clone()));
                    -(bp_pair.0 as i32)
                });

                // We find the biggest breakpoint that fits...

                // find the first layout which
                let opt_layout = sorted_bps
                    .iter()
                    .map(|bp_key| {
                        (
                            with_themes(ReturnBpTuple(bp_key.clone())),
                            self.layouts_hm.get(bp_key),
                        )
                    })
                    .find(|(bp_pair, _layout)| match bp_pair {
                        (lower, _) => window()
                            .match_media(&format!("(min-width: {}px)", lower))
                            .unwrap()
                            .unwrap()
                            .matches(),
                    });

                if let Some((_bp_pair, Some(idx))) = opt_layout {
                    self.render_layout(*idx, model)
                } else {
                    if let Some(idx) = self.default_idx {
                        self.render_layout(idx, model)
                    } else {
                        let smallest_bp_key = sorted_bps.last().unwrap();
                        let idx_of_smallest_layout = self.layouts_hm.get(smallest_bp_key).unwrap();

                        self.render_layout(*idx_of_smallest_layout, model)
                    }
                }
            },
        )
    }

    pub fn set_content<F: Fn(&Mdl) -> Node<Ms> + 'static>(
        &mut self,
        area: A,
        area_view: F,
    ) -> &mut Self {
        let boxed_area_view = Box::new(area_view);
        self.areas_hm.insert(area, boxed_area_view);
        self
    }
    pub fn add_child<F: Fn(&Mdl) -> Node<Ms> + 'static>(&mut self, child_view: F) -> &mut Self {
        let boxed_child_view = Box::new(child_view);
        self.children.push(boxed_child_view);
        self
    }

    pub fn add_style(&mut self, style: Style) -> &mut Self {
        self.container_styles.push(style);
        self
    }

    pub fn add_styles(&mut self, styles: &[Style]) -> &mut Self {
        for style in styles {
            self.add_style(style.clone());
        }
        self
    }

    pub fn render_layout(&self, idx: usize, model: &Mdl) -> Node<Ms> {
        let layout = &self.layouts[idx];
        if layout.layout.len() > 0 {
            self.render_areas(idx, model)
        } else {
            self.render_grid(idx, model)
        }
    }

    pub fn render_grid(&self, idx: usize, model: &Mdl) -> Node<Ms> {
        let layout = &self.layouts[idx];
        div![
            s().grid_template_columns("1 fr ")
                .box_sizing("border_box")
                .display_grid(),
            self.container_styles,
            if let Some(styles) = &layout.container_styles {
                styles.clone()
            } else {
                Style::default()
            },
            self.children
                .iter()
                .map(|child| child(model))
                .collect::<Vec<Node<Ms>>>(),
            if let Some((name, count, width, height)) = &self.mocked_children {
                self.mocked_child(name, *count, width.clone(), height.clone())
            } else {
                vec![]
            }
        ]
    }

    pub fn render_areas(&self, idx: usize, model: &Mdl) -> Node<Ms> {
        let layout = &self.layouts[idx];

        let number_of_columns = layout.layout.first().unwrap().len();
        let number_of_rows = layout.layout.len();
        let one_frs = std::iter::repeat("minmax(0,1fr) ");
        let grid_template_columns = one_frs.clone().take(number_of_columns).collect::<String>();
        let grid_template_rows = one_frs.take(number_of_rows).collect::<String>();

        let mut grid_template_areas = String::new();

        for row in &layout.layout {
            let mut grid_template_areas_row = String::from("\"");

            for area in row {
                if area.is_empty() {
                    grid_template_areas_row.push_str(" . ");
                } else {
                    grid_template_areas_row.push_str(&format!("{:?} ", area).replace("::", "__"));
                }
            }

            grid_template_areas_row.push_str("\" ");
            grid_template_areas.push_str(&grid_template_areas_row);
        }
        let mut gtc = grid_template_columns.clone();
        let mut gtr = "auto".to_string();

        if let Some(style) = &layout.container_styles {
            for rule in &style.rules {
                let rule_str = format!("{}", rule.value);
                if rule_str.contains("grid-template-columns") {
                    gtc = rule_str
                        .trim_start_matches("grid-template-columns: ")
                        .trim_end_matches(";")
                        .to_string();
                }
                if rule_str.contains("grid-template-rows") {
                    gtr = rule_str
                        .trim_start_matches("grid-template-rows: ")
                        .trim_end_matches(";")
                        .to_string();
                }
            }
        }

        div![
            s().grid_template_columns(grid_template_columns.as_str())
                .grid_template_rows(grid_template_rows.as_str())
                .height(pc(100))
                .box_sizing("border_box")
                .display_grid()
                .grid_template_areas(grid_template_areas.as_str()),
            self.container_styles,
            if let Some(styles) = &layout.container_styles {
                styles.clone()
            } else {
                Style::default()
            },
            layout.areas.iter().map(|area| {
                div![
                    {
                        let grid_area_style = s()
                            .grid_area(format!("{:?} ", area).replace("::", "__").as_str())
                            .name(format!("{:?}_wrapper", area).as_str());

                        let area_style = if let Some(styles) = layout.area_styles.get(area) {
                            let mut styles = styles.clone();
                            styles.push(grid_area_style);
                            styles
                        } else {
                            vec![grid_area_style]
                        };
                        area_style
                    },
                    if let Some(view) = self.areas_hm.get(area) {
                        view(model)
                    } else {
                        self.mock(area.clone(), gtc.as_str(), gtr.as_str())
                    },
                ]
            }),
            if let Some((name, count, width, height)) = &self.mocked_children {
                self.mocked_child(name, *count, width.clone(), height.clone())
            } else {
                vec![]
            }
        ]
    }

    pub fn mock(&self, area: A, gtc: &str, gtr: &str) -> Node<Ms> {
        div![
            s().box_sizing_border_box()
                .text_align_center()
                .p(px(30))
                .height(pc(100.))
                .border_style_dashed()
                .border_width(px(2))
                .display_flex()
                .flex_direction_column()
                .align_items_center()
                .justify_content_center()
                .border_color(seed_colors::Gray::No7)
                .bg_color(seed_colors::Gray::No5),
            h1![
                s().font_size(px(24)),
                format!("{:?} ", area).replace("::", "__")
            ],
            p![
                s().color(seed_colors::Gray::No8),
                format!("cols: ({})", gtc)
            ],
            p![
                s().color(seed_colors::Gray::No8),
                format!("rows: ({})", gtr)
            ],
        ]
    }

    pub fn mocked_child<W, H>(&self, name: &str, count: u32, width: W, height: H) -> Vec<Node<Ms>>
    where
        W: Into<CssWidth>,
        H: Into<CssHeight>,
    {
        let width = width.into();
        let height = height.into();
        (0..count)
            .into_iter()
            .map(|i| {
                div![
                    s().box_sizing_border_box()
                        .w(width.clone())
                        .h(height.clone())
                        .max_width(pc(100))
                        .text_align_center()
                        .p(px(30))
                        .border_style_dashed()
                        .border_width(px(2))
                        .display("flex")
                        .flex_direction_column()
                        .align_items("center")
                        .justify_content_center()
                        .border_color(seed_colors::Gray::No7)
                        .bg_color(seed_colors::Red::No4),
                    h1![s().font_size(px(24)), format!("Mocked {} No.{} ", name, i)],
                    p![
                        s().color(seed_colors::Gray::No8),
                        format!("({} × {} ), max-width: 100%;", width, height)
                    ],
                ]
            })
            .collect::<Vec<Node<Ms>>>()
    }
}
