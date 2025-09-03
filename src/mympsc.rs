#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{sync::mpsc, thread, time::Duration};

use log::info;



#[allow(dead_code, unused_variables)]
pub fn test_channels_02() {

    let (tx, rx) = mpsc::channel::<u8>();
    // drop(rx);
    
    // let send_result = tx.send(100);
    // info!("Send status: {:?}", send_result.is_ok());

    // tx.send(150);
    
    // let receive_result = rx.recv_timeout(Duration::from_millis(300));
    // info!("Receive result is: {}", receive_result.is_ok());
    // info!("Receive result data is: {}", receive_result.unwrap());

    // let receive_result = rx.recv_timeout(Duration::from_millis(300));
    // info!("Receive result is: {}", receive_result.is_ok());
    // info!("Receive result data is: {}", receive_result.unwrap());

    let processor_code = move || {
        info!("Starting processor thread ...");
        loop {
            info!("Attaempting to receive message from channel ...");
            let recive_result = rx.recv_timeout(Duration::from_millis(800));

            if recive_result.is_ok() {
                info!("Received message: {}", recive_result.unwrap());
                // break;
            }

            thread::sleep(Duration::from_millis(200));
        }
    };

    // for x in 1..=6 {
    //     let send_result = tx.send(x);
    //     info!("Send status: {}", send_result.is_ok());
    //     thread::sleep(Duration::from_millis(200));
    // }


    let _ = thread::spawn(processor_code).join();

}


#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_test02() {
        crate::init();

        info!("test02");

        test_channels_02();

    }

    #[test]
    fn it_mutex2_test() {
        crate::init();
        test_channels_02();

    }

}