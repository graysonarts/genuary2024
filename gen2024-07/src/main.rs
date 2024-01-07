mod framework;
mod model;
use framework::*;
use model::*;
use nannou::prelude::*;

#[cfg(feature = "small")]
const SIZE: u32 = 512;
#[cfg(not(feature = "small"))]
const SIZE: u32 = 1024;

const MOVEMENT_SCALE: f32 = 25.0;

fn main() {
    setup_logging();
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    // Create the main window which is 1024x1024 my standard size for Genuary Sketches
    app.new_window()
        .size(SIZE, SIZE)
        .key_released(key_released)
        .build()
        .unwrap();

    Model::new(RunMode::Production)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.red_angle = (model.red_angle + model.red_speed) % deg_to_rad(360.0);
    model.green_angle = (model.green_angle + model.green_speed) % deg_to_rad(360.0);
    model.blue_angle = (model.blue_angle + model.blue_speed) % deg_to_rad(360.0);

    if update.since_start.as_secs() > 60 {
        model.recording = false
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let base_draw = app.draw();
    base_draw.background().color(Srgb::<u8>::new(10, 25, 14));
    let draw = base_draw.color_blend(BLEND_ADD);

    draw_circle_sin(&draw, Srgb::new(255, 0, 0), model.red_angle);
    draw_circle_sin(&draw, Srgb::new(0, 255, 0), model.green_angle);
    draw_circle_sin(&draw, Srgb::new(0, 0, 255), model.blue_angle);

    label(app, model, &base_draw);
    base_draw.to_frame(app, &frame).unwrap();

    if model.save_frame.get() || model.recording {
        model.save_current_frame(app, &frame);
    }
}

fn draw_circle_sin(draw: &Draw, color: Srgb<u8>, angle: f32) {
    draw.ellipse()
        .stroke_color(color)
        .no_fill()
        .stroke_weight(3.0)
        .x_y(angle.cos() * MOVEMENT_SCALE, angle.sin() * MOVEMENT_SCALE)
        .radius(SIZE as f32 / 3.0)
        .finish();
}

pub fn sketch_key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.new_values();
        }
        _ => {}
    }
    // This will be called if the framework key released handles doesn't handle the key pressed.
}
