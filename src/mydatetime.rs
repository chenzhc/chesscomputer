#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::{Duration, Instant};

use chrono::{NaiveDate, NaiveTime};
use log::info;


pub fn test_std_time() {
    info!("test_std_time");

    let dur1 = Duration::from_secs(15);
    info!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(14500);
    info!("{}", dur2.as_millis());

    // let dur3 = dur1 - dur2;
    // info!("{}", dur3.as_millis());

    let dur4 = dur1.checked_sub(dur2);
    info!("{}", dur4.unwrap_or_default().as_millis());

    let now = Instant::now();
    // info!("{:?}", now);

    std::thread::sleep(Duration::from_millis(200));

    info!("Elapsed: {}", now.elapsed().as_micros());

}

pub fn test_chrono() {
    let now = chrono::Utc::now();
    info!("{:?}",now);

    let now = chrono::Local::now();
    info!("{:?}", now);

    info!("{}", now.format("%Y-%m-%d %H:%M:%S"));

    let date1 = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Fri);
    let unwrapped_date = date1.unwrap();
    let mut buffer = String::new();
    let df = unwrapped_date.format("Day of year is: %j");
    let _ = df.write_to(&mut buffer);

    info!("{:?}", buffer);

    unwrapped_date.iter_days()
        .take(4)
        .for_each(|d| info!("{}", d.format("%j")));

    let date2 = NaiveDate::from_yo_opt(2024, 366);
    info!("{}", date2.unwrap().format("%A %B %d"));

    let parse_dt = NaiveDate::parse_from_str("2025-08-30", "%Y-%m-%d");
    info!("{}", parse_dt.ok().unwrap().format("%Y-%m-%d"));

}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_chrono_test01() {
        crate::init();

        test_chrono();

    }

    #[test]
    fn it_test01() {
        crate::init();

        test_std_time();

    }
}
