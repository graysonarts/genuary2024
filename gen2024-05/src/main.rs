mod framework;
mod model;
use framework::*;
use model::*;
use nannou::{
    lyon::{
        geom::euclid::{Point2D, UnknownUnit},
        path::Path,
    },
    prelude::*,
};
use once_cell::sync::Lazy;
use settings::{BORDER_SIZE, LINE_SIZE, ORIGIN_OFFSET, SIZE, UNIT_SIZE};

#[cfg(not(feature = "small"))]
mod settings {
    use nannou::glam::Vec3;
    use once_cell::sync::Lazy;

    pub const SCALE: u32 = 2;
    pub const SIZE: u32 = 512 * SCALE;
    pub const UNIT_SIZE: usize = 70 * SCALE as usize;
    pub const BORDER_SIZE: f32 = 20.0 * SCALE as f32;
    pub const LINE_SIZE: f32 = 25.0 * SCALE as f32;
    pub const OFFSET_START: f32 = 0.25;
    pub const OFFSET_END: f32 = 0.75;
    pub static ORIGIN_OFFSET: Lazy<Vec3> =
        Lazy::new(|| Vec3::new((SIZE as f32) / 2.0, (SIZE as f32) / 2.0, 0.0));
}
#[cfg(feature = "small")]
mod settings {
    use nannou::glam::Vec3;
    use once_cell::sync::Lazy;

    pub const SIZE: u32 = 512;
    pub const UNIT_SIZE: usize = 70;
    pub const BORDER_SIZE: f32 = 20.0;
    pub const LINE_SIZE: f32 = 25.0;
    pub const OFFSET_START: f32 = 0.25;
    pub const OFFSET_END: f32 = 0.75;
    pub static ORIGIN_OFFSET: Lazy<Vec3> =
        Lazy::new(|| Vec3::new((SIZE as f32) / 2.0, (SIZE as f32) / 2.0, 0.0));
}

pub static BACKGROUND: Lazy<Srgb<u8>> = Lazy::new(|| ANTIQUEWHITE);
pub static BLACK_COLOR: Lazy<Srgb<u8>> = Lazy::new(|| BLACK);
pub static ACCCENT_COLOR: Lazy<Srgb<u8>> = Lazy::new(|| ROYALBLUE);

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

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(*BACKGROUND);

    model
        .units
        .iter()
        .enumerate()
        .for_each(|(idx, unit)| draw_unit(&draw.translate(-*ORIGIN_OFFSET), idx, unit));

    label(app, model, &draw);
    draw.to_frame(app, &frame).unwrap();

    if model.save_frame.get() {
        model.save_current_frame(app, &frame);
    }
}

fn point_from_vec(v: &Vec2) -> Point2D<f32, UnknownUnit> {
    Point2D::new(v.x, v.y)
}

fn draw_unit(draw: &Draw, idx: usize, unit: &Unit) {
    let x_position = (UNIT_SIZE * idx) as f32 + (UNIT_SIZE as f32) / 2.0 + BORDER_SIZE / 2.0;
    let top_position = Vec2::new(x_position, BORDER_SIZE);
    let bottom_position = Vec2::new(x_position, SIZE as f32 - BORDER_SIZE);
    let arc_y = (bottom_position.y - top_position.y) * unit.arc_offset;
    let arc_start = Vec2::new(x_position, arc_y - UNIT_SIZE as f32 / 2.0);
    let arc_end = Vec2::new(x_position, arc_y + UNIT_SIZE as f32 / 2.0);

    let ctrl_point = Point2D::new(x_position + UNIT_SIZE as f32 / 1.33 - 10.0, arc_y);

    let mut b = Path::builder();
    b.begin(point_from_vec(&top_position));
    b.line_to(point_from_vec(&arc_start));
    // TODO: Find the right placement for ctrl
    b.quadratic_bezier_to(ctrl_point, point_from_vec(&arc_end));
    b.line_to(point_from_vec(&bottom_position));
    b.end(false);
    let path = b.build();

    // let path = Path::builder()
    //     .begin(top_position)
    //     .line_to(arc_start)
    //     .line_to(arc_end)
    //     .line_to(bottom_position)
    //     .end(false) // When they finally implement this
    //     .build();

    draw.path()
        .stroke()
        .stroke_weight(LINE_SIZE)
        .color(unit.color)
        .events(path.iter());

    #[cfg(feature = "debug")]
    {
        draw.ellipse()
            .color(RED)
            .radius(3.0)
            .x_y(arc_start.x, arc_start.y)
            .finish();
        draw.ellipse()
            .color(RED)
            .radius(3.0)
            .x_y(arc_end.x, arc_end.y)
            .finish();
    }

    // draw.ellipse()
}

pub fn sketch_key_released(_app: &App, _model: &mut Model, _key: Key) {
    // This will be called if the framework key released handles doesn't handle the key pressed.
}
