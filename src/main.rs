use ferris_says::say;
use std::io::{stdout,BufWriter};
use std::mem;

fn variables() {
    let byte = 5i8;
    let short = 5i16;
    let longlong = 5i128;

    let result = longlong + short as i128 + byte as i128;
    println!("signed sum: {}", result);

    let ubyte = 5u8;
    let ushort = 5u16;
    let ulonglong = 5u128;
    let uresult = ulonglong + ushort as u128 + ubyte as u128;
    println!("unsinged sum: {}", uresult);

    let character = 'a'; // 4 bytes Unicode scalar value
    println!("{}", character);

    let logical: bool = true;
    let f = 1.0;
    let i = 5i32;
    let ternary =  if logical { f + i as f64 }  else {f - i as f64};
    println!("Ternary operator result: {}", ternary);

    let def_float = 3.0;
    let def_int = 7;
    println!("some numbers: {} {}", def_float, def_int);

    let mut inferred = 12;
    println!("at first mutable is {}", inferred);
    inferred = 4294967296i64;

    let mut mutable = 21;
    println!("Print mutables: {} {}", inferred, mutable);
}

fn arrays_and_tuples() {
    let arr = [1, 4, 8, 8];
    let arr2 = [1i8, 2i8, 4 , 127];

    let tuple = (1, true);
    let tuple2 = (14.0f32, 88f64);

    let (a, b) = tuple2;
    println!("tuple: {:?}", tuple);
    println!("{} {}", a, b);

    for r in arr.iter() {
        print!("{} ", r);
    }
    println!();

    for r in &arr2 {
        print!("hey {} ", r);
    }
    println!();
}

fn operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn arrays_closer() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];     // superfluous syntax
    let ys: [i32; 500] = [14; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}

fn main() {
	let stdout = stdout();
    let t = "hello world!";
    let width   = t.chars().count();
	println!("width: {}", width);

    let mut writer = BufWriter::new(stdout.lock());
    say(t.as_bytes(), width, &mut writer).unwrap();

    operators();
    arrays_closer();
    variables();
    arrays_and_tuples();
}
