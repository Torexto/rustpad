use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::{Entity, IntoElement, ParentElement, Styled, div};

pub fn editor(state: &Entity<TextareaState>, enabled: bool) -> impl IntoElement {
    div().flex_1().min_h_0().child(
        Textarea::new(state)
            .w_full()
            .h_full()
            .bordered(false)
            .disabled(!enabled),
    )
}
