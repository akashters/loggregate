// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use plotters::{
    backend::BitMapBackend,
    chart::{ChartBuilder, LabelAreaPosition},
    coord::ranged1d::{IntoSegmentedCoord, SegmentValue},
    drawing::IntoDrawingArea,
    series::Histogram,
    style::{
        full_palette::{
            BLUEGREY, GREY, LIGHTBLUE, LIGHTGREEN_A400, ORANGE_600, RED_400, RED_500, RED_600,
            RED_700,
        },
        Color, FontTransform, IntoFont, RGBColor, ShapeStyle, TextStyle, WHITE, YELLOW,
    },
};

use crate::analyse::{
    calc::{max_log_count, max_log_occ},
    dtfmt::{get_dt_fmt, DateTimeCat},
    loglevel::LogLevel,
    LogsAggregate,
};

pub fn plot_histograms(plot_gen_dir: &str, logs_aggregate: &LogsAggregate) {
    let num_dt = logs_aggregate.datetimes.len();

    for (loglevel, _) in &logs_aggregate.aggregates {
        let max_logs = max_log_occ(&logs_aggregate.aggregates, loglevel);

        let plot_file_path =
            format!("{}", plot_gen_dir) + "/" + &format!("{:?}", loglevel).to_lowercase() + ".png";
        let root = BitMapBackend::new(&plot_file_path, (1280, 720)).into_drawing_area();
        _ = root.fill(&WHITE);
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("{:?}", loglevel), ("sans-serif", 50).into_font())
            .margin(10)
            .margin_bottom(110)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d((0..num_dt - 1).into_segmented(), 0..max_logs)
            .unwrap();

        _ = chart
            .configure_mesh()
            .disable_x_mesh()
            .x_label_offset(5)
            .x_labels(num_dt)
            .set_tick_mark_size(
                LabelAreaPosition::Bottom,
                get_tick_size(&logs_aggregate.datetime_cat),
            )
            .x_label_formatter(&|x| match x {
                SegmentValue::CenterOf(v) => logs_aggregate.datetimes[*v as usize]
                    .clone()
                    .format(&get_dt_fmt(&logs_aggregate.datetime_cat))
                    .to_string(),
                _ => "UNK".to_string(),
            })
            .x_label_style(
                TextStyle::from(("sans-serif", 12).into_font()).transform(FontTransform::Rotate90),
            )
            .draw();

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .style(get_color_style(&loglevel))
                    .data((0..num_dt).map(|x| (x, logs_aggregate.aggregates[loglevel][x]))),
            )
            .unwrap()
            .label(format!("{:?}", loglevel));

        _ = root.present();
    }
}

pub fn plot_combined_bar_chart(
    plot_gen_dir: &str,
    combined_loglevel_count: &HashMap<LogLevel, i32>,
) {
    let num_log_levels = combined_loglevel_count.len();
    let loglevels: Vec<String> = combined_loglevel_count
        .keys()
        .map(|x| format!("{:?}", x))
        .collect();
    let counts: Vec<i32> = combined_loglevel_count
        .values()
        .map(|x| x.to_owned())
        .collect();
    let max_count = max_log_count(&combined_loglevel_count);
    let plot_file_path = format!("{}", plot_gen_dir) + "/combined.png";
    let root = BitMapBackend::new(&plot_file_path, (1280, 720)).into_drawing_area();
    _ = root.fill(&WHITE);
    let mut chart = ChartBuilder::on(&root)
        .caption("Combined Loglevel Count", ("sans-serif", 50).into_font())
        .margin(10)
        .margin_bottom(70)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((0..num_log_levels - 1).into_segmented(), 0..max_count)
        .unwrap();

    _ = chart
        .configure_mesh()
        .disable_x_mesh()
        .x_label_offset(5)
        .x_labels(num_log_levels)
        .set_tick_mark_size(LabelAreaPosition::Bottom, 20)
        .x_label_formatter(&|x| match x {
            SegmentValue::CenterOf(v) => loglevels[*v as usize].to_owned(),
            _ => "UNK".to_string(),
        })
        .x_label_style(TextStyle::from(("sans-serif", 24).into_font()))
        .draw();

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .data((0..num_log_levels).map(|x| (x, counts[x])))
                .style(BLUEGREY.filled()),
        )
        .unwrap()
        .label("Counts");

    _ = root.present();
}

fn get_tick_size(dt_cat: &DateTimeCat) -> i32 {
    return match dt_cat {
        DateTimeCat::Seconds => 40,
        DateTimeCat::Minutes => 35,
        DateTimeCat::Hours => 35,
        DateTimeCat::Days => 28,
        DateTimeCat::Months => 25,
        DateTimeCat::Years => 15,
    };
}

fn get_color_style(loglevel: &LogLevel) -> ShapeStyle {
    let loglevel_color_map: HashMap<&LogLevel, RGBColor> = HashMap::from([
        (&LogLevel::Emergency, RED_700),
        (&LogLevel::Alert, RED_600),
        (&LogLevel::Critical, RED_500),
        (&LogLevel::Error, RED_400),
        (&LogLevel::Warning, ORANGE_600),
        (&LogLevel::Notice, YELLOW),
        (&LogLevel::Info, LIGHTGREEN_A400),
        (&LogLevel::Debug, LIGHTBLUE),
        (&LogLevel::Others, GREY),
    ]);

    return loglevel_color_map[loglevel].filled();
}
