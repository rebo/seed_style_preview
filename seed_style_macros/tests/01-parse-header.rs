#![feature(proc_macro_hygiene)]
use seed::{prelude::*, *};
use seed_style_macros::*;
use illicit::*;
use seed_hooks::*;
// use seed_style::*;


// pub enum DslBuilderType {
//     Required,
//     HasArgs,
//     VecHasArgs,
//     OptionalHasArgs,
//     Optional,
//     Vec,
// }

#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[derive(Debug)]
enum Msg {
    NoOp,
}

pub trait UpdateView<T> {
    fn update_view(self, builder: &mut T);
}

// #[illicit::from_env(num: &u8)]

#[derive(Default)]
struct ViewArgs {
    count : i32,
}

#[derive(Default)]
struct NameOpts {
    name_count : i32,
}


#[view_macro]
fn my_view<Ms>(args: ViewArgs, mut root: Node<Ms>, children: Vec<Node<Ms>>,  name: Node<Ms>) -> Node<Ms> {
    root![]
}

// #[view_macro]
// fn my_view<Ms>(args: ViewArgs, mut root: Node<Ms>, children: Vec<Node<Ms>>,  name: (Node<Ms>, NameOpts),face: Option<Node<Ms>>, ) -> Node<Ms> {
//     let (mut name, name_args) = name;
//     as_tag![
//         h1,
//         root,
//         ul![
//             args.count,
//             span![
//                 children
//             ],
//             as_tag![
//                 p,
//                 name,
//                 "The name argument is ",
//                 name_args.name_count,
//             ],
//             if let Some(face) = face{
//                 div![
//                     face
//                 ]
//             } else {
//                 empty!()
//             }
//         ]
//     ]
// }

fn main() {
  let foo : Node<Msg> = my![ 
        name![
            // name_count = 99,
            span!["hi"]
        ],
    ];
    println!("{:#?}", foo);
}
