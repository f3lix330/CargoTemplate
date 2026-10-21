use std::fs::File;
use log::{info, LevelFilter};
use simplelog::Config;
use anyhow::Result;

fn main() -> Result<()> {
    simplelog::WriteLogger::init(LevelFilter::Info, Config::default(), File::create(format!("{}.log", chrono::Local::now().to_rfc3339().replace(":", "-")))?)?;
    info!("Application started");

    Ok(())
}
