

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


}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_test_if_test() {
        test_if();

    }

    #[test]
    fn it_test01() {
        println!("test01");

    }

}