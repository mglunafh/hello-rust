fn some_exprs() {
    let x = 48i32;

    let y = {
        let x_sqr = x * x;
        let x_cube = x * x_sqr;

        x_cube + x_sqr + x
    };

    // Here we accidentally typed a semicolon
    // and thus suppressed returning of the calculated result
    let z = {
      2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn if_else_expressions() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    }
    else if n > 0 {
        print!("{} is positive", n);
    }
    else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
             n / 2
        };

    println!("{} -> {}", n, big_n);
}

fn infinite_loops() {

    let mut count = 0u32;
    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("Three!");
            continue;
        }

        println!("Count: {}", count);
        if count == 5 {
            println!("Ok, that's enough for today");
            break;
        }
    }
}

fn nesting_loops() {

    let mut outer = 7;
    'outer: loop {
        println!("Outer loop");

        'inner: loop {
            if outer < 0 {
                break
            } else if outer == 0 {
                break 'outer;
            }
            println!("Inner loop, {}", outer);
            outer -= 2;
        }
        outer += 7;
    }
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result of loop is {}", result);
    assert_eq!(result, 20);
}

const MAX_NUMBER: i32 = 25;

fn while_loop() {
    let mut n = 1;

    while n < MAX_NUMBER {
        if n % 15 == 0 {
            println!("FizzBuzz");
        }
        else if n % 3 == 0 {
            println!("Fizz");
        }
        else if n %  5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", n);
        }
        n += 1;
    }

}

// used to iterate through the an Iterator.
fn for_loops() {
    print!{"Numbers less than 25: "};
    for n in 1..MAX_NUMBER {
        if n % 3 == 0 {
            print!{"{} ", n};

        }
    }
    println!();
    print!{"Numbers less or equal than 25: "};
    for n in 1..=MAX_NUMBER {
        if n % 5 == 0 {
            print!{"{} ", n};

        }
    }
    println!();
}

fn three_ways_to_iterate() {
    let names = vec!["Bob", "Frank", "Ivan"];
    let alert = "Get the spy!";

    println!("=== First way: we borrow every element in the collection.");
    for name in names.iter() {
        match name {
            &"Ivan" => println!("{}", alert),
            _ => println!("Hello, {}", name)
        }
    }

    println!("=== Second way: collection is consumed, exact data is provided.");
    println!("===             Collection is not reusable again.");
    for name in names.into_iter() {
        match name {
            "Ivan" => println!("{}", alert),
            _ => println!("Hello, {}", name)
        }
    }

    let mut names = vec!["Bob", "Frank", "Ivan"];
    println!("=== Third way: Every element is mutably borrowed.");
    println!("===            Collection can be modified in-place.");
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ivan" => alert,
            _ => "Hello"
        }
        // code snippet above replaces elements of the collection!
    }
    println!("names: {:?}", names);
}

fn main() {
    some_exprs();
    if_else_expressions();
    infinite_loops();
    nesting_loops();
    return_from_loop();

    while_loop();
    for_loops();
    three_ways_to_iterate();
}