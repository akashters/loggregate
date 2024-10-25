// SPDX-License-Identifier: GPL-3.0-only

use chrono::NaiveDateTime;
use dtfmt::dt_fmt_to_regex;
use loglevel::loglevel_regex_pattern;
use std::{collections::HashMap, collections::HashSet};

mod aggregate;
pub mod calc;
pub mod dtfmt;
pub mod loglevel;

use crate::analyse::aggregate::{
    aggregate_in_terms_of_days, aggregate_in_terms_of_hours, aggregate_in_terms_of_minutes,
    aggregate_in_terms_of_months, aggregate_in_terms_of_years,
};

use self::aggregate::aggregate_in_terms_of_seconds;
use self::dtfmt::DateTimeCat;
use self::loglevel::{to_loglevel, LogLevel};

#[derive(Copy, Clone)]
struct DateTimeLogLevelMap {
    datetime: NaiveDateTime,
    loglevel: LogLevel,
}

pub struct LogsAggregate {
    pub datetimes: Vec<NaiveDateTime>,
    pub datetime_cat: DateTimeCat,
    pub aggregates: HashMap<LogLevel, Vec<i32>>,
}

const MIN_SECONDS: i64 = 60;
const HOUR_SECONDS: i64 = MIN_SECONDS * 60;
const DAY_SECONDS: i64 = HOUR_SECONDS * 24;
const MONTH_SECONDS: i64 = DAY_SECONDS * 31;
const YEAR_SECONDS: i64 = MONTH_SECONDS * 12;

pub fn analyse_logs(log_lines: &Vec<String>, datetime_format: &str) -> LogsAggregate {
    let mut log_data: Vec<DateTimeLogLevelMap> = Vec::new();
    let mut unq_datetimes: HashSet<NaiveDateTime> = HashSet::new();
    let logs_aggregate: LogsAggregate;

    let dt_regex = dt_fmt_to_regex(datetime_format);
    let ll_regex = loglevel_regex_pattern();
    for log in log_lines {
        let datetime_str = match dt_regex.captures(&log) {
            Some(caps) => caps.get(0).unwrap().as_str().to_owned(),
            _ => {
                eprintln!("Error finding date with given format in line: {}", log);
                continue;
            }
        };

        let datetime = match NaiveDateTime::parse_from_str(&datetime_str, datetime_format) {
            Ok(dt) => dt,
            Err(_) => {
                eprintln!("Error parsing date in line: {}", log);
                continue;
            }
        };

        let loglevel_str = match ll_regex.captures(&log) {
            Some(caps) => caps.get(0).unwrap().as_str().to_owned(),
            _ => "others".to_owned(),
        };

        let loglevel = to_loglevel(&loglevel_str);

        unq_datetimes.insert(datetime);
        log_data.push(DateTimeLogLevelMap { datetime, loglevel });
    }

    let mindt = unq_datetimes.iter().min().unwrap();
    let maxdt = unq_datetimes.iter().max().unwrap();

    let logs_duration = maxdt.signed_duration_since(mindt.to_owned());

    if logs_duration.num_seconds() > YEAR_SECONDS {
        println!("Analysing in terms of years...");
        let duration_years =
            u64::div_ceil(logs_duration.num_seconds() as u64, YEAR_SECONDS as u64) as i64;

        logs_aggregate = aggregate_in_terms_of_years(&log_data, mindt, maxdt, duration_years);
    } else if logs_duration.num_seconds() > MONTH_SECONDS {
        println!("Analysing in terms of months...");
        let duration_months =
            u64::div_ceil(logs_duration.num_seconds() as u64, MONTH_SECONDS as u64) as i64;

        logs_aggregate = aggregate_in_terms_of_months(&log_data, mindt, maxdt, duration_months);
    } else if logs_duration.num_seconds() > DAY_SECONDS {
        println!("Analysing in terms of days...");
        let duration_days =
            u64::div_ceil(logs_duration.num_seconds() as u64, DAY_SECONDS as u64) as i64;

        logs_aggregate = aggregate_in_terms_of_days(&log_data, mindt, maxdt, duration_days);
    } else if logs_duration.num_seconds() > HOUR_SECONDS {
        println!("Analysing in terms of hours...");
        let duration_hours =
            u64::div_ceil(logs_duration.num_seconds() as u64, HOUR_SECONDS as u64) as i64;

        logs_aggregate = aggregate_in_terms_of_hours(&log_data, mindt, maxdt, duration_hours);
    } else if logs_duration.num_seconds() > MIN_SECONDS {
        println!("Analysing in terms of minutes...");
        let duration_mins =
            u64::div_ceil(logs_duration.num_seconds() as u64, MIN_SECONDS as u64) as i64;

        logs_aggregate = aggregate_in_terms_of_minutes(&log_data, mindt, maxdt, duration_mins);
    } else {
        println!("Analysing in terms of seconds...");
        let duration_secs: i64;
        if logs_duration.subsec_nanos() > 0 {
            duration_secs = logs_duration.num_seconds() + 1;
        } else {
            duration_secs = logs_duration.num_seconds();
        }

        logs_aggregate = aggregate_in_terms_of_seconds(&log_data, mindt, maxdt, duration_secs);
    }

    return logs_aggregate;
}
