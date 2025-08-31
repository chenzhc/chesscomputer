#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{ops::AddAssign, sync::Mutex, thread};

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
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
        }
    };

    let myfunc2 = || {
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
        }
    };

    let _ = thread::scope(|scope| {
        scope.spawn(myfunc);
        scope.spawn(myfunc2);
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