use std::marker::PhantomData;

use crate::prelude::*;

#[derive(Component)]
pub struct ToolDivider<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> ToolDivider<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Component<S> {
        let theme = theme(cx);

        div().w_px().h_3().bg(theme.border)
    }
}
