#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;

use log::info;


pub fn test_threads() {
    let mut x = 0u128;

    for i in 1..500{
        x += i;
    }

    info!("Main thread finish a little bit of work x ");
    
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;

        for i in 1..50_000_000 {
            x += i;
        }
        info!("Main thread finish a little bit of work x");
    };

    info!("Starting new worker thread...");
    let handle = thread::spawn(thread_fn);
    let handle2 = thread::spawn(thread_fn);
    info!("Worker thread completed...");

    // test_threads();

    // let _ = handle.join();
    // let _ = handle2.join();

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            info!("All the worker are done, let's get out of here!");
            break;
        }
    }

}


#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_test_info_color() {
        crate::init();
        info!("Main thread finish a little bit of work ");
    }

    #[test]
    fn it_spwan_thread_test() {
        crate::init();

        spawn_thread();

    }

    #[test]
    fn it_test01() {
        crate::init();

        info!("Test");

        test_threads();

    }

}