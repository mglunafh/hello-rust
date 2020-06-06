// There are two types of constants which can be declared in all types of scope
//      including Global scope.
// 'const' is an unchangeable value.
// 'static' is a possibly mutable variable with 'static' lifetime. Its lifetime is inferred
//      and needs not to be specified. Accessing or modifying a mutable static variable is _unsafe_.

static LANGUAGE: &str = "Rust";
static mut PARADIGM: &str = "OOP";
const THRESHOLD: i32 = 15;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

// One does not simply declare to perform an unsafe operation
unsafe fn unsafe_block() {
    println!("=== Entered the unsafe block ===");
    println!("Paradigm is {}", PARADIGM);
    println!("=== Got out the unsafe block ===");
}

fn main() {
    let n = 16;

    println!("Is {} big enough? {}", n, is_big(n));
    println!("This is {}", LANGUAGE);

    // One does not simply perform an unsafe operation
    unsafe {
        unsafe_block();
    }

    unsafe {
        println!("=== Quickly learnt that the paradigm is {}", PARADIGM);
    }
}
