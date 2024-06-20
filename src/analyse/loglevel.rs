// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum LogLevel {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
    Others,
}

pub fn to_loglevel(loglevel: &str) -> LogLevel {
    return match loglevel.to_uppercase().as_str() {
        "INFO" => LogLevel::Info,
        "INFORMATION" => LogLevel::Info,
        "DEBUG" => LogLevel::Debug,
        "WARNING" => LogLevel::Warning,
        "WARN" => LogLevel::Warning,
        "ERROR" => LogLevel::Error,
        "NOTICE" => LogLevel::Notice,
        "CRITICAL" => LogLevel::Critical,
        "ALERT" => LogLevel::Alert,
        "EMERGENCY" => LogLevel::Emergency,
        _ => LogLevel::Others,
    };
}

pub fn make_loglevel_count_map() -> HashMap<LogLevel, i32> {
    return HashMap::from([
        (LogLevel::Info, 0),
        (LogLevel::Debug, 0),
        (LogLevel::Warning, 0),
        (LogLevel::Error, 0),
        (LogLevel::Notice, 0),
        (LogLevel::Critical, 0),
        (LogLevel::Alert, 0),
        (LogLevel::Emergency, 0),
        (LogLevel::Others, 0),
    ]);
}

pub fn make_loglevel_count_vec_map() -> HashMap<LogLevel, Vec<i32>> {
    return HashMap::from([
        (LogLevel::Info, Vec::new()),
        (LogLevel::Debug, Vec::new()),
        (LogLevel::Warning, Vec::new()),
        (LogLevel::Error, Vec::new()),
        (LogLevel::Notice, Vec::new()),
        (LogLevel::Critical, Vec::new()),
        (LogLevel::Alert, Vec::new()),
        (LogLevel::Emergency, Vec::new()),
        (LogLevel::Others, Vec::new()),
    ]);
}
