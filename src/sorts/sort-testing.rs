
fn test_data() -> Box<[(&[i32; 2], &[i32; 2]); 2]> {
    let arr1 = (&[1,3], &[1,3]);
    let arr2 = (&[1, 4, 8 ,8], &[1, 4, 8, 8]);

    let result = Box::new([arr1, arr2]);
    result
}

fn main() {
    println!("{:?}", test_data());
}