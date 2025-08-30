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

        info!("test01");

        let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Oragne", "Apple"];
        info!("{:?}", fruit_list);

        let fruit_iter = fruit_list.iter();

        for fruit in fruit_iter {
            info!("{:?}", fruit);
        }
        
    }
}