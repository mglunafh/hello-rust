
fn scope_stuff() {

    println!("OUTER --");
    let outer = 15;
    println!("outer = {}", outer);

    {
        println!("-- INNER --");
        let inner = 134;
        let outer = 136.5555_f32;      // <-- Shadows other binding
        println!("-- inner = {}", inner);
        println!("-- outer = {}", outer);
    }
    println!("OUTER --");
    println!("outer = {}", outer);

    let outer = "some_string";
    println!("outer = {}", outer);          // <-- Shadows other binding
}

fn declaration() {
    let binding;

    {
        let x = 15;
        binding = x * x;
    }
    println!("First binding: {}", binding);

    // Use of uninitialized bindings won't compile.
    let _another;
}

fn main() {
    scope_stuff();

    declaration();
}
