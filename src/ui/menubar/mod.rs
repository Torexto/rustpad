use crate::AppWindow;
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::{IntoElement, ParentElement, Styled, WeakEntity, div, px};

#[derive(Clone, Copy)]
enum FileCommand {
    New,
    Open,
    Reload,
    Save,
    Close,
}

pub fn menubar(app: WeakEntity<AppWindow>, has_active_buffer: bool) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_1()
        .w_full()
        .h(px(38.))
        .px_2()
        .border_b_1()
        .child(
            Button::new("menu-file")
                .label("File")
                .ghost()
                .compact()
                .dropdown_menu(move |menu, _, _| file_menu(menu, app.clone(), has_active_buffer)),
        )
        .child(empty_menu_button("menu-edit", "Edit"))
        .child(empty_menu_button("menu-search", "Search"))
        .child(empty_menu_button("menu-view", "View"))
        .child(empty_menu_button("menu-encoding", "Encoding"))
        .child(empty_menu_button("menu-language", "Language"))
        .child(empty_menu_button("menu-settings", "Settings"))
        .child(empty_menu_button("menu-tools", "Tools"))
        .child(empty_menu_button("menu-run", "Run"))
        .child(empty_menu_button("menu-help", "Help"))
}

fn empty_menu_button(id: &'static str, label: &'static str) -> impl IntoElement {
    Button::new(id)
        .label(label)
        .ghost()
        .compact()
        .dropdown_menu(|menu, _, _| menu.item(PopupMenuItem::label("No commands yet")))
}

fn file_menu(menu: PopupMenu, app: WeakEntity<AppWindow>, has_active_buffer: bool) -> PopupMenu {
    menu.item(command_item(
        app.clone(),
        "New",
        "Ctrl+N",
        true,
        FileCommand::New,
    ))
    .item(command_item(
        app.clone(),
        "Open...",
        "Ctrl+O",
        true,
        FileCommand::Open,
    ))
    .item(PopupMenuItem::label("Open Containing Folder"))
    .item(PopupMenuItem::label("Open Folder as Workspace").disabled(true))
    .separator()
    .item(command_item(
        app.clone(),
        "Reload from Disk",
        "Ctrl+R",
        has_active_buffer,
        FileCommand::Reload,
    ))
    .item(command_item(
        app.clone(),
        "Save",
        "Ctrl+S",
        has_active_buffer,
        FileCommand::Save,
    ))
    .item(shortcut_item("Save As...", "Ctrl+Alt+S", has_active_buffer))
    .item(shortcut_item("Save All", "Ctrl+Shift+S", has_active_buffer))
    .separator()
    .item(PopupMenuItem::label("Rename").disabled(true))
    .item(command_item(
        app.clone(),
        "Close",
        "Ctrl+W",
        has_active_buffer,
        FileCommand::Close,
    ))
    .item(shortcut_item(
        "Close All",
        "Ctrl+Shift+W",
        has_active_buffer,
    ))
    .item(PopupMenuItem::label("Remove").disabled(true))
    .separator()
    .item(shortcut_item("Print", "Ctrl+P", has_active_buffer))
    .separator()
    .item(shortcut_item("Exit", "Alt+F4", true))
}

fn shortcut_item(label: &'static str, shortcut: &'static str, enabled: bool) -> PopupMenuItem {
    menu_item(label, shortcut).disabled(!enabled)
}

fn command_item(
    app: WeakEntity<AppWindow>,
    label: &'static str,
    shortcut: &'static str,
    enabled: bool,
    command: FileCommand,
) -> PopupMenuItem {
    let app_for_click = app.clone();
    menu_item(label, shortcut)
        .disabled(!enabled)
        .on_click(move |_, window, cx| {
            let _ = app_for_click.update(cx, |app, cx| match command {
                FileCommand::New => app.new_buffer(window, cx),
                FileCommand::Open => app.open_file(window, cx),
                FileCommand::Reload => app.reload_file(window, cx),
                FileCommand::Save => app.save_file(),
                FileCommand::Close => app.close_active_buffer(window, cx),
            });
        })
}

fn menu_item(label: &'static str, shortcut: &'static str) -> PopupMenuItem {
    PopupMenuItem::element(move |_, cx| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w(px(260.))
            .gap_x_6()
            .child(div().child(label))
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(shortcut),
            )
    })
}
