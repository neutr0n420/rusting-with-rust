fn main() {
    stack_fn(); // call the function that uses stack memory
    heap_fn(); // call the functio that uses heap memory
    update_string(); // call the function that changes size of variable at runtime
}

fn stack_fn() {
    // declaring bunch of variable which will be pushed to the first layer of the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {a} and {b} is {c}");
}

fn heap_fn() {
    // here a heap is allocated and that contain three different sting and a stack pointer point
    // toward the starting of the string
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{s1} {s2}");
    println!("Heap function: Combined string is {combined}");
}

fn update_string() {
    // As we update the string theere are multiple things going on here.
    // stack have a pointer that was pointing towards the original string
    // but as the string updates the pointer maybe updated cause it doesn't have that much space
    // for contigious memory to allocate
    let mut s = String::from("Initial string");
    println!("Before update: {s}");
    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    s.push_str(" adding some additional text");
    println!("After update: {s}");
    println!(
        "Capacity: {}, Length: {}, pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}
