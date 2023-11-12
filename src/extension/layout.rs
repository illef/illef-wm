use std::iter::repeat;

use penrose::{
    builtin::layout::messages::{ExpandMain, ShrinkMain},
    core::layout::{Layout, Message},
    impl_message,
    pure::{geometry::Rect, Stack},
    Xid,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FocusShrink;
impl_message!(FocusShrink);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FocusExpand;
impl_message!(FocusExpand);

/// A simple port of ResizableTall of Xmonad
#[derive(Debug, Clone)]
pub struct ResizableTall {
    ratio: f32,
    ratio_step: f32,
    stack_ratio: Vec<f32>,
    focus_expand_required: Option<bool>,
    focus_shrink_required: Option<bool>,
}

impl Default for ResizableTall {
    fn default() -> Self {
        Self {
            ratio: 0.6,
            ratio_step: 0.1,
            stack_ratio: vec![],
            focus_expand_required: None,
            focus_shrink_required: None,
        }
    }
}

impl ResizableTall {
    pub fn new(ratio: f32, ratio_step: f32) -> Self {
        Self {
            ratio,
            ratio_step,
            stack_ratio: vec![],
            focus_shrink_required: None,
            focus_expand_required: None,
        }
    }
    pub fn boxed(self) -> Box<dyn Layout> {
        Box::new(self)
    }
    pub fn boxed_default() -> Box<dyn Layout> {
        Box::<Self>::default()
    }

    fn ratio(&self) -> f32 {
        self.ratio
    }

    fn layout_this<T: PartialEq + Copy>(&mut self, s: &Stack<T>, r: Rect) -> Vec<(T, Rect)> {
        let n = s.len();

        if n == 1 {
            r.as_rows(n as u32)
                .iter()
                .zip(s)
                .map(|(r, c)| (*c, *r))
                .collect()
        } else {
            assert!(n - 1 > 0);
            // We have two stacks so split the screen in two and then build a stack for each
            let (main, stack) = r
                .split_at_width_perc(self.ratio())
                .expect("split point to be valid");

            // self.slaves_ratio 수는 stack_rows 수 보다 크거나 같아야 한다
            self.stack_ratio
                .extend(repeat(1.0).take((n - 1).saturating_sub(self.stack_ratio.len())));

            // slave_ratio 를 수정한다
            self.adjust_stack_ratio(s, n - 1);

            let stack_rows = Self::make_stack_rects(&stack, &self.stack_ratio, n - 1);

            main.as_rows(1)
                .into_iter()
                .chain(stack_rows)
                .zip(s)
                .map(|(r, c)| (*c, r))
                .collect()
        }
    }

    fn make_stack_rects(r: &Rect, ratios: &Vec<f32>, size: usize) -> Vec<Rect> {
        if size <= 1 {
            return vec![*r];
        }

        let total_ratio: f32 = ratios.iter().take(size).sum();

        let mut y = r.y;
        let mut stack_rects = vec![];

        for ratio in ratios {
            let h = (r.h as f32 * (ratio / total_ratio)).floor() as u32;
            stack_rects.push(Rect::new(r.x, y, r.w, h));
            y += h;
        }

        if y - r.y < r.h {
            let remain_h = r.h - (y - r.y);
            let last_r = stack_rects.last_mut().unwrap();
            last_r.h = last_r.h + remain_h
        }

        stack_rects
    }

    fn adjust_stack_ratio<T: PartialEq>(&mut self, s: &Stack<T>, stack_size: usize) {
        let ratio_step = if let Some(_) = self.focus_expand_required.take() {
            Some(self.ratio_step)
        } else if let Some(_) = self.focus_shrink_required.take() {
            Some(self.ratio_step * -1.0)
        } else {
            None
        };

        if let Some(rs) = ratio_step {
            // focus 가 Main 이거나, stack 크기가 1 이하라면 무시한다
            if stack_size > 1 && s.head() != s.focused() {
                let slave_xid_vec: Vec<&T> = s.iter().skip(1).collect();
                let focus_idx = slave_xid_vec
                    .iter()
                    .position(|x| *x == s.focused())
                    .unwrap();

                // focus 된 slave 의 크기는 항상 증가
                self.stack_ratio[focus_idx] += rs;
                if self.stack_ratio[focus_idx] < 0.0 {
                    self.stack_ratio[focus_idx] = 0.0
                }
                // case 1 - focus 가 stack row 의 가장 아래일 경우, focus 바로 위 window 크기를 감소
                let shrink_idx = if s.last() == s.focused() {
                    focus_idx - 1
                }
                // case 2 - 그 이외의 경우, focus 바로 아래 window 크기를 감소
                else {
                    focus_idx + 1
                };
                self.stack_ratio[shrink_idx] -= rs;
                if self.stack_ratio[shrink_idx] < 0.0 {
                    self.stack_ratio[shrink_idx] = 0.0
                }
            }
        }
    }
}

impl Layout for ResizableTall {
    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(self.clone())
    }

    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        (None, self.layout_this(s, r))
    }

    fn handle_message(&mut self, m: &Message) -> Option<Box<dyn Layout>> {
        if let Some(&ExpandMain) = m.downcast_ref() {
            self.ratio += self.ratio_step;
            if self.ratio > 1.0 {
                self.ratio = 1.0;
            }
        } else if let Some(&ShrinkMain) = m.downcast_ref() {
            self.ratio -= self.ratio_step;
            if self.ratio < 0.0 {
                self.ratio = 0.0;
            }
        } else if let Some(&FocusExpand) = m.downcast_ref() {
            self.focus_expand_required = Some(true);
        } else if let Some(&FocusShrink) = m.downcast_ref() {
            self.focus_shrink_required = Some(true);
        }
        None
    }

    fn name(&self) -> String {
        return "ResizableTall".to_owned();
    }
}
