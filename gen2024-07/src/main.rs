mod framework;
mod model;
use framework::*;
use model::*;
use nannou::{
    geom::{Ellipse, Path},
    prelude::*,
};

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
    model.red_angle = cycle_angle(model.red_angle, model.red_speed);
    model.green_angle = cycle_angle(model.green_angle, model.green_speed);
    model.blue_angle = cycle_angle(model.blue_angle, model.blue_speed);
    model.cycle = cycle_cycle(model.cycle + 0.01);

    if update.since_start.as_secs() > 10 {
        model.recording = false
    }
}

fn cycle_angle(angle: f32, speed: f32) -> f32 {
    let mut value = angle + speed;
    if value < -2.0 * PI {
        value += 4.0 * PI
    } else if value > 2.0 * PI {
        value -= 4.0 * PI
    }

    value
}

fn cycle_cycle(value: f32) -> f32 {
    if value < -1.0 {
        value + 2.0
    } else if value > 1.0 {
        value - 2.0
    } else {
        value
    }
}

fn map(value: f32, low: f32, high: f32) -> f32 {
    low + ((high - low) / 4.0 * PI) * value
}

fn view(app: &App, model: &Model, frame: Frame) {
    let base_draw = app.draw();
    base_draw.background().color(Srgb::<u8>::new(10, 25, 14));
    let draw = base_draw.color_blend(BLEND_ADD);

    draw_circle_sin(&draw, Srgb::new(255, 0, 0), model.red_angle, model.cycle);
    draw_circle_sin(&draw, Srgb::new(0, 255, 0), model.green_angle, model.cycle);
    draw_circle_sin(&draw, Srgb::new(0, 0, 255), model.blue_angle, model.cycle);

    #[cfg(feature = "debug")]
    base_draw
        .text(&format!("{}", model.red_angle))
        .center_justify()
        .color(WHITE)
        .finish();

    label(app, model, &base_draw);
    base_draw.to_frame(app, &frame).unwrap();

    if model.save_frame.get() || model.recording {
        model.save_current_frame(app, &frame);
    }
}

fn draw_circle_sin(draw: &Draw, color: Srgb<u8>, angle: f32, cycle: f32) {
    let draw = draw.rotate(angle);
    // draw.ellipse()
    //     .stroke_color(color)
    //     .no_fill()
    //     .stroke_weight(3.0)
    //     .x_y(angle.cos() * MOVEMENT_SCALE, angle.sin() * MOVEMENT_SCALE)
    //     .radius(SIZE as f32 / 3.0)
    //     .finish();
    let size = SIZE as f32 / 3.0;
    let points: Vec<Point2> = (0..360)
        .map(|x| deg_to_rad(x as f32))
        // .map(|x| x + angle)
        .map(|a| {
            let offset = map(a, -PI / 2.0, PI / 2.0) + cycle;
            let mut div = offset + cycle;
            if div == 0.0 {
                div += 0.00001;
            }
            let x = (size + ((offset * MOVEMENT_SCALE).sin()) / (div)) * a.cos();
            let y = (size + ((offset * MOVEMENT_SCALE).sin()) / (div)) * a.sin();
            Point2::new(x, y)
        })
        .collect();
    draw.path().stroke().color(color).points_closed(points);
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
