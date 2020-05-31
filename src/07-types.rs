// Implicit type conversion is disallowed, you can convert values explicitly with keyword 'as'

#![allow(overflowing_literals)]

fn casting() {
    let dec = 65.545_f32;

    let integer: u8 = dec as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", dec, integer, character);

    println!("1000 as a u16 is {}", 1000 as u16);
    println!("1000 as a u8 is {}", 1000 as u8);
    println!("1000 % 256 is {}", 1000 % 256);
    println!("-1 as a u8 is {}", (-1i8) as u8);

    println!("128 as an i8 is {}", 128 as i8);
    println!("232 as an i8 is {}", 232 as i8);
}

fn literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference_capabilities() {
    let elem = 355u8;

    let mut vect = Vec::new();      // type can be inferred
    let another:Vec<String> = Vec::new();   // type must be specified
    vect.push(elem);

    println!("mutable vect is {:?}", vect);
    println!("empty vect is {:?}", another);
}

type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn mess_with_types() {
    let nano: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    let real_inches = 14 as Inch;

    println!("{} nanoseconds + {} inches + {} real inches = {} unit?",
             nano, inches, real_inches,  nano + inches + real_inches);
}

fn main() {
    casting();
    literals();
    inference_capabilities();

    mess_with_types();
}

