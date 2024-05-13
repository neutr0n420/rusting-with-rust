// const THREE_HOURS_IN_SEC : u32 = 60*60*3;
fn main() {
    // let mut x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");
    // println!("The hours in sec are {THREE_HOURS_IN_SEC}");

    //shadowing in rust 
    let x = 5;

    let x = x * 2;
    {
        let x = x + 2;
        println!("The value of inner block of x is: {x}");
    }

        println!("The value of outer block of x is: {x}");
    // The actual concept of the shadowing is that here the value of variable is string
    let spaces = "   .";
    println!("The spaces are {spaces}");
    //but it can be changed to int without
    let spaces = spaces.len();

    println!("The spaces are {spaces}");
}
