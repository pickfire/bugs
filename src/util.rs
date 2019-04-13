use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logger() {
    let colors = ColoredLevelConfig::new()
        // .error(Color::Red)
        // .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::BrightBlack);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<5}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("gfx_device_gl", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
