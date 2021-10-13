use std::io;

fn main() {
    
    println!("Please enter degrees in C°: ");
    
    let mut cdegrees = String::new();
    
    io::stdin().read_line(&mut cdegrees).expect("Failed to read line!");
    let cdegrees: i32 = cdegrees.trim().parse().expect("Please enter a number!");
    
    let mut fdegrees = (cdegrees * 9/5) + 32;
    println!("In F° that is: {}", fdegrees);
}