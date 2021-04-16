use crate::net::Event;
use eframe::{egui::*, epi};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const EVENTS: &[Event] = &[Event::default_drive()];

pub struct App {
    pub events: Vec<Event>,
    pub event_index: usize,
    pub event: Event,
    pub stream: Option<TcpStream>,
    pub listener: TcpListener,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let listener = TcpListener::bind("192.168.2.66:35566")?;
        listener.set_nonblocking(true)?;

        Ok(Self {
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
            stream: None,
            listener,
        })
    }
}

impl App {
    pub fn listen(&mut self) -> anyhow::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => {
                    if self.stream.is_none() {
                        s.set_nonblocking(true)?;
                        self.stream = Some(s);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    return Ok(());
                }
                e => {
                    e?;
                }
            }
        }

        Ok(())
    }

    pub fn recv(&self) -> anyhow::Result<Option<crate::net::Respone>> {
        if let Some(stream) = &self.stream {
            let f = || -> std::io::Result<Vec<u8>> {
                let mut len = [0; 4];
                stream.peek(&mut len)?;
                let len = u32::from_be_bytes(len) as usize;
                let mut data = vec![0; 4 + len];
                (&*stream).read(&mut data)?;
                Ok(data)
            };

            match f() {
                Ok(data) => Ok(Some(bincode::deserialize(&data[4..])?)),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
                Err(e) => Err(e.into()),
            }
        } else {
            Ok(None)
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Rover Controller"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        self.listen().unwrap();

        SidePanel::left("side_panel", 100.0).show(ctx, |ui| {
            ui.heading("Rover");

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.stream.is_some(), "Connected");
            });

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
