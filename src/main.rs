fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // using if in a "let" statement
        // if and else arms result's must have the same type
    let y = if true {5}else{6};
    println!("number is: {}",y);
}