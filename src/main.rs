use std::io;
slint::include_modules!();

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Node {
    radius: i32,
    color: Color,
    x: u32;
    y: u32;
}

fn main() {
    let ui = AppWindow::new();

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 0);
    });

    ui.run();
}
