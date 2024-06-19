// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;
use core::panic;
use plot::plot_combined_bar_chart;
use report::generate_html_report;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;

mod analyse;
mod cli;
mod fileops;
mod plot;
mod report;

use crate::analyse::analyse_logs;
use crate::analyse::calc::sum_of_log_occ;
use crate::cli::{validate_input, Cli};
use crate::fileops::read_logs;
use crate::plot::plot_histograms;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Cli::parse();
    let mut log_lines: Vec<String> = Vec::new();

    match validate_input(&input) {
        Ok(_) => {}
        Err(err) => {
            panic!("{:?}", err);
        }
    };

    let no_of_files = read_logs(&input.file_pattern, &mut log_lines);
    let no_of_logs = log_lines.len() as i32;

    let logs_aggregate = analyse_logs(
        &log_lines,
        input.datetime_start_pos,
        input.datetime_end_pos,
        &input.datetime_str_format,
        input.loglevel_position,
    );

    let tmp_dir = std::env::temp_dir().to_str().unwrap().to_owned();

    let tmp_logrregate_dir = tmp_dir + "/loggregate";
    if Path::new(&tmp_logrregate_dir).is_dir() {
        remove_dir_all(&tmp_logrregate_dir)?;
    }
    let plots_gen_dir = format!("{}/{}", tmp_logrregate_dir, "plots");
    create_dir_all(&plots_gen_dir)?;

    plot_histograms(&plots_gen_dir, &logs_aggregate);

    let combined_loglevel_count = sum_of_log_occ(&logs_aggregate.aggregates);
    plot_combined_bar_chart(&plots_gen_dir, &combined_loglevel_count);

    generate_html_report(
        &tmp_logrregate_dir,
        no_of_files,
        no_of_logs,
        &combined_loglevel_count,
        input.user,
    );

    Ok(())
}
