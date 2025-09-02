#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{sync::mpsc, time::Duration};

use log::info;



#[allow(dead_code, unused_variables)]
pub fn test_channels() {

    let (tx, rx) = mpsc::channel::<u8>();
    // drop(rx);
    
    let send_result = tx.send(100);
    info!("Send status: {:?}", send_result.is_ok());

    tx.send(150);
    
    let receive_result = rx.recv_timeout(Duration::from_millis(300));
    info!("Receive result is: {}", receive_result.is_ok());
    info!("Receive result data is: {}", receive_result.unwrap());

    let receive_result = rx.recv_timeout(Duration::from_millis(300));
    info!("Receive result is: {}", receive_result.is_ok());
    info!("Receive result data is: {}", receive_result.unwrap());

}


#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use log::info;

    use super::*;

    #[test]
    fn it_mutex2_test() {
        crate::init();
        info!("test");
        test_channels();

    }

}