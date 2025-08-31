#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod closures;

pub mod generices;

pub mod match_test;

pub mod optiontest;

pub mod mystruct;

pub mod traits_test;

pub mod myverc;

pub mod myhashmap;

pub mod myiters;

pub mod mydatetime;

pub mod mydatafusion;

pub mod mythreads;

// init log config 
pub fn init() {
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init();
}

#[allow(dead_code)]
fn test_if() {
    let age_to_derive: u8 = 16u8;
    println!("{0}", age_to_derive);

    println!("Enter the person's age: ");

    let  my_input = &mut String::from("");

    std::io::stdin()
        .read_line(my_input)
        .unwrap();

    let age = my_input.trim().parse::<u8>().unwrap();

    if age > age_to_derive {
        println!("Issuing driver's license, because they are old enough");
    } else if age == 16 || age > 14 {
        println!("You are just on the verage of being old enough! Wiat one more year.");

    } else {
        println!("Wait a bit longer, you aren't old enough for a driver's license!");
    }

    let drivers_license: bool = if age >= 16 { true } else { false };
    println!("{0}", drivers_license);


}

fn test_while() {
    let age_to_drive = 16u8;
    let mut current_age = 0u8;

    while current_age < age_to_drive {
        println!("Waiting...{0}", current_age);

        current_age+=1;

        if current_age == 6 {
            break;
        }

    }
}

fn test_loop() {
    let mut x = 1;
    loop {
        println!("Hello from Rust! ");

        if x > 5 {
            break;
        }

        x += 1;

    }
}

fn test_for() {
    let ages = [14, 18, 26, 35,41];

    let age_to_drive = 16i32;

     for value in ages {
        println!("The current age is: {0}", value);
        if value >= age_to_drive {
            println!("You are old enough to drive!");
        } else {
            println!("You need to wait a little bit more...");
        }
     }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_test_for_test() {
        test_for();

    }

    #[test]
    fn it_test_loop_test() {
        test_loop();

    }

    #[test]
    fn it_test_while_test() {
        test_while();

    }

    #[test]
    fn it_test_if_test() {
        test_if();

    }

    #[test]
    fn it_test01() {
        println!("test01");

    }

}