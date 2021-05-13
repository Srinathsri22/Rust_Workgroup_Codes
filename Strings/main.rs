use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0 ; 500];

    println!("last element of array ys is {}", xs[2]);
    println!("first element of array xs is {}", xs[0]);
    println!("length of array xs is {}", xs.len());
    println!("first element of array xs is {}", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
   println!("{:?}",analyze_slice(&ys[1 .. 4]));

    // Out of bound indexing causes compile error
   // println!("{}", xs[5]);

}