#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]


#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn it_test01() {
        crate::init();
        let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Oragne", "Apple"];
        let nut_list = vec!["Walnut", "Almonds", "Pecans", "Brail Nuts"];

        info!("{:?}", fruit_list);

        let mut fruit_iter = fruit_list.iter();

        let item01 = fruit_iter.next();
        info!("Frist item in Iterator is: {}", item01.unwrap());

        let aggregate_foods =  fruit_list.iter().chain(&nut_list);

        let all_foods: Vec<&&str> = aggregate_foods.clone().collect();
        info!("{:?}", all_foods);

        // Testing .chan() method 
        for food in aggregate_foods {
            info!("Eating {}", food);
        }

        let mut fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));
        info!("{:?}", fruit_list_strings);
        let new_fruits = fruit_list_strings.map(|mut e| { e.push_str(" fruit"); return e});

        new_fruits.clone().for_each(|e| info!("{:?}", e));

        let last = new_fruits.clone().last();
        info!("Last fruit is: {}", last.unwrap());

        let mut stepby = new_fruits.clone().step_by(2);
        info!("{}", stepby.next().unwrap());

        let first_name = vec!["Trevor", "Shannon", "James", "Tasha"];
        let first_name_strings = 
            first_name.iter().map(|e| String::from(*e));

        let last_name = vec!["Jones", "Sullivan", "Tanner", "Redman"];
        let last_name_strings = 
            last_name.iter().map(|e| String::from(*e));

        let full_names = first_name_strings.zip(last_name_strings);
        full_names.clone().for_each(|e| info!("{} {}", e.0, e.1));

        for (index, value) in full_names.clone().enumerate() {
            info!("Index: {0} value: {1} {2}", index, value.0, value.1);
        }

        // let mut f_iter = full_names.clone().skip(1);
        full_names.clone()
            .skip(2)
            .take(1)
            .for_each(|e| info!("{}, {}", e.0, e.1));

        let foods = vec![
            ("potatoes", 12), 
            ("strawberries", 25),
            ("burgers",31),
            ];
        let rood_quantity = foods.clone().iter().fold(0, |acc, x| acc + x.1);
        info!("Your total food quantity is: {}", rood_quantity);

        let foods_iter = foods.iter();
        let mut mypeekable = foods_iter.peekable();
        mypeekable.next();
        let food = mypeekable.peek();
        info!("{:?}", food);

    }
}