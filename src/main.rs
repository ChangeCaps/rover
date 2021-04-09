#![allow(dead_code)]

mod app;
mod net;

#[cfg(not(feature = "rover"))]
fn main() {
    eframe::run_native(Box::new(app::App::new()));
}

#[cfg(feature = "rover")]
fn main() {}
