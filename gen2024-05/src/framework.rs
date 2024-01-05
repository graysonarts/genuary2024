use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use nannou::{color::Alpha, prelude::*};

use crate::{
    model::{Model, RunMode},
    sketch_key_released,
};

pub fn projname() -> Option<String> {
    let result = std::env::current_exe()
        .ok()
        .as_ref()
        .map(PathBuf::as_path)
        .map(Path::file_stem)?
        .map(OsStr::to_string_lossy)
        .map(|name| format!("{} {}", name, env!("VERGEN_GIT_DESCRIBE")));

    result
}

pub fn label(app: &App, model: &Model, draw: &Draw) -> () {
    let win_rect = app.main_window().rect();

    let text_rect = win_rect.pad_bottom(20.0).pad_right(10.0);

    if model.run_mode == RunMode::Debug {
        // Draw Origin
        draw.line()
            .start([-10.0, 0.0].into())
            .end([10.0, 0.0].into())
            .color(WHITE);
        draw.line()
            .start([0.0, -10.0].into())
            .end([0.0, 10.0].into())
            .color(WHITE);
    }

    // Draw bottom bar
    let bottom_bar_rect = Rect::from_x_y_w_h(0.0, win_rect.bottom() + 14.0, win_rect.w(), 27.0);
    draw.rect()
        .xy(bottom_bar_rect.xy())
        .wh(bottom_bar_rect.wh())
        .color(Alpha {
            color: BLACK,
            alpha: 0.75,
        });

    // Draw Label
    match model.run_mode {
        RunMode::Production => draw.text(&model.label),
        RunMode::Debug => draw.text(&format!("(DEBUG) {}", model.label)),
    }
    .color(WHITE)
    .font_size(12)
    .align_text_bottom()
    .right_justify()
    .wh(text_rect.wh());
}

pub fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => model.save_next_frame(),
        Key::M => model.next_run_mode(),
        _ => sketch_key_released(app, model, key),
    };
}

pub fn setup_logging() {
    pretty_env_logger::init();
}

// From nannou example code
pub fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
