// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(about = "Aggregates, Analyses and Generates reports from log files", long_about = None)]
pub struct Cli {
    /// Datetime String format eg: "%d/%m/%y %H:%M:%S"
    #[arg(short, long = "datetime-format")]
    pub datetime_str_format: String,

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

    if input.datetime_str_format.is_empty() {
        return Err("Empty datetime string format given, give a valid datetime format");
    }

    Ok(true)
}
