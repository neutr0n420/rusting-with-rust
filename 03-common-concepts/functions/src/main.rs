fn main() {
    println!("Hello, world!");
    another_function(32,'h'); 
   // what are expressions? -> those thing which retuns values are called as expressions
   let y = {
    let x = 3;
    //ending statement of the code block don't have semi-coloun cause it will be turend to the statement and will not be a expression
    x + 1
   };

    //The expression here is in the codeblock
   println!("{y}");

   // Let's get the valeu of five() function here
   let x = five();
   println!("The value of x is, {x}");
   let z = plus_one(3);
   println!("The value of z is, {z}");
  
}
fn another_function(x:u32, str:char){
    println!("Hello from another function with parameters: {x}, with message: {str}");
}

// functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x:u32) -> u32 {
    x+1
}