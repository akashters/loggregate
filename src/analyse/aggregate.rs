// SPDX-License-Identifier: GPL-3.0-only

use chrono::{Datelike, Months, NaiveDateTime, NaiveTime, TimeDelta, Timelike};
use std::collections::HashMap;

use crate::analyse::loglevel::{make_loglevel_count_map, make_loglevel_count_vec_map, LogLevel};
use crate::analyse::{DateTimeLogLevelMap, LogsAggregate};

use super::dtfmt::DateTimeCat;

pub fn aggregate_in_terms_of_seconds(
    log_data: &Vec<DateTimeLogLevelMap>,
    start_datetime: &NaiveDateTime,
    end_datetime: &NaiveDateTime,
    duration_secs: i64,
) -> LogsAggregate {
    let cur_datetime = start_datetime.to_owned();
    let mut datetime_secs: Vec<NaiveDateTime> = Vec::new();

    datetime_secs.push(cur_datetime.with_nanosecond(0).unwrap());
    for i in 1..=duration_secs + 1 {
        let inc_datetime = datetime_secs[0]
            .checked_add_signed(TimeDelta::try_seconds(i).unwrap())
            .unwrap();
        datetime_secs.push(inc_datetime);
    }

    let analyzed_data = aggregate_logs(&datetime_secs, log_data);

    let (filtered_datetimes, filtered_data) =
        clean_analyzed_data(&datetime_secs, &analyzed_data, end_datetime);

    return LogsAggregate {
        datetimes: filtered_datetimes,
        datetime_cat: DateTimeCat::Seconds,
        aggregates: filtered_data,
    };
}

pub fn aggregate_in_terms_of_minutes(
    log_data: &Vec<DateTimeLogLevelMap>,
    start_datetime: &NaiveDateTime,
    end_datetime: &NaiveDateTime,
    duration_mins: i64,
) -> LogsAggregate {
    let mut datetime_mins: Vec<NaiveDateTime> = Vec::new();

    datetime_mins.push(
        NaiveDateTime::with_second(start_datetime, 0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap(),
    );
    for i in 1..=duration_mins + 1 {
        let inc_datetime = datetime_mins[0]
            .checked_add_signed(TimeDelta::try_minutes(i).unwrap())
            .unwrap();
        datetime_mins.push(inc_datetime);
    }

    let analyzed_data = aggregate_logs(&datetime_mins, log_data);

    let (filtered_datetimes, filtered_data) =
        clean_analyzed_data(&datetime_mins, &analyzed_data, end_datetime);

    return LogsAggregate {
        datetimes: filtered_datetimes,
        datetime_cat: DateTimeCat::Minutes,
        aggregates: filtered_data,
    };
}

pub fn aggregate_in_terms_of_hours(
    log_data: &Vec<DateTimeLogLevelMap>,
    start_datetime: &NaiveDateTime,
    end_datetime: &NaiveDateTime,
    duration_hours: i64,
) -> LogsAggregate {
    let mut datetime_hrs: Vec<NaiveDateTime> = Vec::new();

    datetime_hrs.push(
        NaiveDateTime::with_minute(start_datetime, 0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap(),
    );
    for i in 1..=duration_hours + 1 {
        let inc_datetime = datetime_hrs[0]
            .checked_add_signed(TimeDelta::try_hours(i).unwrap())
            .unwrap();
        datetime_hrs.push(inc_datetime);
    }

    let analyzed_data = aggregate_logs(&datetime_hrs, log_data);

    let (filtered_datetimes, filtered_data) =
        clean_analyzed_data(&datetime_hrs, &analyzed_data, end_datetime);

    return LogsAggregate {
        datetimes: filtered_datetimes,
        datetime_cat: DateTimeCat::Hours,
        aggregates: filtered_data,
    };
}

pub fn aggregate_in_terms_of_days(
    log_data: &Vec<DateTimeLogLevelMap>,
    start_datetime: &NaiveDateTime,
    end_datetime: &NaiveDateTime,
    duration_days: i64,
) -> LogsAggregate {
    let mut datetime_days: Vec<NaiveDateTime> = Vec::new();

    datetime_days.push(
        start_datetime
            .date()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
    );
    for i in 1..=duration_days + 1 {
        let inc_datetime = datetime_days[0]
            .checked_add_signed(TimeDelta::try_days(i).unwrap())
            .unwrap();
        datetime_days.push(inc_datetime);
    }
    let analyzed_data = aggregate_logs(&datetime_days, log_data);

    let (filtered_datetimes, filtered_data) =
        clean_analyzed_data(&datetime_days, &analyzed_data, end_datetime);

    return LogsAggregate {
        datetimes: filtered_datetimes,
        datetime_cat: DateTimeCat::Days,
        aggregates: filtered_data,
    };
}

pub fn aggregate_in_terms_of_months(
    log_data: &Vec<DateTimeLogLevelMap>,
    start_datetime: &NaiveDateTime,
    end_datetime: &NaiveDateTime,
    duration_months: i64,
) -> LogsAggregate {
    let mut datetime_mnths: Vec<NaiveDateTime> = Vec::new();

    datetime_mnths.push(
        start_datetime
            .date()
            .with_day(1)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
    );
    for i in 1..=duration_months + 1 {
        let inc_datetime = datetime_mnths[0]
            .checked_add_months(Months::new(i as u32))
            .unwrap();
        datetime_mnths.push(inc_datetime);
    }
    let analyzed_data = aggregate_logs(&datetime_mnths, log_data);

    let (filtered_datetimes, filtered_data) =
        clean_analyzed_data(&datetime_mnths, &analyzed_data, end_datetime);

    return LogsAggregate {
        datetimes: filtered_datetimes,
        datetime_cat: DateTimeCat::Months,
        aggregates: filtered_data,
    };
}

pub fn aggregate_in_terms_of_years(
    log_data: &Vec<DateTimeLogLevelMap>,
    start_datetime: &NaiveDateTime,
    end_datetime: &NaiveDateTime,
    duration_years: i64,
) -> LogsAggregate {
    let mut datetime_yrs: Vec<NaiveDateTime> = Vec::new();

    datetime_yrs.push(
        start_datetime
            .date()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
    );
    for i in 1..=duration_years + 1 {
        let inc_datetime = datetime_yrs[0]
            .checked_add_months(Months::new(i as u32 * 12))
            .unwrap();
        datetime_yrs.push(inc_datetime);
    }
    let analyzed_data = aggregate_logs(&datetime_yrs, log_data);

    let (filtered_datetimes, filtered_data) =
        clean_analyzed_data(&datetime_yrs, &analyzed_data, end_datetime);

    return LogsAggregate {
        datetimes: filtered_datetimes,
        datetime_cat: DateTimeCat::Years,
        aggregates: filtered_data,
    };
}

fn aggregate_logs(
    datetimes: &Vec<NaiveDateTime>,
    log_data: &Vec<DateTimeLogLevelMap>,
) -> HashMap<LogLevel, Vec<i32>> {
    let mut analyzed_data: HashMap<LogLevel, Vec<i32>> = make_loglevel_count_vec_map();

    for i in 0..datetimes.len() - 1 {
        let datetime = datetimes[i];
        let next_datetime = datetimes[i + 1];
        let counts = count_log_levels(log_data, &datetime, &next_datetime);
        for (loglevel, data) in analyzed_data.to_owned() {
            let mut updated_data = data.to_vec();
            updated_data.push(counts[&loglevel]);
            analyzed_data.insert(loglevel, updated_data);
        }
    }

    return analyzed_data;
}

fn count_log_levels(
    log_data: &Vec<DateTimeLogLevelMap>,
    from_dt: &NaiveDateTime,
    to_dt: &NaiveDateTime,
) -> HashMap<LogLevel, i32> {
    let mut loglevel_counts: HashMap<LogLevel, i32> = make_loglevel_count_map();

    for log in log_data {
        if log.datetime >= from_dt.to_owned() && log.datetime < to_dt.to_owned() {
            for (loglevel, count) in loglevel_counts.to_owned() {
                if log.loglevel == loglevel {
                    loglevel_counts.insert(loglevel, count + 1);
                    break;
                }
            }
        }
    }

    return loglevel_counts;
}

fn clean_analyzed_data(
    datetimes: &Vec<NaiveDateTime>,
    analyzed_data: &HashMap<LogLevel, Vec<i32>>,
    end_datetime: &NaiveDateTime,
) -> (Vec<NaiveDateTime>, HashMap<LogLevel, Vec<i32>>) {
    let filtered_datetimes: Vec<NaiveDateTime> = datetimes
        .iter()
        .filter(|&dt| dt <= end_datetime)
        .cloned()
        .collect();
    let total_num = filtered_datetimes.len();
    let mut loglevel_counts = make_loglevel_count_vec_map();
    for (loglevel, counts) in analyzed_data {
        let filtered_counts = counts[..total_num].to_vec();
        loglevel_counts.insert(loglevel.to_owned(), filtered_counts);
    }

    return (filtered_datetimes, loglevel_counts);
}
