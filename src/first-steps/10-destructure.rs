
fn for_tuples() {
    let vect = vec![(14, 1), (0, 8), (15, 0), (6, 6)];
    for pair in vect.iter() {
        println!("Tell me about {:?}", pair);

        match *pair {
            (0, y) => println!("It lies on  Ox axis, y is {}", y),
            (x, 0) => println!("It lies on Oy axis, x is {}", x),
            _ => println!("Doesn't matter at all")
        }
    }

}

#[allow(dead_code)]
enum Color {
    Red, Blue, Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32)
}

fn for_enums() {
    let color = Color::RGB(122, 70, 89);
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
}

// Distinction has to be made between destructuring and dereferencing
fn for_pointers() {
    let reference = &4;

    match reference {
        &val => println!("Got the val via destructuring: {}", val)
    }
    match *reference {
        val => println!("Dereferenced and got the val : {}", val)
    }

    // Just a binding for the value
    let not_reference = 15;

    // If we got only a value, this construct will help us
    // obtain a reference to it
    let ref definitely_reference = not_reference;
    println!("The reference is {}", definitely_reference);

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r)
    }
    match mut_value {
        ref mut mr => {
            *mr += 10;
            println!("We added 10, so the result is {:?}", mr);
        }
    }

}

fn for_structs() {
    struct Foo {
        x: (i32, i32),
        y: i32
    }

    let foo = Foo { x: (3, 14), y: 15 };

    match foo {
        Foo {x:(3, b), y} => println!("First x is '3', b = {}, y = {}", b, y),
        Foo {x, y: 15} => println!("x is {:?}, y is '15'", x),
        Foo {y, ..} => println!("y = {}, we don't care about x", y),
    }
}

fn with_guards() {
    let pairs = vec![(14, 1), (0, 8), (15, 0), (6, 6)];

    for pair in pairs.iter() {
        print!("Tell me about {:?}. -- ", pair);
        match *pair {
            (x, y) if x == y => println!("These are twins, {}", x),
            (x, y) if x + y == 0 => println!("Antimatter, annihilated!"),
            (x, _) if x % 2 == 1 => println!("The first one '{}' is odd", x),
            _ => println!("No correlation...")
        }
    }
}

fn main() {
    for_tuples();
    for_enums();
    for_pointers();
    for_structs();
    with_guards();
}