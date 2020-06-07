
fn stalin_sort(arr: &mut[i32]) {
    let mut i:usize = 0;
    let mut shift: usize = 0;
    let mut total = arr.len() - 1;
    while i < total {
        if arr[i] > arr[i + shift + 1] {
            i -= 1;
            total -= 1;
            shift += 1;
        }
        else {
            arr[i + 1] = arr[i + shift + 1];
            i += 1;
        }
    }
    i += 1;
    while i < arr.len() {
        arr[i] = 0;
        i += 1;
    }
}

fn main() {
    let mut arr = [3, 14, 92, 9, 59, 14, 88, 89, 90];
    stalin_sort(&mut arr);
    println!("{:?}", arr);
}
