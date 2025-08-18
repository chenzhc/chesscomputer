
fn largest<T>(list: &[T]) -> T  
    where T: PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2)  + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn longtest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y 
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_longtest_test() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longtest(string1.as_str(), string2);
        println!("The longest string is {}", result);

    }

    #[test]
    fn it_largest_test01() {
        let number_list = vec![34,50,25,100,65];

        let result = largest(&number_list);
        println!("The largest num is : {}", result);

        let char_list = vec!['y','m','a','q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);

    }

    #[test]
    fn it_point2_mixup_test() {
        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "hello", y: 'c' };

        let p3 = p1.mixup(p2);
        
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    }

    #[test]
    fn it_test_01() {
        println!("test");

        let p = Point { x: 10, y: 20 };
        println!("p.x = {}", p.x());


    }
}