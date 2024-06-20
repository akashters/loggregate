// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;
use core::panic;
use fileops::{
    copy_reports_to_destination, prepare_plots_gen_dir, prepare_report_destination_dir,
    prepare_tmp_loggregate_dir,
};
use plot::plot_combined_bar_chart;
use report::generate_html_report;

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

    println!("Reading the log files...");
    let no_of_files = read_logs(&input.file_pattern, &mut log_lines);
    let no_of_logs = log_lines.len() as i32;
    println!("Completed reading the files");

    let logs_aggregate = analyse_logs(
        &log_lines,
        input.datetime_start_pos,
        input.datetime_end_pos,
        &input.datetime_str_format,
        input.loglevel_position,
    );

    let tmp_loggregate_dir = prepare_tmp_loggregate_dir();
    let plots_gen_dir = prepare_plots_gen_dir(&tmp_loggregate_dir);

    println!("Preparing the plots...");
    plot_histograms(&plots_gen_dir, &logs_aggregate);

    let combined_loglevel_count = sum_of_log_occ(&logs_aggregate.aggregates);
    plot_combined_bar_chart(&plots_gen_dir, &combined_loglevel_count);

    println!("Preparing the report...");
    generate_html_report(
        &tmp_loggregate_dir,
        no_of_files,
        no_of_logs,
        &combined_loglevel_count,
        input.user,
    );

    println!("Copying report to the current directory...");
    let report_destination_dir = prepare_report_destination_dir();
    copy_reports_to_destination(&tmp_loggregate_dir, &report_destination_dir);

    println!("Report is now available in 'report' directory");
    Ok(())
}
