fn main() {

    // Combining Owned and Borrowed Variants
    // ESTE EXEMPLO ENCONTRA-SE NO CHATGPT APRENDER RUST RAPIDO
    enum Example<'a> {
        Owned(String),     // Owned variant.
        Borrowed(&'a str), // Borrowed variant with a lifetime.
    }
    

    let text = "Borrowed text";
    let owned_variant = Example::Owned(String::from("Owned text"));
    let borrowed_variant = Example::Borrowed(text);

    match owned_variant {
        Example::Owned(data) => println!("Owned1: {}", data),
        Example::Borrowed(data) => println!("Borrowed1: {}", data),
    }

    match borrowed_variant {
        Example::Owned(data) => println!("Owned2: {}", data),
        Example::Borrowed(data) => println!("Borrowed2: {}", data),
    }
    
    println!("Hello, world!");
}
