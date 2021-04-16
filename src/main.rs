#![allow(dead_code)]

mod app;
mod net;
mod rover;

#[cfg(not(feature = "rover"))]
fn main() {
    eframe::run_native(Box::new(app::App::new().unwrap()));
}

#[cfg(feature = "rover")]
#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut rover = rover::Rover::connect("localhost:35566").await?;

    rover.run().await?;

    Ok(())
}
