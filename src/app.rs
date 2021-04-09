use crate::net::Event;
use eframe::{egui::*, epi};

const EVENTS: &[Event] = &[Event::default_drive()];

pub struct App {
    pub events: Vec<Event>,
    pub event_index: usize,
    pub event: Event,
}

impl App {
    pub fn new() -> Self {
        Self {
            events: vec![
                Event::Drive {
                    speed: 30.0,
                    seconds: 5.0,
                    angle: 20.0,
                },
                Event::Drive {
                    speed: 15.0,
                    seconds: 10.0,
                    angle: -15.0,
                },
            ],
            event_index: 0,
            event: EVENTS[0].clone(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Rover Controller"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        SidePanel::left("side_panel", 100.0).show(ctx, |ui| {
            ui.heading("Rover");

            ui.separator();

            ui.label("Event queue:");

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    for event in &mut self.events {
                        ui.group(|ui| {
                            event.ui(ui);
                        });
                    }
                });
            });
        });

        Window::new("Input").show(ctx, |ui| {
            let prev = self.event_index;

            ComboBox::from_id_source("event_selections").show_index(
                ui,
                &mut self.event_index,
                EVENTS.len(),
                |i| EVENTS[i].name().into(),
            );

            if self.event_index != prev {
                self.event = EVENTS[self.event_index].clone();
            }

            self.event.ui(ui);
        });
    }
}
