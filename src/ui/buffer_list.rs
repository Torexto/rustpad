use crate::AppWindow;
use gpui_kit::component::Selectable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::{IntoElement, ParentElement, Styled, WeakEntity, div, px};

pub fn buffer_list(
    tabs: Vec<(usize, String, bool)>,
    app: WeakEntity<AppWindow>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .w_full()
        .h(px(40.))
        .px_2()
        .border_b_1()
        .children(tabs.into_iter().map(move |(index, title, active)| {
            let select_app = app.clone();
            let close_app = app.clone();
            div()
                .flex()
                .items_center()
                .child(
                    Button::new(("buffer-tab", index))
                        .label(title)
                        .ghost()
                        .compact()
                        .selected(active)
                        .on_click(move |_, window, cx| {
                            let _ = select_app
                                .update(cx, |app, cx| app.select_buffer(index, window, cx));
                        }),
                )
                .child(
                    Button::new(("close-buffer", index))
                        .label("×")
                        .ghost()
                        .compact()
                        .on_click(move |_, window, cx| {
                            let _ =
                                close_app.update(cx, |app, cx| app.close_buffer(index, window, cx));
                        }),
                )
        }))
}
