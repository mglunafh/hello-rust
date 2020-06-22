fn main() {
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) {
    for i in 1.. n + 1 {
        let result = fizzbuzz(i);
        println!("{}", result);
    }

}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return true;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> String {
    if is_divisible_by(n, 15) {
        String::from("fizzbuzz")
    }
    else if is_divisible_by(n, 3) {
        String::from("fizz")
    }
    else if is_divisible_by(n, 5) {
        String::from("buzz")
    }
    else {
        format!("{}", n)
    }
}
