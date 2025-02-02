use std::io;

fn checker(){
    let mut input = String::new();
    println!("Enter  character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("invalid input");

    if ch 
    {
        println!("Character '{}' is a digit",ch);
    }
    else 
    {
        print!("Character '{}' is not a digit", ch);
    }
}

fn main() {
    //calling function
    println!("Welcome! This program checks wether a character variable contains a digit or not");
    checker()
}
