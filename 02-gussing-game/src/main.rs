use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");
    // here we are generating the number! from between 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100); 
  
 loop {
    println!("Please input the number");

    // here we are creating a new instance of the String
    let mut guess = String::new();

    // taking the user type number as input 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Read line");

    //.trim() -> eliminate the whitespaces from the string    
    // .parse() -> Convert the string to int as the type of guess is u32 unsigned 32bit integer
     let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid input! Please enter valid input");
                continue
            },
        };

   
    //comparing the "guess" number with the "secret" number 
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    }
}