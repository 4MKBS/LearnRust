
pub fn tuple() {
    // A tuple with a bunch of different types.
    // let long_tuple = (255u8, 65535u16, 3u32, 4u64,
    //                   -1i8, -2i16, -3i32, -4i64,
    //                   0.1f32, 0.2f64,
    //                   'a', true);
    // println!("Long tuple first value: {}", long_tuple.0);
    // println!("Long tuple second value: {}", long_tuple.1);
    // let name:&str;
    // println!("Enter your name: ");
    // std::io::stdin().read_line(&mut name).unwrap();
    // println!("Your name is {}",name);

    let mut input = String::new();
    println!("Enter your name: ");
    std::io::stdin().read_line(&mut input).unwrap();

    println!("Your name is {}",input);
}