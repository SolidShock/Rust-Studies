fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    }; // there's obviously else if as well
    
    println!("The value of number is: {}", number);
}
