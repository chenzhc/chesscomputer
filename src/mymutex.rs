#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{ops::AddAssign, sync::Mutex, thread, time::Duration};

use log::info;

pub fn test_mutex() {
    let mut score = Mutex::new(0u16);

    let unlocked_data = score.lock();

    let mut data = unlocked_data.unwrap();
    data.add_assign(5);

    info!("{:?}", data);
    drop(data);

}

pub fn test_mutex2() {
    let mut score = Mutex::new(0u16);

    let myfunc = || {
        info!("Thread 1 is waiting for mutex lock...");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            info!("Thread 1 is adding {i}");
        }
    };

    let myfunc2 = || {
        loop {
            info!("Thread 2 is waiting for mutex lock...");
            let guard = score.try_lock();

            if guard.is_ok() {

                let mut data = guard.unwrap();

                for i in 1..10 {
                    data.add_assign(i);
                    info!("Thread 2 is adding {i}");
                }
                drop(data);

            }

            thread::sleep(Duration::from_millis(300));
        }
        
        panic!("Error in thred 2");

        
    };

    let myfunc3 = || {
        info!("Thread 3 is waiting for mutex lock...");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            info!("Thread 3 is adding {i}");
        }
    };

    let _ = thread::scope(|scope| {
        let handle1 = scope.spawn(myfunc).join();
        let handle2 = scope.spawn(myfunc2).join();
        let handle3 = scope.spawn(myfunc3).join();

        if handle2.is_err() {
            info!("Thread 2 had a error, let's handle it here");
        }
    });

    info!("{:?}", score.lock().unwrap());
}


#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use log::info;

    use super::*;

    #[test]
    fn it_mutex2_test() {
        crate::init();

        test_mutex2();

    }

    #[test]
    fn it_mutex_test() {
        crate::init();

        test_mutex();

    }

    #[test]
    fn it_test_info_color() {
        crate::init();

        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        info!("m = {:?}", m);
        info!("Test");

    }

}