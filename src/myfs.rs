#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use std::{fs, path};

use log::info;

pub fn test_create_dir() {
    let path: &'static str = "./data";
    
    let my_path = path::Path::new(path);
    
    if my_path.exists() {
        info!("Directory already exisits! Skeeping creating ...");
        return ;
    }
    let create_dir_result = fs::create_dir(path);

    if create_dir_result.is_ok() {
        info!("Created new data directory!");
    } else {
        info!("Some problem occurred creating data directory. {:?}", create_dir_result.err());
    }

}

pub fn test_crate_file() {
    let path = "./data/file01.txt";
    let path2 = "./data/file02.txt";
    let path3 = "./data/file03.txt";
    let text = "Trevor Sullivan";
    let text2 = "Nacy Drew";
    let text3 = "Shannon Jones";
    let _ = fs::write(path, text);
    let _ = fs::write(path2, text2);
    let _ = fs::write(path3, text3);

    // let _ = fs::remove_file(path2);

}

pub fn remove_dir() {
    let path = "./data";
    let rm_result = fs::remove_dir_all(path);

    info!("{:?}", rm_result.err());

}

pub fn read_somefile() {
    let file_to_read = "./data/file01.txt";
    let read_result = fs::read(file_to_read);

    let convert_bytes_to_string = |mut a: String, v: &u8 | {
        let new_char = char::from(*v);
        _ = a.push(new_char);
        return a;
    };

    if read_result.is_ok() {
        info!("Data found is {}", read_result.ok().unwrap().iter().fold(String::from(""),convert_bytes_to_string));
    }
}


#[cfg(test)]
mod tests {

    use log::info;

    use super::*;

    #[test]
    fn it_read_somefile_test() {
        crate::init();
        read_somefile();
    }

    #[test]
    fn it_remove_dir_test() {
        crate::init();
        remove_dir();
    }

    #[test]
    fn it_create_file_test() {
        crate::init();
        test_crate_file();

    }

    #[test]
    fn it_test_log_test01() {
        crate::init();

        info!("test01012211");
        test_create_dir();

    }
}