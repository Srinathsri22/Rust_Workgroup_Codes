fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                        -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("first value is {}", long_tuple.0);
    println!("third value is {}", long_tuple.2);

    let tuple_of_tuples = ((1u16, 2u32, 3u64), (1i16, 2i64), true);
    println!("Tuple of tuples is {:?}", tuple_of_tuples);
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse(pair));
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix); 
}