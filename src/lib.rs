pub mod buffer;
pub mod rustpad;
pub mod ui;

use gpui_kit::component::input::{InputEvent, TextareaState};
use gpui_kit::*;

pub struct AppWindow {
    rustpad: rustpad::Rustpad,
    editor: Entity<TextareaState>,
    _editor_subscription: Subscription,
}

impl Render for AppWindow {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app = cx.entity().downgrade();
        let active_buffer = self.rustpad.current_buffer().is_some();
        let tabs = self
            .rustpad
            .buffers()
            .iter()
            .enumerate()
            .map(|(index, buffer)| {
                (
                    index,
                    buffer.title().to_owned(),
                    self.rustpad.active_buffer_index() == Some(index),
                )
            })
            .collect::<Vec<_>>();

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(ui::menubar::menubar(app.clone(), active_buffer))
            .child(ui::buffer_list::buffer_list(tabs, app))
            .child(ui::editor::editor(&self.editor, active_buffer))
    }
}

impl AppWindow {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| TextareaState::new(window, cx));
        let editor_subscription = cx.subscribe(
            &editor,
            |this: &mut Self, editor, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    if let Some(buffer) = this.rustpad.current_buffer_mut() {
                        buffer.content = editor.read(cx).value().to_string();
                    }
                }
            },
        );

        Self {
            rustpad: rustpad::Rustpad::default(),
            editor,
            _editor_subscription: editor_subscription,
        }
    }

    pub(crate) fn new_buffer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rustpad.new_buffer();
        self.sync_editor(window, cx);
        cx.notify();
    }

    pub(crate) fn open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(error) = self.rustpad.open_file() {
            log::error!("Failed to open file: {error:#}");
        }
        self.sync_editor(window, cx);
        cx.notify();
    }

    pub(crate) fn save_file(&mut self) {
        if let Some(buffer) = self.rustpad.current_buffer() {
            buffer.save();
        }
    }

    pub(crate) fn reload_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(error) = self.rustpad.reload_current_buffer() {
            log::error!("Failed to reload file: {error:#}");
        }
        self.sync_editor(window, cx);
        cx.notify();
    }

    pub(crate) fn close_active_buffer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rustpad.remove_active_buffer();
        self.sync_editor(window, cx);
        cx.notify();
    }

    pub(crate) fn select_buffer(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.rustpad.set_buffer(index);
        self.sync_editor(window, cx);
        cx.notify();
    }

    pub(crate) fn close_buffer(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.rustpad.remove_buffer(index);
        self.sync_editor(window, cx);
        cx.notify();
    }

    fn sync_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let value = self
            .rustpad
            .current_buffer()
            .map(|buffer| buffer.content().to_owned())
            .unwrap_or_default();
        self.editor
            .update(cx, |editor, cx| editor.set_value(value, window, cx));
    }

    pub fn run() {
        application()
            .with_assets(assets::Assets)
            .run(|cx: &mut App| {
                init(cx);

                open_window(WindowOptions::default(), cx, |window, cx| {
                    cx.new(|cx| AppWindow::new(window, cx))
                })
                .expect("Failed to open window");
            })
    }
}
