use seed::{prelude::*, *};
use super::measures::{ExactLength, px};
use super::*;
use crate::style::s;
use crate::view_macro;


use crate::style::state_access::StateAccess;



// #[view_macro]
// fn center_view<Ms>(root: Node<Ms>, children: Vec<Node<Ms>>) -> Node<Ms> {
//     root![
//         s()
//         .display_flex()
//         .flex_direction_row()
//         .justify_content_center()
//         .align_items_center()
//         ,
//         children,
//     ]
// }

pub struct RowColumnArgs {
  pub   gap: ExactLength,
  pub   padding: ExactLength,
  pub   flex: Flex,
}


impl Default for RowColumnArgs {
    fn default() -> Self {
        RowColumnArgs {
            gap: px(0),
            padding: px(0),
            flex: Flex::Full,
        }
    }
}
pub enum Flex {
    None,
    Unset,
    Content,
    Full,
    Custom(String),
}



pub enum RowAlign {
    Left,
    Center,
    Right,
    LeftTop,
    CenterTop,
    RightTop,
    LeftMiddle,
    CenterMiddle,
    RightMiddle,
    LeftBottom,
    CenterBottom,
    RightBottom,
}


pub struct RowItemArgs {
    pub align: RowAlign,
    pub flex: Flex,
}

impl std::default::Default for RowItemArgs {
    fn default() -> Self {    
        RowItemArgs {
             align: RowAlign::Left,
             flex: Flex::Unset,
        }   
    }
}


#[view_macro]
#[allow(non_snake_case)]
fn Row_view<Ms>(args: RowColumnArgs, mut root: Node<Ms>, mut children: Vec<Node<Ms>>, mut Item: Vec<(Node<Ms>, RowItemArgs)>) -> Node<Ms> {
    let (flex,gap, padding) = (args.flex, args.gap, args.padding);

    let mut gap = gap.clone();
    gap.value = gap.value / 2.0;

    let mut left_nodes = vec![];
    let mut center_nodes = vec![];
    let mut right_nodes = vec![];

    

    for item in Item.drain(0..) {
        let mut item = item;

        item.0.style(s().mx(gap.clone()));
        item.0.style(s().first_child().margin_left(px(0)));
        item.0.style(s().last_child().margin_right(px(0)));

     
        match &item.1.flex {
            Flex::Unset => (),
            Flex::None => item.0.style(s().flex("0 0 auto")),
            Flex::Full => item.0.style(s().flex("1 1 0%").min_width(px(0)).min_height(px(0))),
            Flex::Content => item.0.style(s().flex("1 0 auto")),
            Flex::Custom(flex_string) => item.0.style(s().flex(flex_string.as_str())),
        }

        match item.1.align {
            RowAlign::Left => {item.0.style(s().p(padding.clone()).name("row_item"));left_nodes.push(item)},
            RowAlign::Center => {item.0.style(s().p(padding.clone()).name("row_item"));center_nodes.push(item)},
            RowAlign::Right=> {item.0.style(s().p(padding.clone()).name("row_item"));right_nodes.insert(0,item)},
            
            RowAlign::LeftTop =>   {item.0.style(s().align_self_flex_start().p(padding.clone()).name("row_item")); left_nodes.push(item);},
            RowAlign::CenterTop=>  {item.0.style(s().align_self_flex_start().p(padding.clone()).name("row_item"));center_nodes.push(item)},
            RowAlign::RightTop =>  {item.0.style(s().align_self_flex_start().p(padding.clone()).name("row_item")); right_nodes.insert(0,item)},
            
            RowAlign::LeftMiddle =>    {item.0.style(s().align_self_center().p(padding.clone()).name("row_item")); left_nodes.push(item)},
            RowAlign::CenterMiddle =>  {item.0.style(s().align_self_center().p(padding.clone()).name("row_item")); center_nodes.push(item)},
            RowAlign::RightMiddle =>   {item.0.style(s().align_self_center().p(padding.clone()).name("row_item")); right_nodes.insert(0,item)},
            
            RowAlign::LeftBottom =>   {item.0.style(s().align_self_flex_end().p(padding.clone()).name("row_item")); left_nodes.push(item)},
            RowAlign::CenterBottom=>  {item.0.style(s().align_self_flex_end().p(padding.clone()).name("row_item")); center_nodes.push(item)},
            RowAlign::RightBottom =>  {item.0.style(s().align_self_flex_end().p(padding.clone()).name("row_item")); right_nodes.insert(0,item)},
     }


    

 }


    for mut c in children.drain(0..){
        c.style(s().mx(gap.clone()));
        c.style(s().first_child().margin_left(px(0)));
        c.style(s().last_child().margin_right(px(0)));
    
        left_nodes.push(   (c, RowItemArgs::default()));
    }

    
    root.style(s().name("row_layout").display_grid());
    match flex {
        Flex::Unset => (),
        Flex::None => root.style(s().flex("0 0 auto")),
        Flex::Full => root.style(s().flex("1 1 0%").min_width(px(0)).min_height(px(0))),
        Flex::Content => root.style(s().flex("1 0 auto")),
        Flex::Custom(flex_string) => root.style(s().flex(flex_string.as_str())),
    }

    match ( left_nodes.len(), center_nodes.len(), right_nodes.len()){
        (0, 0, 0) => empty![],
        (0, 0, _) => root![
            s().grid_template_columns("minmax(0px, 1fr)").grid_template_rows("minmax(0,1fr)"),
            div![s().display_flex().flex_direction_row().justify_content_flex_end(),right_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (0, _, 0) => root![
            s().grid_template_columns("auto").grid_template_rows("minmax(0,1fr)"), 
            div![s().display_flex().flex_direction_row().justify_content_center(), center_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (0, _, _) => root![
            s().grid_template_columns("minmax(0px, 1fr) auto minmax(0px, 1fr)").grid_template_rows("minmax(0,1fr)"), 
            div![],
            div![s().mr(gap.clone()).display_flex().flex_direction_row().justify_content_center(), center_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().ml(gap.clone()).display_flex().flex_direction_row().justify_content_flex_end(),right_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (_, 0, 0) => root![
            s().grid_template_columns("minmax(0px, 1fr)").grid_template_rows("minmax(0,1fr)"), 
            div![s().display_flex().flex_direction_row().justify_content_flex_start(), left_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (_, 0, _) => root![
            s().grid_template_columns("minmax(0px, 1fr) minmax(0px, 1fr)").grid_template_rows("minmax(0,1fr)"), 
            div![s().mr(gap.clone()).display_flex().flex_direction_row().justify_content_flex_start(),left_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().ml(gap.clone()).display_flex().flex_direction_row().justify_content_flex_end(),right_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (_, _, _) => root![
            s().grid_template_columns("minmax(0px, 1fr) auto minmax(0px, 1fr)").grid_template_rows("minmax(0,1fr)"), 
            div![s().mr(gap.clone()).display_flex().flex_direction_row().justify_content_flex_start(), left_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().mx(gap.clone()).display_flex().flex_direction_row().justify_content_center(), center_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().ml(gap.clone()).display_flex().flex_direction_row().justify_content_flex_end(), right_nodes.iter().map(|(Item,_arg)| Item)],
        ],
    }
}





pub enum ColumnAlign {
    Top,
    Middle,
    Bottom,
    TopLeft,
    MiddleLeft,
    BottomLeft,
    TopCenter,
    MiddleCenter,
    BottomCenter,
    TopRight,
    MiddleRight,
    BottomRight,
}

pub struct ColumnItemArgs {
    pub align: ColumnAlign,
    pub flex: Flex,
}

impl std::default::Default for ColumnItemArgs {
    fn default() -> Self {    
         ColumnItemArgs {
            align: ColumnAlign::Top,
            flex: Flex::Unset,
        }   
    }
}


#[view_macro]
#[allow(non_snake_case)]
pub fn Column_view<Ms>(args: RowColumnArgs, mut root: Node<Ms>, mut children: Vec<Node<Ms>>, mut Item: Vec<(Node<Ms>, ColumnItemArgs)>) -> Node<Ms> {
    let (flex,gap, padding) = (args.flex, args.gap, args.padding);

     let mut gap = gap.clone();
    gap.value = gap.value / 2.0;
    
    let mut top_nodes = vec![];
    let mut middle_nodes = vec![];
    let mut bottom_nodes = vec![];


    for Item in Item.drain(0..) {
        let mut item = Item;
        item.0.style(s().my(gap.clone()));
        item.0.style(s().first_child().margin_top(px(0)));
        item.0.style(s().last_child().margin_bottom(px(0)));
        
      
        match &item.1.flex {
            Flex::Unset => (),
            Flex::None => item.0.style(s().flex("0 0 auto")),
            Flex::Full => item.0.style(s().flex("1 1 0%").min_width(px(0)).min_height(px(0))),
            Flex::Content => item.0.style(s().flex("1 0 auto")),
            Flex::Custom(flex_string) => item.0.style(s().flex(flex_string.as_str())),
        }

        match item.1.align {
            ColumnAlign::Top => {item.0.style(s().p(padding.clone()).name("col_item"));top_nodes.push(item)},
            ColumnAlign::Bottom=> {item.0.style(s().p(padding.clone()).name("col_item"));bottom_nodes.insert(0,item)},
            ColumnAlign::Middle =>{item.0.style(s().p(padding.clone()).name("col_item")); middle_nodes.push(item)},
            ColumnAlign::TopLeft => {item.0.style(s().align_self_flex_start().p(padding.clone()).name("col_item")); top_nodes.push(item);},
            ColumnAlign::MiddleLeft=> {item.0.style(s().align_self_flex_start().p(padding.clone()).name("col_item")); middle_nodes.push(item)},
            ColumnAlign::BottomLeft => {item.0.style(s().align_self_flex_start().p(padding.clone()).name("col_item")); bottom_nodes.insert(0,item)},
            ColumnAlign::TopCenter =>  {item.0.style(s().align_self_center().p(padding.clone()).name("col_item")); top_nodes.push(item)},
            ColumnAlign::MiddleCenter => {item.0.style(s().align_self_center().p(padding.clone()).name("col_item")); middle_nodes.push(item)},
            ColumnAlign::BottomCenter => {item.0.style(s().align_self_center().p(padding.clone()).name("col_item")); bottom_nodes.insert(0,item)},
            ColumnAlign::TopRight => {item.0.style(s().align_self_flex_end().p(padding.clone()).name("col_item")); top_nodes.push(item)},
            ColumnAlign::MiddleRight => {item.0.style(s().align_self_flex_end().p(padding.clone()).name("col_item")); middle_nodes.push(item)},
            ColumnAlign::BottomRight => {item.0.style(s().align_self_flex_end().p(padding.clone()).name("col_item")); bottom_nodes.insert(0,item)},
        }
    }

    for mut c in children.drain(0..){
        c.style(s().mx(gap.clone()));
        c.style(s().first_child().margin_top(px(0)));
        c.style(s().last_child().margin_bottom(px(0)));
        top_nodes.push((c, ColumnItemArgs::default()));
    }



      
    root.style(s().name("row_layout").display_grid());
    match flex {
        Flex::Unset => (),
        Flex::None => root.style(s().flex("0 0 auto")),
        Flex::Full => root.style(s().flex("1 1 0%").min_width(px(0)).min_height(px(0))),
        Flex::Content => root.style(s().flex("1 0 auto")),
        Flex::Custom(flex_string) => root.style(s().flex(flex_string.as_str())),
    }

    
    root.style(s().flex("1 1 0%").min_height(px(0)).name("column_layout").display_grid());

    match ( top_nodes.len(), middle_nodes.len(), bottom_nodes.len()){
        (0, 0, 0) => empty![],
        (0, 0, _) => root![
            s().grid_template_rows("minmax(0px, 1fr)").grid_template_columns("minmax(0,1fr)"), 
            div![s().display_flex().flex_direction_column().justify_content_flex_end(),bottom_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (0, _, 0) => root![
            s().grid_template_rows("auto").grid_template_columns("minmax(0,1fr)"), 
            div![s().display_flex().flex_direction_column().justify_content_center(), middle_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (0, _, _) => root![
            s().grid_template_rows("minmax(0px, 1fr) auto minmax(0px, 1fr)").grid_template_columns("minmax(0,1fr)"), 
            div![],
            div![s().mb(gap.clone()).display_flex().flex_direction_column().justify_content_center(),  middle_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().mt(gap.clone()).display_flex().flex_direction_column().justify_content_flex_end(),  bottom_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (_, 0, 0) => root![
            s().grid_template_rows("minmax(0px, 1fr)").grid_template_columns("minmax(0,1fr)"), 
            div![s().display_flex().flex_direction_column().justify_content_flex_start(), top_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (_, 0, _) => root![
            s().grid_template_rows("minmax(0px, 1fr) minmax(0px, 1fr)").grid_template_columns("minmax(0,1fr)"), 
            div![s().mb(gap.clone()).display_flex().flex_direction_column().justify_content_flex_start(),  top_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().mt(gap.clone()).display_flex().flex_direction_column().justify_content_flex_end(),  bottom_nodes.iter().map(|(Item,_arg)| Item)],
        ],
        (_, _, _) => root![
            s().grid_template_rows("minmax(0px, 1fr) auto minmax(0px, 1fr)").grid_template_columns("minmax(0,1fr)"), 
            div![s().mb(gap.clone()).display_flex().flex_direction_column().justify_content_flex_start(), top_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().my(gap.clone()).display_flex().flex_direction_column().justify_content_center(),  middle_nodes.iter().map(|(Item,_arg)| Item)],
            div![s().mt(gap.clone()).display_flex().flex_direction_column().justify_content_flex_end(),  bottom_nodes.iter().map(|(Item,_arg)| Item)],
        ],
    }
}
// pub use Column;