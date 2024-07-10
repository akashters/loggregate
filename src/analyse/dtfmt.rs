// SPDX-License-Identifier: GPL-3.0-only

use regex::Regex;

#[derive(Debug)]
pub enum DateTimeCat {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

const DATETIME_SECS_FMT: &str = "%Y-%m-%d %H:%M:%S";
const DATETIME_MIN_FMT: &str = "%Y-%m-%d %H:%M";
const DATETIME_HR_FMT: &str = "%Y-%m-%d %H:%M";
const DATETIME_DAY_FMT: &str = "%Y-%m-%d";
const DATETIME_MTH_FMT: &str = "%Y-%b";
const DATETIME_YR_FMT: &str = "%Y";

pub fn get_dt_fmt(dt_cat: &DateTimeCat) -> String {
    return match dt_cat {
        DateTimeCat::Seconds => String::from(DATETIME_SECS_FMT),
        DateTimeCat::Minutes => String::from(DATETIME_MIN_FMT),
        DateTimeCat::Hours => String::from(DATETIME_HR_FMT),
        DateTimeCat::Days => String::from(DATETIME_DAY_FMT),
        DateTimeCat::Months => String::from(DATETIME_MTH_FMT),
        DateTimeCat::Years => String::from(DATETIME_YR_FMT),
    };
}

pub fn dt_fmt_to_regex(dt_str: &str) -> Regex {
    // Supported specs
    // %Y %y %m %b %B %h %d %e %F %H %k %I %l %P %p %M %S

    let mut regex_pattern = String::from(dt_str);
    regex_pattern = regex_pattern
        .replace("%Y", r"\d{4}")
        .replace("%y", r"\d{2}")
        .replace("%m", r"\d{2}")
        .replace(
            "%b",
            r"(?i)(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)",
        )
        .replace(
                 "%h",
                 r"(?i)(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)",
                 )
        .replace(
                 "%B",
                 r"(?i)(january|february|march|april|may|june|july|august|september|october|november|december)"
                 )
        .replace("%d", r"\d{2}")
        .replace("%e", r"( |[1-9])[0-9]")
        .replace("%F", r"\d{4}-\d{2}-\d{2}")
        .replace("%H", r"\d{2}")
        .replace("%k", r"( |[1-9])[0-9]")
        .replace("%I", r"\d{2}")
        .replace("%l", r"( |[1-9])[0-9]")
        .replace("%P", r"(am|pm)")
        .replace("%p", r"(AM|PM)")
        .replace("%M", r"\d{2}")
        .replace("%S", r"\d{2}");

    let re = Regex::new(&regex_pattern).unwrap();

    return re;
}
