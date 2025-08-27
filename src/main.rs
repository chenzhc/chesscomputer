#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
use fast_log::Config;
use log::{error, info, warn};

pub mod helpers; 

fn init() {
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init();
}

fn main() {
    init();
    fast_log::init(Config::new().console().chan_len(Some(100000))).unwrap();
    println!("Hello, world!");
    // test_func();
    let rs_name = helpers::namehelpers::get_full_name("test1", "test2");
    println!("{0}", rs_name);

    let rs_age = helpers::private_fns::get_age_plus_5(10);
    println!("The new age is {0}", rs_age);

}


#[allow(dead_code, unused_assignments)]
fn test_func() {
    let x: f32 = 255.0;
    let y = x as u8 -5;
    println!("{:?}", y);

    let iamold = true;
    // iamold = false;

    println!("{}", iamold);

    let mystr = 'A';
    println!("{0}", mystr);


    let mut first_name = "test";
    first_name = "Bob";
    println!("{0}", first_name);


    let name = ("Trevor", "Sullivan", 40 as u8);
    println!("{:?}", name);

    let ages: [u16; 6] = [40, 50, 50, 55, 60, 65];
    println!("{:?}", ages);

    let new_ages = &ages[1..5];
    println!("{:?}", new_ages);
    

}
