#![allow(dead_code, unused_variables)]

#[cfg(test)] 
mod tests {

    #[test]
    fn it_match_array_test() {
        let prices = [30000,50000,9000,120000];

        match prices[0..=3] {
            [30000,50000] => println!("You have some reasonably priced cars"),
            [30000,50000,..] => println!("You have a variety of cars!"),
            _ => println!("You don't have any reasonably priced cars"),
        }
    }

    #[test]
    fn it_match_string_test() {
        let car_manfacturer = "Porsche";

        match car_manfacturer {
            "Hyundai" => {
                println!("Hyundai it is!");
            },
            "Porsche" => {
                println!("Processing Porsche vehicle...");
            },
            _ => {
                println!("Manufacturer is not supported by this program.");
            }
        }
    }

    #[test]
    fn it_test() {
        println!("test");

        let myage: u16 = 5;

        match myage {
            0  => {
                println!("Your age is zero");
            },
            1..=35 => {
                println!("Your age is up to 35 ");
            },
            36..=134 => {
                println!("Age is between is 36 to 134");
            }
            135.. => {
                println!("Your age is over 135");
            }
            _ => {
                println!("Your age is not 35, Default: {}", myage);
            }
        }

    }
}