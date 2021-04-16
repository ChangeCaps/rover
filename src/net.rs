use eframe::egui::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Respone {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Drive {
        speed: f32,
        seconds: f32,
        angle: f32,
    },
}

impl Event {
    pub const fn default_drive() -> Self {
        Self::Drive {
            speed: 0.0,
            seconds: 0.0,
            angle: 0.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Event::Drive { .. } => "Drive",
        }
    }

    pub fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        match self {
            Event::Drive {
                speed,
                seconds,
                angle,
            } => {
                ui.heading("Drive");
                ui.horizontal(|ui| {
                    ui.label("Speed");
                    ui.add(DragValue::new(speed));
                });
                ui.horizontal(|ui| {
                    ui.label("Time");
                    ui.add(DragValue::new(seconds));
                });
                ui.horizontal(|ui| {
                    ui.label("Angle");
                    ui.add(DragValue::new(angle));
                });
            }
        }
    }
}
