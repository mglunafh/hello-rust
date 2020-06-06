#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// That's a unit struct
struct Nil;

// that's a tuple struct
struct IntPair(i32, i32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn area(rect: Rectangle) -> f32 {
    let p1 = rect.top_left;
    let p2 = rect.bottom_right;
    return (p2.x - p1.x) * (p1.y - p2.y);
}

fn square(p_lower_left: Point, size: f32) -> Rectangle {
    let x_top_left = p_lower_left.x;
    let y_top_left = p_lower_left.y - size;

    let x_right_bottom = p_lower_left.x + size;
    let y_right_bottom = p_lower_left.y;

    let _result = Rectangle {
        top_left: Point { y: p_lower_left.y - size, ..p_lower_left },
        bottom_right: Point { x: p_lower_left.x + size, ..p_lower_left },
    };

    let result = Rectangle {
        top_left: Point { x: x_top_left, y: y_top_left },
        bottom_right: Point { x: x_right_bottom, y: y_right_bottom },
    };

    return result;
}

fn main() {
    println!("hello world from structs!");

    let name = "Peter";
    let age = 34;
    let peter = Person { name, age };

    let brother_name = "Chris";
    let christian = Person { name: brother_name, age };
    //  This will end up with a compilation error (sic!)
    // let christian = Person {brother_name, age};

    println!("{:?}", peter);
    println!("{:?}", christian);

    // constructor arguments are NOT positional!
    // Be very careful with variable naming.
    let y = 0.3f32;
    let x = 0.4f32;
    let point: Point = Point { y, x };
    println!("Point: ({}, {})", point.x, point.y);

    // We took the rest of the fields from the 'point' object
    let bottom_right: Point = Point { x: 5.2, ..point };
    println!("Point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;
    println!("That's how we destructure structs:");
    println!("let Point{{x: top_edge, y: left_edge}} = bottom_right;");
    println!("top_edge = {}; left_edge = {}", top_edge, left_edge);

    let rect = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };

    let _nil = Nil;
    let pair = IntPair(1, 15);
    println!("Pair contents. first: {:?}, second: {:?}", pair.0, pair.1);
    let IntPair(n, m) = pair;
    println!("pair contains {:?} and {:?}", n, m);

    println!("Area of a rectangle: {:?}", area(rect));
    println!("Square: {:?}", square(point, 0.2));
}