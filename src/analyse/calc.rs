// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::analyse::loglevel::LogLevel;

pub fn max_log_occ(loglevel_count: &HashMap<LogLevel, Vec<i32>>, loglevel: &LogLevel) -> i32 {
    let log_counts = &loglevel_count[loglevel];
    let max_logs_count = log_counts.iter().max().unwrap().to_owned();
    return max_logs_count;
}

pub fn sum_of_log_occ(loglevel_count: &HashMap<LogLevel, Vec<i32>>) -> HashMap<LogLevel, i32> {
    let combined_log_occ: HashMap<LogLevel, i32> = loglevel_count
        .iter()
        .map(|(loglevel, counts)| {
            return (loglevel.to_owned(), counts.iter().sum());
        })
        .collect();

    return combined_log_occ;
}

pub fn max_log_count(loglevel_count: &HashMap<LogLevel, i32>) -> i32 {
    return loglevel_count
        .iter()
        .map(|(_, count)| {
            return count;
        })
        .max()
        .unwrap()
        .to_owned();
}
