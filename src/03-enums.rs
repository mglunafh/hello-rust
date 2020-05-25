
enum WebEvent {
    // everyone of those is different and independent from one another.
    PageLoad,                   // unit-like variants
    PageUnload,                 //
    KeyPress(char),             // tuple-like structs
    Paste(String),              //
    Click {x: i64, y: i64}      // C-like structs
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// aliases for enums
enum VeryVerboseEnumDoToThingsWithNumbers {
    Add,
    Subtract
}

type Operations = VeryVerboseEnumDoToThingsWithNumbers;

impl VeryVerboseEnumDoToThingsWithNumbers {
    fn fun(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let t = Operations::Add;
    let result = t.fun(15, 17);
    println!("Result of Operations::Add is {}", result);
}