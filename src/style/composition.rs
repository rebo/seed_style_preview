use super::layout::*;
use super::measures::{pc, px};
use super::theme::*;
use super::*;
use crate::style::Style;
use std::collections::HashMap;

pub struct Composition<T, A, Mdl, Ms>
where
    T: BreakpointTheme,
    A: LayoutArea,
{
    pub layouts: Vec<Layout<A>>,
    pub mocked_children: Option<(String, u32, ExactLength, ExactLength)>,
    pub children: Vec<Box<dyn Fn(&Mdl) -> Node<Ms> + 'static>>,
    pub default_idx: Option<usize>,
    pub areas_hm: HashMap<A, Box<dyn Fn(&Mdl) -> Node<Ms> + 'static>>,
    pub layouts_hm: HashMap<T, usize>,
}

impl<T, A, Mdl, Ms> Default for Composition<T, A, Mdl, Ms>
where
    T: BreakpointTheme,
    A: LayoutArea,
{
    fn default() -> Self {
        Self {
            layouts: vec![],
            default_idx: None,
            layouts_hm: HashMap::new(),
            mocked_children: None,
            children: vec![],
            areas_hm: HashMap::new(),
        }
    }
}

impl<T, A, Mdl, Ms> Composition<T, A, Mdl, Ms>
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

    pub fn with_layout(bp: T, layout: Layout<A>) -> Self {
        let mut c = Composition::default();
        c.layouts.push(layout);
        let idx = c.layouts.len() - 1;
        c.layouts_hm.insert(bp.clone(), idx);
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

    pub fn mock_children(
        &mut self,
        name: &str,
        count: u32,
        width: ExactLength,
        height: ExactLength,
    ) -> &mut Self {
        self.mocked_children = Some((name.to_string(), count, width, height));
        self
    }

    pub fn render(&self, model: &Mdl) -> Node<Ms> {
        // sorted breakpoints

        let mut sorted_bps = self.layouts_hm.keys().cloned().collect::<Vec<T>>();
        sorted_bps.sort_unstable_by_key(|bp_key| {
            let bp_pair = with_themes(|borrowed_themes| {
                borrowed_themes
                    .iter()
                    .find_map(|theme| theme.get::<T, (u32, Option<u32>)>(bp_key.clone().clone()))
                    .unwrap()
            });
            -(bp_pair.0 as i32)
        });

        // We find the biggest breakpoint that fits...

        // find the first layout which
        let opt_layout = sorted_bps
            .iter()
            .map(|bp_key| {
                (
                    with_themes(|borrowed_themes| {
                        borrowed_themes
                            .iter()
                            .find_map(|theme| {
                                theme.get::<T, (u32, Option<u32>)>(bp_key.clone().clone())
                            })
                            .unwrap()
                    }),
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
            S.grid_template_columns("1 fr ")
                .box_sizing("border_box")
                .display_grid(),
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
                self.mocked_child(name, *count, width, height)
            } else {
                vec![]
            }
        ]
    }

    pub fn render_areas(&self, idx: usize, model: &Mdl) -> Node<Ms> {
        let layout = &self.layouts[idx];

        let number_of_columns = layout.layout.iter().map(|v| v.len()).max().unwrap();
        let one_frs = std::iter::repeat("1fr ");
        let grid_template_columns = one_frs.take(number_of_columns).collect::<String>();

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
            S.grid_template_columns(grid_template_columns.as_str())
                .box_sizing("border_box")
                .display_grid()
                .grid_template_areas(grid_template_areas.as_str()),
            if let Some(styles) = &layout.container_styles {
                styles.clone()
            } else {
                Style::default()
            },
            layout.areas.iter().map(|area| {
                div![
                    if let Some(styles) = layout.area_styles.get(area) {
                        styles.clone()
                    } else {
                        Style::default()
                    }
                    .grid_area(format!("{:?} ", area).replace("::", "__").as_str())
                    .name(format!("{:?}_wrapper", area).as_str()),
                    if let Some(view) = self.areas_hm.get(area) {
                        view(model)
                    } else {
                        self.mock(area.clone(), gtc.as_str(), gtr.as_str())
                    },
                ]
            }),
            if let Some((name, count, width, height)) = &self.mocked_children {
                self.mocked_child(name, *count, width, height)
            } else {
                vec![]
            }
        ]
    }

    pub fn mock(&self, area: A, gtc: &str, gtr: &str) -> Node<Ms> {
        div![
            S.box_sizing_border_box()
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
                S.font_size(px(24)),
                format!("{:?} ", area).replace("::", "__")
            ],
            p![S.color(seed_colors::Gray::No8), format!("cols: ({})", gtc)],
            p![S.color(seed_colors::Gray::No8), format!("rows: ({})", gtr)],
        ]
    }

    pub fn mocked_child(
        &self,
        name: &str,
        count: u32,
        width: &ExactLength,
        height: &ExactLength,
    ) -> Vec<Node<Ms>> {
        (0..count)
            .into_iter()
            .map(|i| {
                div![
                    S.box_sizing_border_box()
                        .w(width.clone())
                        .h(height.clone())
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
                    h1![S.font_size(px(24)), format!("Mocked {} No.{} ", name, i)],
                    p![
                        S.color(seed_colors::Gray::No8),
                        format!("({} × {} )", width, height)
                    ],
                ]
            })
            .collect::<Vec<Node<Ms>>>()
    }
}
