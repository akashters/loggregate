// SPDX-License-Identifier: GPL-3.0-only

use fs_extra::dir::{copy, CopyOptions};
use glob::glob;
use std::{
    env::{current_dir, temp_dir},
    fs::{create_dir_all, read_to_string, remove_dir_all},
    path::Path,
};

pub fn read_logs(file_pattern: &str, log_lines: &mut Vec<String>) -> i32 {
    let mut file_paths: Vec<String> = Vec::new();
    for entry in glob(file_pattern).expect("Failed to read file pattern") {
        match entry {
            Ok(path) => file_paths.push(path.to_str().unwrap().to_owned()),
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }

    for path in &file_paths {
        for line in read_to_string(path).unwrap().lines() {
            log_lines.push(line.to_owned());
        }
    }

    return file_paths.len() as i32;
}

pub fn prepare_tmp_loggregate_dir() -> String {
    let tmp_dir = temp_dir().to_str().unwrap().to_owned();

    let tmp_logrregate_dir = tmp_dir + "/loggregate";
    if Path::new(&tmp_logrregate_dir).is_dir() {
        remove_dir_all(&tmp_logrregate_dir).expect("Error while cleaning tmp directory");
    }

    return tmp_logrregate_dir;
}

pub fn prepare_plots_gen_dir(tmp_loggregate_dir: &str) -> String {
    let plots_gen_dir = format!("{}/{}", tmp_loggregate_dir, "plots");
    create_dir_all(&plots_gen_dir).expect("Error while creating plots directory");
    return plots_gen_dir;
}

pub fn prepare_report_destination_dir() -> String {
    let cur_dir = current_dir().unwrap().to_str().unwrap().to_owned();
    let report_dest_dir = format!("{}/{}", cur_dir, "report");
    if Path::new(&report_dest_dir).is_dir() {
        remove_dir_all(&report_dest_dir).expect("Error while cleaning up report directory");
    }
    create_dir_all(&report_dest_dir).expect("Error while creating report directory");
    return report_dest_dir;
}

pub fn copy_reports_to_destination(tmp_loggregate_dir: &str, report_destination_dir: &str) {
    let options = CopyOptions::new().content_only(true);
    copy(tmp_loggregate_dir, report_destination_dir, &options)
        .expect("Error while copying report to destination");
}
