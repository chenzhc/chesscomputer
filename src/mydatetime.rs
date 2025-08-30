#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::{Duration, Instant};

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
