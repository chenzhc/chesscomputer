
pub fn test_closeures() {
    let add = || println!("Returing some text");

    add();

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_test02() {
        println!("test");
        test_closeures();

    }

}