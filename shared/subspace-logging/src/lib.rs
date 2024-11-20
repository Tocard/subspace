
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

pub fn init_logger(json_logging_enabled: bool) {
    // TODO: Workaround for https://github.com/tokio-rs/tracing/issues/2214, also on
    //  Windows terminal doesn't support the same colors as bash does
    let enable_color = if cfg!(windows) {
        false
    } else {
        supports_color::on(supports_color::Stream::Stderr).is_some()
    };
    if json_logging_enabled {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .with_ansi(enable_color)
                    .json() // Use JSON formatting
                    .with_filter(
                        EnvFilter::builder()
                            .with_default_directive(LevelFilter::INFO.into())
                            .from_env_lossy(),
                    ),
            )
            .init();
        info!("JSON logger successfully launched");
    } else {
        tracing_subscriber::registry()
            .with(
                fmt::layer().with_ansi(enable_color).with_filter(
                    EnvFilter::builder()
                        .with_default_directive(LevelFilter::INFO.into())
                        .from_env_lossy(),
                ),
            )
            .init();
        info!("Logger successfully launched");
    }
}
