

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
    
}

#[cfg(test)]
mod tests {

    use super::*;

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