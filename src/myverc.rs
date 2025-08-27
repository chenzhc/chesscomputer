#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]





#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

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