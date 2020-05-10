use super::theme::*;
use seed::{prelude::*, *};
use seed_hooks::use_state;
use std::collections::HashMap;

#[track_caller]
pub fn conditionally_skip_rendering<
    'a,
    T: 'static + BreakpointTheme,
    Ms: 'static,
    Q: Orders<Ms>,
    F: 'a + Fn() -> Theme,
>(
    theme_func: F,
    orders: &mut Q,
) {
    let current_breakpoint = use_state::<Option<T>, _>(|| None);

    let bp_hm = use_state(|| {
        theme_func()
            .anymap
            .remove::<HashMap<T, (u32, Option<u32>)>>()
            .unwrap()
    });

    bp_hm.get_with(|bp_hm| {
        current_breakpoint.update(|current_breakpoint| {
            let mut need_to_set_bp = false;

            if let Some(bp_key) = current_breakpoint {
                let bp_pair = bp_hm.get(bp_key).cloned().unwrap();

                let is_the_same_bp = match bp_pair {
                    (lower, Some(higher)) => window()
                        .match_media(&format!(
                            "(min-width: {}px) and (max-width: {}px",
                            lower,
                            higher - 1
                        ))
                        .unwrap()
                        .unwrap()
                        .matches(),
                    (lower, _) => window()
                        .match_media(&format!("(min-width: {}px)", lower))
                        .unwrap()
                        .unwrap()
                        .matches(),
                };

                if is_the_same_bp {
                    orders.skip();
                } else {
                    need_to_set_bp = true;
                }
            } else {
                need_to_set_bp = true
            }

            if need_to_set_bp {
                let opt_bp = bp_hm.iter().find_map(|(bp_key, bp_pair)| {
                    let is_this_bp = match bp_pair {
                        (lower, Some(higher)) => window()
                            .match_media(&format!(
                                "(min-width: {}px) and (max-width: {}px)",
                                lower,
                                higher - 1
                            ))
                            .unwrap()
                            .unwrap()
                            .matches(),
                        (lower, _) => window()
                            .match_media(&format!("(min-width: {}px)", lower))
                            .unwrap()
                            .unwrap()
                            .matches(),
                    };

                    if is_this_bp {
                        Some(bp_key.clone())
                    } else {
                        None
                    }
                });
                *current_breakpoint = opt_bp.clone();
            }
        });
    });
}
