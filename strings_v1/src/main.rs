fn main() {

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The version of the code using format! is much easier 
    //  to read and doesn’t take ownership of any of its parameters.
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("Hello, world: {}", s);
}
