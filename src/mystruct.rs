#![allow(dead_code, unused_variables)]

use log::info;

#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
}

pub fn new_psersion() -> Person {
    let p1 = Person {
        first_name: String::from("test01"),
        last_name: String::from("value"),
        birth_month: 2,
        birth_year: 2025,
        visited_europe: false,
    };

    return p1;
}

pub fn test_crate_person() {
    let p1 = new_psersion();
    info!("First name: {0}, last name: {1}, birth month: {2}, birth year: {3}, visisted europe: {4}",
            p1.first_name, 
            p1.last_name,
            p1.birth_month,
            p1.birth_year, 
            p1.visited_europe);
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

pub fn new_vehicle() -> Vehicle {
    let v1 = Vehicle {
        manufacturer: "Porsche".to_string(),
        model: String::from("911"),
        year: 1991,
        color: VehicleColor::Red,
    };

    return v1;
}

pub fn create_vehicle() {
    let myvehicle = new_vehicle();
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
        create_vehicle();

    }
}