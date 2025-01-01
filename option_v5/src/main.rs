fn find_username(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None // Explicitly return "None" if no value is found
    }
}

fn main() {
    let username = find_username(2);

    match username {
        Some(name) => println!("Username: {}", name),
        None => println!("No user found"),
    }

    let username = find_username(1);

    match username {
        Some(name) => println!("Username: {}", name),
        None => println!("No user found"),
    }

    // Using unwrap and unwrap_or
    let value = Some(5);
    println!("{}", value.unwrap()); // Prints 5

    let value: Option<i32> = None;
    println!("{}", value.unwrap_or(-1)); // Prints 10

    let value: Option<i32> = Some(7);
    println!("{}", value.unwrap_or(-1)); // Prints 10


}
