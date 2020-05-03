use super::*;
use crate::style::Style;
// use seed::{prelude::*, *};
use std::collections::HashMap;
use std::marker::PhantomData;


#[derive(Default, Clone)]
pub struct Layout<A>
where
    A: GridArea,
{
    pub areas: Vec<A>,
    pub layout: Vec<Vec<A>>,
    pub container_styles: Option<Style>,
    pub area_styles: HashMap<A, Style>,
    pub _phantom_data: PhantomData<A>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum NoArea {}
impl GridArea for NoArea {}

impl<A> Layout<A>
where
    A: GridArea,
{
    pub fn style(mut self, style: Style) -> Self {
        self.container_styles = Some(style);
        self
    }

    pub fn area_style(mut self, area: A, style: Style) -> Self {
        self.area_styles.insert(area, style);
        self
    }

    pub fn areas(layout_array: &[&[A]]) -> Self {
        let mut areas = vec![];
        let mut layout = vec![];
        for row in layout_array {
            let mut inner_vec_layout = vec![];
            for area in row.iter().cloned() {
                if !areas.contains(&area) {
                    if !area.is_empty() {
                        areas.push(area.clone());
                    }
                }
                inner_vec_layout.push(area.clone());
            }
            layout.push(inner_vec_layout);
        }

        Layout {
            areas,
            layout,
            container_styles: None,
            area_styles: HashMap::new(),
            _phantom_data: PhantomData,
        }
    }

    pub fn grid() -> Layout<NoArea> {
        Layout::<NoArea> {
            areas: vec![],
            layout: vec![],
            container_styles: None,
            area_styles: HashMap::new(),
            _phantom_data: PhantomData::<NoArea>,
        }
    }
}

pub trait GridArea: Hash + PartialEq + Eq + std::fmt::Debug + Clone + 'static {
    fn is_empty(&self) -> bool {
        false
    }
}
