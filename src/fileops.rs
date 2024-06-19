// SPDX-License-Identifier: GPL-3.0-only

use glob::glob;
use std::fs::read_to_string;

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
