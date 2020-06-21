
fn bubble_sort(arr: &mut[i32]) {
    for i in 0..arr.len() {
        for j in i..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                let t = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = t;
            }
        }
    }
}

fn select_sort(arr: &mut[i32]) {
    for i in 0..arr.len() {
        let mut ind = i;
        for j in i+1..arr.len() {
            if arr[ind] > arr[j] {
                ind = j;
            }
        }
        let t = arr[i];
        arr[i] = arr[ind];
        arr[ind] = t
    }
}

fn main() {
    let mut arr = [3, 14, 92, 9, 59, 14, 88, 89, 90];
    println!("{:?}", arr);
    select_sort(&mut arr);
    println!("{:?}", arr);
}