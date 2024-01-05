use std::cell::Cell;

use crate::{
    framework::*,
    settings::{BORDER_SIZE, OFFSET_END, OFFSET_START, SIZE},
    ACCCENT_COLOR, BLACK_COLOR, UNIT_SIZE,
};
use nannou::{
    prelude::*,
    rand::{thread_rng, Rng},
};

#[derive(Default, PartialEq)]
pub enum RunMode {
    #[default]
    Production,
    Debug,
}

pub struct Unit {
    // percentage of height
    pub arc_offset: f32,
    pub color: Srgb<u8>,
}

impl Unit {
    pub fn new(idx: usize) -> Unit {
        let mut rng = thread_rng();
        let arc_offset = OFFSET_START + (OFFSET_END - OFFSET_START) * rng.gen::<f32>();
        Self {
            arc_offset,
            color: match idx % 2 {
                0 => *BLACK_COLOR,
                _ => *ACCCENT_COLOR,
            },
        }
    }
}

pub struct Model {
    pub run_mode: RunMode,
    pub label: String,
    pub save_frame: Cell<bool>,

    // Sketch Specific
    pub units: Vec<Unit>,
}

fn initialize_units() -> Vec<Unit> {
    let num_units = (((SIZE as f32) - BORDER_SIZE * 2.0) / UNIT_SIZE as f32) as usize + 1;
    // let num_units = 1;
    let mut units = Vec::with_capacity(num_units);

    (0..num_units).for_each(|idx| units.push(Unit::new(idx)));

    return units;
}

impl Model {
    pub fn new(debug: RunMode) -> Model {
        Model {
            run_mode: debug,
            units: initialize_units(),
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
            units: initialize_units(),
        }
    }
}
