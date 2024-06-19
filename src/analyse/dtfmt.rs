// SPDX-License-Identifier: GPL-3.0-only

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
