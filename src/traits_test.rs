#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]



use std::ops::Not;

use log::info;

#[derive(Debug)]
pub struct Person<T, U> 
    where T: Animal + NotDangerous,
          U: Animal + Dangerous
{
    first_name: String,
    pet: T,
    pet2: U,
}


pub trait Animal {
    fn make_sound(&self) -> ();
}

pub trait NotDangerous {
    
}

pub trait Dangerous {
    
}

#[derive(Debug)]
struct Dog {

}

impl NotDangerous for Dog {

}

impl Animal for Dog {
    fn make_sound(&self) -> () {
        info!("Dog barked!");
    }
}

#[derive(Debug)]
struct Cat {

}

impl Animal for Cat {
    fn make_sound(&self) -> () {
        info!("Cat meowed!");
    }
}

impl NotDangerous for Cat {

}

#[derive(Debug)]
struct Bear {

}

impl Dangerous for Bear {

}

impl Animal for Bear {
    fn make_sound(&self) -> () {
        info!("Bear roared!");
    }
}

#[derive(Debug)]
struct Tiger {

}

impl Dangerous for Tiger { 

}

impl Animal for Tiger {
    fn make_sound(&self) -> () {
        info!("Tiger roared!");
    }
}

impl Dog {
    fn bark(&self) {
        info!("Bark!");
    }
}

struct Character {
    hit_points: u16,
}

pub fn create_person<T,U>(pet: T, pet2: U) -> Person<T, U>
    where T: Animal + NotDangerous,
          U: Animal + Dangerous
{
    let p1 = Person {
        first_name: "Trevor".to_string(),
        pet: pet,
        pet2: pet2,
    };


    return p1;
}


#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_create_person_test() {
        crate::init();

        let pet1 = Cat{};
        let pet2 = Tiger {};


        let p1 = create_person(pet1 , pet2);
        info!("{:?}",p1);
        p1.pet.make_sound();
        p1.pet2.make_sound();


    }

    #[test]
    fn it_test01() {
        crate::init();

        info!("test log");


    }
}