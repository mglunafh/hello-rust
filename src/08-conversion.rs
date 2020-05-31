use std::convert::{TryFrom, TryInto};
use std::fmt;

fn from_trait() {

    let my_str = "some_string";
    let my_string = String::from(my_str);
    println!("my_str is {}", my_str);
    println!("my_string is {}", my_string);
}

#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}

#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64
}

impl From<f64> for Complex {
    fn from(num: f64) -> Self {
        Complex {re: num, im: 0f64}
    }
}

fn using_from_into() {

    let num = Number::from(1488);
    println!("My number is {:?}", num);

    let complex = Complex::from(15_f64);
    println!("My complex is {:?}", complex);

    let float = 5f64;

    let c1: Complex = float.into();
    println!("Number {} got converted into {:?}", float, c1);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn using_tryfrom() {
    let some_even = EvenNumber::try_from(14);
    let another = EvenNumber::try_from(15);

    assert_eq!(some_even, Ok(EvenNumber(14)));
    assert_eq!(another, Err(()));

    println!("Some number: {:?}", some_even);
    println!("Another number: {:?}", another);

    let result:Result<EvenNumber, ()> = 8i32.try_into();
    println!("Some result: {:?}", result);
    let result:Result<EvenNumber, ()> = 69i32.try_into();
    println!("Some result: {:?}", result);
}

struct Circle {
    rad: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.rad)
    }
}

fn string_conversions() {
    let circle = Circle {rad: 15};
    println!("{}", circle.to_string());     // this method got implemented, huh.
    println!("{}", circle);
}

fn parse_numbers() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_fish = "10".parse::<i32>().unwrap();

    let bad: Result<i32, _> = "hah".parse();
    let good: Result<i32, _> = "1488".parse();

    let sum = parsed + turbo_fish;
    println!("Sum: {}", sum);
    println!("Good: {:?}", good);
    println!("Bad: {:?}", bad);
}

fn main() {

    from_trait();
    using_from_into();
    using_tryfrom();
    string_conversions();
    parse_numbers();
}
