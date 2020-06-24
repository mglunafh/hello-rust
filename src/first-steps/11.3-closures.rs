
fn closures() {
    fn function (i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 15;

    println!("Regular function: {}", function(i));
    println!("Annotated closure: {}", closure_annotated(i));
    println!("Inferred closure: {}", closure_inferred(i));
}

// Closures are able to capture variables from the outer scope three ways:
// 1) by reference:         &T
// 2) by mutable reference: &mut T
// 3) by value:             T
fn capturing() {
    use std::mem;

    let color = "green";

    // 'print' borrows a value of 'color'.
    // 'Color' remains borrowed until 'print' is called the last time.
    let print = || println!("Color: {}", color);
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;

    // 'mut' on the closure is required bc we use mutable variable inside it

    let mut incr = || {
        count += 1;
        println!("count is {}", count);
    };
    incr();

    // this borrow cannot be immutable since 'count' is declared as mutable
//    let _reborrow = &count;
//    incr();

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {}", movable);
        mem::drop(movable)
    };

    consume();

    // You cannot call consume twice
//  consume();
}

fn captures_cont() {
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&2));
    println!("{}", contains(&4));

    // We used 'move' during the declaration of the 'contains',
    // forcing the closure to take ownership of the captured variables,
    // in this case 'haystack'
//    println!("Elements of the haystack: {}", haystack.len());
}

fn main() {

    closures();
    capturing();
    captures_cont();
}