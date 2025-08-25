#![allow(dead_code, unused_variables)]

pub fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);

    return opt1;
}

pub fn test_option_string() -> Option<String> {
    let mut opt1:Option<String> = None;
    opt1 = Some(String::from("Trevor Sullivan"));

    return opt1;
}

enum CharacterType {
    Archer,
    Warrior,
    Mage,
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        return match self {
            CharacterType::Archer => "Archer",
            CharacterType::Mage => "Mage",
            CharacterType::Warrior => "Warrior",
        }.to_string();
    }
}

pub fn test_option_chartype() -> Option<CharacterType> {
    let mut  chartype: Option<CharacterType> = None;
    chartype = Some(CharacterType::Archer);

    return chartype;
}

#[cfg(test)] 
mod tests {

    use super::*;

    #[test]
    fn it_option_chartype_test() {
        let rs_result = test_option_chartype();

        if rs_result.is_some() {
            println!("User has selected a character type");
            println!("type: {0}", rs_result.unwrap().to_string());
        } else {
            println!("Charter type is None");
        }

        

    }

    #[test]
    fn test_option_string_test() {
        let str_result = test_option_string();
        println!("Name is: {}", str_result.unwrap());

    }

    #[test]
    fn it_test01() {
        let rs_opt = test_option_type();
        println!("{:?}", rs_opt);

    }
}