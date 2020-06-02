#![feature(proc_macro_hygiene)]
use seed::{prelude::*, *};
use seed_style_macros::{view_macro,process_part,process_submacro_part,as_tag};
// use seed_style::*;

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

#[view_macro]
fn my_view<Ms>(args: ViewArgs, mut root: Node<Ms>, children: Vec<Node<Ms>>, mut name: Node<Ms>, face: Option<Node<Ms>>) -> Node<Ms> {
    as_tag![
        h1,
        root,
        ul![
            args.count,
            span![
                children
            ],
            as_tag![
                p,
                name,
            ],
            if let Some(face) = face{
                div![
                    face
                ]
            } else {
                empty!()
            }
        ]
    ]
}

fn main() {
  let foo : Node<Msg> = my![ 
        count = 3,
        name![
            span!["hi"]
        ],
        face![
            ol!["sds"]
        ],
        div![
            my![
                count = 4,
                name![
                    "dsds"
                ],
                face![
                    "dddd"
                ]
            ]
        ],
    ];
    println!("{:#?}", foo);
}
