fn main() {

    // Listing 8-12: Using the to_string method to create a String from a string literal
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // Listing 8-13: Using the String::from function to create a String from a string literal
    let s = String::from("initial contents");

    // Listing 8-14: Storing greetings in different languages in strings
    // All of these are valid String values.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Listing 8-15: Appending a string slice to a String using the push_str method
    let mut s = String::from("foo");
    s.push_str("bar");


    println!("Hello, world! {}", s);

    //  Listing 8-16: Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Listing 8-18: Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("Hello, world! {}", s3);
    println!("Hello, world! {}", s2);
    //println!("Hello, world! {}", s1);  // esse comando dá erro!!!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Hello, world: {}", s);
    println!("Hello, world: {}", s1);  // somente esse comando dá erro !!!
    println!("Hello, world: {}", s2);
    println!("Hello, world: {}", s3);

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Hello, world: {}", s);

   
}
