#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]


#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use log::info;

    use super::*;

    #[test]
    fn it_hashset_test() {
        crate::init();
        let mut planet_list = HashSet::from(["Mercury", "Venus", "Earth"]);
        let planet_list_more = HashSet::from(["Earth", "Mars", "Jupite"]);

        // let planet_diff =  planet_list.difference(&planet_list_more);
        
        // let planet_sdiff = planet_list.symmetric_difference(&planet_list_more);

        planet_list.insert("Saturn");
        planet_list.insert("Uranus");
        planet_list.insert("Neptune");
        planet_list.insert("Pluto");
        planet_list.insert("Pluto");


        // for planet in planet_sdiff {
        //     info!("diff: {}", planet);
        // }

        info!("{:?}", planet_list);

        for plane in planet_list {
            info!("Thanks for adding {}", plane);
        }

    }

    #[test]
    fn it_test01() {
        crate::init();

        info!("Test01");

        let mut stock_list:HashMap<String, f32> = HashMap::new();
        info!("{}", stock_list.len());
        info!("{}", stock_list.is_empty());

        stock_list.insert("NVDA".to_string(), 478.32);
        stock_list.insert("APPLE".to_string(), 1478.32);
        stock_list.insert("AMSC".to_string(), 50.78);
        stock_list.insert(String::from("APPLE"), 2000.29);

        stock_list.entry(String::from("META")).or_insert(28.90);

        info!("{:?}", stock_list);

        stock_list.remove("APPLE");

        info!("{:?}", stock_list);

        for (ticker, current_value) in stock_list {
            info!("{} is trading at {}", ticker, current_value);
        }

    }
}