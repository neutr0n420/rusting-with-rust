use std::io;
fn main() {

    // INTEGERS
// There are 5 types of integers and two catagories of them
// The two catagories are unsigned and signed 
// 8bit -> i8 and u8 i-> signed integer | u -> unsigned integer 
// 16bit -> i16 and u16 i-> signed integer | u -> unsigned integer 
// 32bit -> i32 and u32 i-> signed integer | u -> unsigned integer 
// 64bit -> i64 and u64 i-> signed integer | u -> unsigned integer 
// 128bit -> i128 and u128 i-> signed integer | u -> unsigned integer 

// Each signed variant can store numbers from -(2^n - 1) to 2^n - 1 - 1  eg: i = 8 -> 2^-7 to 2^7 

//tuple in Rust
//destruciting of tuples
    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("with destructured tuple: {x},{y}, {z}");

    let x:(i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_four = x.1;
    let one = x.2;
    println!("Without destructed tuple: {five_hundred}, {six_four}, {one}");
    
    let array = [1,2,3,4,5];
    println!("Please enter the index of the array");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Faild to Read line");

    let index:usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number");

    let element = array[index];
    println!("The value of the entered index {index} is : {element}");

}
