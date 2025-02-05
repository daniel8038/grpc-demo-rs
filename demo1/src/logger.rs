use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
pub fn init_logger() {
    let file_appender = RollingFileAppender::new(Rotation::HOURLY, "logs", "bot.log");
    let file_layer = Layer::new()
        .with_file(true)
        .with_writer(file_appender)
        .pretty()
        .with_filter(LevelFilter::INFO);
    let console_layer = Layer::new()
        .with_file(true)
        .with_writer(std::io::stdout)
        .pretty()
        .with_filter(LevelFilter::INFO);
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
}
