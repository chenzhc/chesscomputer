#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;

use log::info;

struct Person {
    first_name: String,

}

pub fn test_thread_variables() {
    info!("test");

    let person01 = Person { first_name: String::from("Trevor")};


    let age = 34;

    let print_age =  || {
        info!("This is the child closure");
        info!("Your age is {age}");
        info!("Person name is {}", person01.first_name);
    };

    info!("{}", age);
    let _t1 = 30;
    info!("{}", _t1);


    // let _ = thread::spawn(print_age).join();
    let _ = thread::scope(|scope | {
        scope.spawn(print_age);
        scope.spawn(print_age);
        scope.spawn(print_age);
    });


    info!("Give control back to Main");
    info!("Your age is {age}");
    info!("Your name is: {}", person01.first_name);
    
}


#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_test_info_color() {
        crate::init();
        test_thread_variables();

    }

}