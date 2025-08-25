#![allow(dead_code, unused_variables)]

use std::cell::Cell;

use log::info;

pub struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
    meters_walked: u32,
}

impl Person {
    fn walk_meters(&mut self, meters: u32) {
        self.meters_walked = meters;
    }
}

pub fn new_psersion() -> Person {
    let mut  p1 = Person {
        first_name: String::from("test01"),
        last_name: String::from("value"),
        birth_month: 2,
        birth_year: 2025,
        visited_europe: false,
        meters_walked: 3,
    };

    p1.first_name = String::from("Shannon");

    return p1;
}

pub fn test_crate_person() {
    let mut p1 = new_psersion();
    p1.walk_meters(30);

    info!("First name: {0}, 
    last name: {1}, 
    birth month: {2}, 
    birth year: {3}, 
    visisted europe: {4} 
    meters_walked: {5}",
        p1.first_name, 
        p1.last_name,
        p1.birth_month,
        p1.birth_year, 
        p1.visited_europe,
        p1.meters_walked);
}

#[derive(Debug)]
enum VehicleColor {
    Silver,
    Blue,
    Red,
    Black,
    White,
    Green,
}

#[derive(Debug)]
pub struct Vehicle {
    manufacturer: String,
    model: String, 
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color; 
    }

    pub fn create_vehicle() -> Vehicle {
        let v1 = Vehicle {
            manufacturer: "Porsche".to_string(),
            model: String::from("911"),
            year: 1991,
            color: VehicleColor::Red,
        };

        return v1;
    }
}

pub fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle {
        manufacturer: "Porsche".to_string(),
        model: String::from("911"),
        year: 1991,
        color: VehicleColor::Red,
    };

    v1.paint(VehicleColor::Black);

    return v1;
}

pub fn create_vehicle() {
    // let myvehicle = new_vehicle();
    let myvehicle = Vehicle::create_vehicle();

    info!("{:?}", myvehicle);

}

pub struct VehicleTuple(String, String, u16);

pub fn new_vehicletuple() -> VehicleTuple {
    return VehicleTuple(String::from("DasAuto"), String::from("Tugun"), 1993);
}

#[cfg(test)]
mod tests {
    use log::info;
    use super::*;

    #[test]
    fn it_struct_function_test() {
        crate::init();
        let v1 = Vehicle::create_vehicle();
        info!("{:?}", v1);
    }

    #[test]
    fn it_create_vehicle_test() {
        crate::init();

        create_vehicle();

        let vtup1= new_vehicletuple();
        info!("{0}, {1}, {2}", 
            vtup1.0, vtup1.1, vtup1.2);
    }

    #[test]
    fn it_test01() {
        crate::init();

        info!("starting up");

        log::debug!("Hi");

        info!("Checking whether it still works...");
        log::info!("Commencing yak shaving{}", 0);
        println!("test01");

        test_crate_person();
        // create_vehicle();

    }
}