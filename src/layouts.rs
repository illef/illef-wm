//! Configuration of our layouts and custom layout algorithms
use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReserveTop},
        MainAndStack, Monocle,
    },
    core::layout::LayoutStack,
    stack,
};

pub fn layouts() -> LayoutStack {
    let max_main = 1;
    let ratio = 0.6;
    let ratio_step = 0.1;

    stack!(
        MainAndStack::side(max_main, ratio, ratio_step),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, 0, 0), 35))
}
