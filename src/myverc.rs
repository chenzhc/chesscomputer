#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use log::info;

#[derive(Debug,Clone)]
pub struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_car() {
    let car_list  = vec![ Car { manufacturer: "Porsche".to_string(), model: "Panamera".to_string() }; 10];

    info!("{:?}", car_list);

    let mut car_list2: Vec<Car> = vec![];
    let mut car_lot2: Vec<Car> = vec![];

    for _ in 0..5_000_000u32 {
        car_list2.push(Car { manufacturer: "Porshe".to_string(), model: "911".to_string() });
    }

    for _ in 0..100u8 {
        car_lot2.push(Car { manufacturer: "Hyundai".to_string(), model: "Sonata".to_string() });
    }

    car_list2.append(&mut car_lot2);
    car_list2.insert(0, 
        Car { manufacturer: "Lamborghini".to_string(), model: "Aventador".to_string()}
    );
    car_list2.remove(0);

    let keep = |e: &Car| {
        if e.manufacturer == "Porshe" {
            return true;
        } else {
            return false;
        }
    };
    car_list2.retain(keep);

    car_list2.reserve(5000);

    // info!("{:?}", car_list2);
    info!("Car list length: {:?}", car_list2.len());
    info!("Car lot length: {:?}", car_lot2.len());
    info!("{:?}", car_list2.capacity());
    info!("{:?}", car_list2.get(0).unwrap());
    
    let mut input = String::new();

    _ = std::io::stdin().read_line(&mut input).expect("Something bad happened");


}



#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_vec_car_test() {
        crate::init();
        
        let str_list = vec!["Trevor"; 10];
        info!("{:?}", str_list);

        test_vec_car();

    }

    #[test]
    fn it_vec_string_test() {
        crate::init();

        let first_names = vec!["Trevor", "Nancy", "Shannon", "Billy", "Rachel"];
        info!("{:?}", first_names);

        for item in first_names.clone() {
            info!("Processing {}...", item);
        }

        info!("{:?}", first_names);

    }

    #[test]
    fn it_test01() {
        crate::init();

        info!("test01");

        let mut my_ints: Vec<i32> = Vec::new();
        my_ints.push(30);
        my_ints.push(40);
        my_ints.push(50);
        my_ints.push(60);
        my_ints.push(70);
        my_ints.push(80);
        my_ints.push(90);

        info!("Size of Vec: {}", my_ints.len());
        info!("Capacity of Vec: {}", my_ints.capacity());
        info!("{:?}", my_ints);

        info!("First item in Vec is: {}", my_ints[0]);
        info!("Slice item in Vec is: {:?}", &my_ints.as_slice()[3..]);

        info!("Fist element is: {:?}", my_ints.get(10));


    }
}