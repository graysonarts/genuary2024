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
}

impl Model {
    pub fn new(debug: RunMode) -> Model {
        Model {
            run_mode: debug,
            red_angle: random_range(deg_to_rad(0.0), deg_to_rad(360.0)),
            blue_angle: random_range(deg_to_rad(0.0), deg_to_rad(360.0)),
            green_angle: random_range(deg_to_rad(0.0), deg_to_rad(360.0)),
            red_speed: random_range(-0.05, 0.05),
            blue_speed: random_range(-0.05, 0.05),
            green_speed: random_range(-0.05, 0.05),
            recording: true,
            ..Default::default()
        }
    }

    pub fn new_values(&mut self) {
        self.red_speed = random_range(-0.05, 0.05);
        self.blue_speed = random_range(-0.05, 0.05);
        self.green_speed = random_range(-0.05, 0.05);
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
            recording: false,
            label: projname().unwrap_or_else(|| "unlabeled".to_owned()),
        }
    }
}
