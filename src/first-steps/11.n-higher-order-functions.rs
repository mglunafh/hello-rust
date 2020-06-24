
fn imperative(upper: u32) {

    let mut result = 0;

    let mut n = 1;

    while n * n < upper {
        result += n * n;
        n += 2;
    }

    println!("Imperative style: {}", result);
}

fn functional(upper: u32) {

    let is_odd = |n| n % 2 == 1;

    let sum_of_squared_odd_numbers =
        (0..).map(|n| n * n)
            .take_while(|&n| n < upper)
            .filter(|&n| is_odd(n))
            .fold(0, |acc, n| acc + n);

    println!("Functional style: {}", sum_of_squared_odd_numbers);
}

fn main() {
    let upper: u32 = 1000;
    imperative(upper);
    functional(upper);
}