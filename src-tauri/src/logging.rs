
use simplelog::*;
use std::fs::File;

pub fn setup_logging() -> Result<(), String> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("newgan.log").map_err(|e| e.to_string())?),
        ]
    ).map_err(|e| e.to_string())?;
    Ok(())
}
