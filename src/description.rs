use chrono::{Utc, NaiveTime, Datelike, Duration, NaiveDateTime, NaiveDate};
use serde::{ Serialize, Deserialize };

use std::{env, io};
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Interval {
    Weekly,
    Monthly,
    Yearly
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Description {
    pub label: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub tags: Vec<String>,
    pub start: NaiveDate,
    pub interval: Interval,
    pub interval_multiplier: i64
}

#[derive(Debug, PartialEq)]
pub struct RenderArgs {
    pub start_time: Option<NaiveTime>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub remaining_days: i64,
    pub bar_length: f64,
    pub bar_value: f64
}

impl Description {

    pub fn render_arguments(&self, _time: Option<NaiveDate>) -> RenderArgs {
        let time = _time.unwrap_or_else(|| Utc::now().date().naive_local());
        match self.interval {
            Interval::Weekly => {
                 let total_interval_days = self.interval_multiplier*7;
                 let total_days = (time - self.start).num_days().abs();
                 let remaining_days = total_interval_days - (total_days % total_interval_days);
                 let bar_length = (total_interval_days - 1) as f64;
                 let bar_value = (total_interval_days - remaining_days) as f64;
                 let start_date = self.start + Duration::days(total_interval_days*(total_days / total_interval_days));
                 let end_date = start_date + Duration::days(total_interval_days);
                 RenderArgs { start_time: None, start_date, end_date, remaining_days, bar_value, bar_length }
            },
            Interval::Monthly => {
                let mut new = self.clone();
                new.interval = Interval::Weekly;
                new.interval_multiplier = 4;
                new.render_arguments(_time)
            }
            Interval::Yearly => {
                let mut new = self.clone();
                new.interval = Interval::Weekly;
                new.interval_multiplier = 52;
                new.render_arguments(_time)
                // let target_month = self.start.month();
                // let target_day = self.start.day();
                // let total_days = (time - self.start).num_days() as i32;
                // let total_years = total_days as i32/ 365;
                // let start_date = NaiveDate::from_ymd(self.start.year() + total_years, target_month, target_day);
                // let days_since_start = total_days - total_years * 365;
                // let days_to_go = self.interval_multiplier*365 - days_since_start as i64;
                // RenderArgs {
                //     start_date,
                //     start_time: None,
                //     end_date: NaiveDate::from_ymd(start_date.year()+1, target_month, target_day),
                //     remaining_days: days_to_go as i64,
                //     bar_length: self.interval_multiplier as f64 * 365.,
                //     bar_value: days_since_start as f64
                // }
            }
        }
    }

}
