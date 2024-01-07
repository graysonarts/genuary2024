use std::cell::Cell;

use crate::framework::*;
use nannou::prelude::*;

#[derive(Default, PartialEq)]
pub enum RunMode {
    #[default]
    Production,
    Debug,
}
pub struct Model {
    pub run_mode: RunMode,
    pub label: String,
    pub save_frame: Cell<bool>,
    pub recording: bool,

    pub red_angle: f32,
    pub green_angle: f32,
    pub blue_angle: f32,
    pub red_speed: f32,
    pub green_speed: f32,
    pub blue_speed: f32,
    pub cycle: f32,
}

impl Model {
    pub fn new(debug: RunMode) -> Model {
        let start_deg = random_range(-180.0, 180.0);
        let deg_sep = random_range(-3.0, 3.0);
        let speed = random_range(-0.01, 0.01);
        let cycle = random_range(-1.0, 1.0);
        Model {
            run_mode: debug,
            red_angle: deg_to_rad(start_deg),
            blue_angle: deg_to_rad(start_deg + deg_sep),
            green_angle: deg_to_rad(start_deg - deg_sep),
            red_speed: speed,
            blue_speed: speed,
            green_speed: speed,
            cycle,
            ..Default::default()
        }
    }

    pub fn new_values(&mut self) {
        let speed = random_range(-0.01, 0.01);
        let start_deg = random_range(-180.0, 180.0);
        let deg_sep = random_range(-3.0, 3.0);
        let cycle = random_range(-1.0, 1.0);

        self.red_angle = start_deg;
        self.green_angle = self.red_angle + deg_sep;
        self.blue_angle = self.red_angle - deg_sep;
        self.red_speed = speed;
        self.blue_speed = speed;
        self.green_speed = speed;
        self.cycle = cycle;
    }

    pub fn next_run_mode(&mut self) {
        self.run_mode = match self.run_mode {
            RunMode::Production => RunMode::Debug,
            RunMode::Debug => RunMode::Production,
        }
    }

    pub fn save_next_frame(&self) {
        self.save_frame.set(true);
    }

    pub fn save_current_frame(&self, app: &App, frame: &Frame) {
        let capture_path = captured_frame_path(app, frame);
        app.main_window().capture_frame(capture_path);
        self.save_frame.set(false);
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            run_mode: Default::default(),
            save_frame: Default::default(),
            red_angle: Default::default(),
            green_angle: Default::default(),
            blue_angle: Default::default(),
            red_speed: Default::default(),
            green_speed: Default::default(),
            blue_speed: Default::default(),
            recording: true,
            cycle: Default::default(),
            label: projname().unwrap_or_else(|| "unlabeled".to_owned()),
        }
    }
}
