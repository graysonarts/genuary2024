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
}

impl Model {
    pub fn new(debug: RunMode) -> Model {
        Model {
            run_mode: debug,
            ..Default::default()
        }
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
            label: projname().unwrap_or_else(|| "unlabeled".to_owned()),
        }
    }
}
