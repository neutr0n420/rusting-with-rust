fn main() {
    //     let number = 0;
    //     if number != 0 {
    //         println!("number was something other than zero")
    //     }
    //    println!("Number was zero")
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");
    another_function();
    loop_over_collection();
    for_loop();
}

fn another_function() {
    let mut number = 6;
    // basic while loop
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}
// There might be much more vernabilities in the code, like if the size of the array is 5 then what can happen
fn loop_over_collection() {
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
}

// This is much more safe code eleminates the chances of breaking code
fn for_loop() {
    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("The elemetns in the 'arr' array are: {element}");
    }
}