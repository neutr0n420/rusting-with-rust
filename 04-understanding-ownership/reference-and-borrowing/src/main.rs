fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // here &m1 and &m2 is creating the reference to a string
    //
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{}", s);
    test1();
}
fn greet(g1: &String, g2: &String) -> (String, String) {
    println!("from another function {} {}!", g1, g2);
    (g1.to_string(), g2.to_string())
}

fn test1() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    println!("{}", s1);
    println!("{}", s2);
}
