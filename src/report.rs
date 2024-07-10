// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, fs::File, io::Write};

use crate::analyse::loglevel::LogLevel;

pub fn generate_html_report(
    report_dir: &str,
    no_of_files: i32,
    no_of_logs: i32,
    combined_loglevel_count: &HashMap<LogLevel, i32>,
    user: Option<String>,
) {
    let mut html_text: String = include_str!("../assets/index.html").to_owned();
    let html_report_path = format!("{}", report_dir) + "/report.html";

    html_text = html_text.replace("{#NO_OF_FILES#}", &format!("{}", no_of_files));
    let value_map = prepare_placeholder_map(no_of_files, no_of_logs, combined_loglevel_count, user);

    html_text = replace_placeholders(html_text, &value_map);
    let mut file = File::create(html_report_path).unwrap();
    file.write_all(html_text.as_bytes()).unwrap();
}

fn prepare_placeholder_map(
    no_of_files: i32,
    no_of_logs: i32,
    combined_loglevel_count: &HashMap<LogLevel, i32>,
    user: Option<String>,
) -> HashMap<&str, String> {
    return HashMap::from([
        ("NO_OF_FILES", format!("{}", no_of_files)),
        ("NO_OF_LOGS", format!("{}", no_of_logs)),
        (
            "NO_OF_ALERT",
            format!("{}", combined_loglevel_count[&LogLevel::Alert]),
        ),
        (
            "NO_OF_CRITICAL",
            format!("{}", combined_loglevel_count[&LogLevel::Critical]),
        ),
        (
            "NO_OF_EMERGENCY",
            format!("{}", combined_loglevel_count[&LogLevel::Emergency]),
        ),
        (
            "NO_OF_ERROR",
            format!("{}", combined_loglevel_count[&LogLevel::Error]),
        ),
        (
            "NO_OF_WARNING",
            format!("{}", combined_loglevel_count[&LogLevel::Warning]),
        ),
        (
            "NO_OF_NOTICE",
            format!("{}", combined_loglevel_count[&LogLevel::Notice]),
        ),
        (
            "NO_OF_INFO",
            format!("{}", combined_loglevel_count[&LogLevel::Info]),
        ),
        (
            "NO_OF_DEBUG",
            format!("{}", combined_loglevel_count[&LogLevel::Debug]),
        ),
        (
            "NO_OF_OTHERS",
            format!("{}", combined_loglevel_count[&LogLevel::Others]),
        ),
        (
            "ALERT_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Alert] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "CRITICAL_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Critical] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "EMERGENCY_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Emergency] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "ERROR_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Error] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "WARNING_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Warning] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "NOTICE_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Notice] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "INFO_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Info] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "DEBUG_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Debug] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "OTHERS_VISIBILITY",
            if combined_loglevel_count[&LogLevel::Others] <= 0 {
                "hide".to_owned()
            } else {
                "".to_owned()
            },
        ),
        (
            "BY_USER",
            match user {
                None => "".to_owned(),
                Some(user) => match user.as_str() {
                    "" => "".to_owned(),
                    _ => format!(" by {}", user),
                },
            },
        ),
    ]);
}

fn replace_placeholders(text: String, value_map: &HashMap<&str, String>) -> String {
    let mut replaced_text = text;
    for (&from, to) in value_map {
        replaced_text = replaced_text.replace(format!("{{#{}#}}", from).as_str(), to);
    }

    return replaced_text;
}
