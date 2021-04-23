#![allow(non_snake_case)]
use Cycles::{Description, Interval, RenderArgs};
use chrono::NaiveDate;

fn monday() -> Description {
    Description {
        tags: vec![],
        label: "Every Monday".into(),
        description: None,
        color: None,
        start: NaiveDate::from_ymd(2021, 04, 4),
        interval: Interval::Weekly,
        interval_multiplier: 1
    }
}
#[test]
fn render_small() {
    let monday = monday();
    let datetime = NaiveDate::from_ymd(2021, 04, 5);
    assert_eq!(RenderArgs {
        start_date: NaiveDate::from_ymd(2021, 04, 4),
        end_date: NaiveDate::from_ymd(2021, 04, 11),
        remaining_days: 6,
        bar_length: 6.0,
        bar_value: 1.
    }, monday.render_arguments(Some(datetime)));

    let datetime = NaiveDate::from_ymd(2021, 04, 6);
    assert_eq!(RenderArgs {
        start_date: NaiveDate::from_ymd(2021, 04, 4),
        end_date: NaiveDate::from_ymd(2021, 04, 11),
        remaining_days: 5,
        bar_length: 6.0,
        bar_value: 2.
    }, monday.render_arguments(Some(datetime)));
}

#[test]
fn render_large() {
    let monday = monday();
    let datetime = NaiveDate::from_ymd(2021, 04, 13);
    assert_eq!(RenderArgs {
        start_date: NaiveDate::from_ymd(2021, 04, 11),
        end_date: NaiveDate::from_ymd(2021, 04, 18),
        remaining_days: 5,
        bar_length: 6.0,
        bar_value: 2.
    }, monday.render_arguments(Some(datetime)));
}
