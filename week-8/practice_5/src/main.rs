fn main() {
    // Create an empty vector "City"
    let mut city : Vec<String> = Vec::new();
    // Print City Vector
    println!("The City vector has element {}",city.len());
    // Push new elements into
    let mut input1 = String::new();
    println!("How many Cities do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse();
}
