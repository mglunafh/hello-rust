
fn basic_match(num: i32) {
    print!("Tell me about {}\n -- ", num);
    match num {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special")
    }
}

fn return_match() {
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0
    };
    println!("C equivalent of {} is {}", boolean, binary);
}

fn main() {
    let vect = vec![3, 1, 4, 15, 92];
    for n in vect.iter() {
        basic_match(*n);
    }

    return_match();
}