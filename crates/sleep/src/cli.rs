use std::str::FromStr;

use clap::{Parser, ValueEnum, command};
use shared::prelude::*;

#[derive(Parser, Debug)]
#[command(
    name = shared::crate_name!(),
    author = shared::crate_authors!(),
    version = shared::crate_version!(),
    about = shared::crate_description!(),
    long_about = "\n\
    This is a simple command line tool to hibernate or sleep the system.\n\
    It uses the Windows API to set the system to sleep or hibernate mode.\n\
    \n\
    ",
    arg_required_else_help = true,
    styles = get_styles(),
    disable_version_flag = true,
)]
#[rustfmt::skip]
pub struct Cli {

    /// Timeout duration in seconds for how long before the system sleeps
    #[arg( short = 't', long = "timeout", help = "Timeout duration in seconds for how long before the system sleeps" )]
    pub timeout: Option<u64>,

    /// Should hibernate - internal param.
    #[arg(short = 'n', long = "hibernate", default_value_t = false, help = "Internal - Should hibernate")]
    pub hibernate: bool,

    /// Whether to force the operation or not
    #[arg(short = 'f', long = "force", default_value_t = false, help = "Internal - Whether to force the operation or not")]
    pub force: bool,

    /// Whether wake events are allowed or not
    #[arg(short = 'w', long = "wake-events", default_value_t = false, help = "Internal - Whether wake events are allowed or not")]
    pub wake_events: bool,


    /// Optional verbosity level of the logger.
    /// You may provide this as either a string or a number.
    ///
    /// The least verbose as 0 (Error -> Error Only)
    /// Most verbose as 4 (Trace -> Trace Everything)
    /// If not provided, the default value is "INFO".
    #[arg(value_enum, name = "level_verbosity", short = 'l', long = "level_verbosity", help = "The verbosity level of the logger.", required = false, default_value = "INFO", value_hint = clap::ValueHint::Other)]
    pub level_verbosity: Option<VerbosityLevel>,

    /// Other version flag
    #[arg(short = 'v', long = "version", help = "Prints version information")]
    pub version: bool,
}

impl Cli {
    pub fn new() -> Self {
        let mut s = Self::parse();
        if s.version {
            println!("{} {}", crate::crate_name!(), crate::crate_version!());
            std::process::exit(0);
        }
        if s.timeout.is_none() {
            s.timeout = Some(10); // default_value_t
        }
        s
    }

    #[inline]
    #[allow(dead_code)]
    pub fn verbosity_level(&self) -> VerbosityLevel {
        self.level_verbosity.unwrap_or(VerbosityLevel::Info)
    }
}

/// The verbosity level of the logger.
///
/// The least verbose as 0 (Error -> Error Only)
/// Most verbose as 4 (Trace -> Trace Everything).
#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[clap(name = "VerbosityLevel", rename_all = "upper")]
pub enum VerbosityLevel {
    #[value(name = "ERROR", alias = "error", alias = "Error", alias = "0")]
    Error,
    #[value(name = "WARN", alias = "warn", alias = "Warn", alias = "1")]
    Warn,
    #[value(name = "INFO", alias = "info", alias = "Info", alias = "2")]
    Info,
    #[value(name = "DEBUG", alias = "debug", alias = "Debug", alias = "3")]
    Debug,
    #[value(name = "TRACE", alias = "trace", alias = "Trace", alias = "4")]
    Trace,
}

// impl From<VerbosityLevel> for tracing_subscriber::filter::EnvFilter {
//     #[inline]
//     fn from(level: VerbosityLevel) -> Self {
//         match level {
//             VerbosityLevel::Error => tracing_subscriber::filter::EnvFilter::new("ERROR"),
//             VerbosityLevel::Warn => tracing_subscriber::filter::EnvFilter::new("WARN"),
//             VerbosityLevel::Info => tracing_subscriber::filter::EnvFilter::new("INFO"),
//             VerbosityLevel::Debug => tracing_subscriber::filter::EnvFilter::new("DEBUG"),
//             VerbosityLevel::Trace => tracing_subscriber::filter::EnvFilter::new("TRACE"),
//         }
//     }
// }

impl From<u8> for VerbosityLevel {
    #[inline]
    fn from(level: u8) -> Self {
        match level {
            0 => VerbosityLevel::Error,
            1 => VerbosityLevel::Warn,
            2 => VerbosityLevel::Info,
            3 => VerbosityLevel::Debug,
            4 => VerbosityLevel::Trace,
            _ => VerbosityLevel::Info,
        }
    }
}

impl FromStr for VerbosityLevel {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "ERROR" => Ok(VerbosityLevel::Error),
            "WARN" => Ok(VerbosityLevel::Warn),
            "INFO" => Ok(VerbosityLevel::Info),
            "DEBUG" => Ok(VerbosityLevel::Debug),
            "TRACE" => Ok(VerbosityLevel::Trace),
            _ => Err(Error::Generic(format!("Verbosity level: {} is not supported.", s))),
        }
    }
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))), // When a command is inc. This is the tag collor for 'Usage:'
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))), // Main headers in the help menu (e.g. Arguments, Options)
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite))), // Strings for args etc { -t, --total }
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)))
                .effects(anstyle::Effects::ITALIC),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .placeholder(anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))))
}
