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
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn for_enums() {
    let color = Color::RGB(122, 70, 89);
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
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
        y: i32,
    }

    let foo = Foo { x: (3, 14), y: 15 };

    match foo {
        Foo { x: (3, b), y } => println!("First x is '3', b = {}, y = {}", b, y),
        Foo { x, y: 15 } => println!("x is {:?}, y is '15'", x),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
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

fn aux_age() -> u32 {
    15
}

fn optional_age(x: u32) -> Option<u32> {
    return if x <= 0 { None } else { Some(14 * x) };
}

fn binding() {
    println!("Tell me what type of person you are");

    let conclusion = match aux_age() {
        0 => String::from("i'm not born yet"),
        n @ 1..=12 => format!("I'm a child of age {:?}", n),
        n @ 13..=19 => format!("I'm a teenager of age {:?}", n),
        n => format!("I'm an adult person of age {:?}", n)
    };
    println!("{}", conclusion);

    for i in 0..=4 {
        print!("Attempt {}: ", i);
        match optional_age(i) {
            Some(n @ 42) => println!("The answer: {}!", n),
            Some(n) if n < 15 => println!("Huh, that {} is not impressive", n),
            Some(n) => println!("Not interesting... {}", n),
            _ => ()
        }
    }
}

fn if_let() {
    let opt = Some(7);
    let letter: Option<i32> = None;
    let i_like_letters = false;

    // We'e got a rightward drift, 2 indentations
    match opt {
        Some(i) => {
            println!("This is a {} right here", i);
        },
        None => {}
    }

    if let Some(k) = opt {
        println!("This is a {} without indentation", k);
    }

    // Advanced capabilities of 'if let' syntax
    if let Some(c) = letter {
        println!("Matched {:?}!", c);
    }
    else if i_like_letters {
        println!("Didn't match a number, let's go with a letter");
    }
    else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

#[derive(Debug)]
enum Foo { Bar, Baq }

fn match_non_comparable() {
    let a = Foo::Bar;

    //// This code does not work since Foo does not derive PartialEq trait
    //// and we cannot use '==' operator and compare anything at all
    // if a == Foo::Bar {
    //     println!("a is a so-called 'foobar', we compared it!");
    // }

    // But this if-let destructuring works!
    if let Foo::Bar = a {
        println!("a is a so-called 'foobar'");
        println!("{:?}", Foo::Baq)
    }
}

fn main() {
    for_tuples();
    for_enums();
    for_pointers();
    for_structs();
    with_guards();
    binding();
    if_let();
    match_non_comparable();
}