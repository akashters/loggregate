// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(about = "Aggregates, Analyses and Generates reports from log files", long_about = None)]
pub struct Cli {
    /// Position of datetime's start on log line
    #[arg(long = "datetime-start")]
    pub datetime_start_pos: i32,

    /// Position of datetime's end on log line
    #[arg(long = "datetime-end")]
    pub datetime_end_pos: i32,

    /// Datetime String format eg: "%d/%m/%y %H:%M:%S"
    #[arg(long = "datetime-format")]
    pub datetime_str_format: String,

    /// Position of start of log level on log line
    #[arg(short, long = "loglevel-pos")]
    pub loglevel_position: i32,

    /// User can optionally give their name and it will be shown in the report
    #[arg(short, long = "user", required = false)]
    pub user: Option<String>,

    /// File patter to read log files eg: "./*.log"
    pub file_pattern: String,
}

pub fn validate_input(input: &Cli) -> Result<bool, &str> {
    if input.file_pattern.is_empty() {
        return Err("No file pattern given, give a valid file pattern");
    }

    if input.datetime_end_pos < input.datetime_start_pos {
        return Err("Datetime end position cannot be less than datetime start position");
    }

    Ok(true)
}
