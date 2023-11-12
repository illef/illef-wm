//! Configuration of our layouts and custom layout algorithms
use penrose::{
    builtin::layout::{
        transformers::{Gaps, ReserveTop},
        Monocle,
    },
    core::layout::LayoutStack,
    stack,
};

use crate::extension::layout::ResizableTall;

pub fn layouts() -> LayoutStack {
    let ratio = 0.6;
    let ratio_step = 0.05;

    stack!(
        ResizableTall::new(ratio, ratio_step).boxed(),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, 0, 0), 35))
}
