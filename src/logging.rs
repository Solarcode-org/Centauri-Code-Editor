use color_eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer as _};

use crate::config::{get_data_dir, init_config};

pub fn initialize_logging() -> Result<()> {
    let config = init_config();

    let directory = get_data_dir(&config);
    std::fs::create_dir_all(directory.clone())?;

    let log_path = directory.join(config.log_file.clone());

    println!("LOG PATH: {log_path:#?}");

    let log_file = std::fs::File::create(log_path)?;

    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG")
            .or_else(|_| std::env::var(config.log_env.clone()))
            .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
      (target: $target:expr, level: $level:expr, $ex:expr) => {{
          match $ex {
              value => {
                  tracing::event!(target: $target, $level, ?value, stringify!($ex));
              }
          }
      }};
      (level: $level:expr, $ex:expr) => {
          trace_dbg!(target: module_path!(), level: $level, $ex)
      };
      (target: $target:expr, $ex:expr) => {
          trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
      };
      ($ex:expr) => {
          trace_dbg!(level: tracing::Level::DEBUG, $ex)
      };
  }
