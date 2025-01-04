fn main() {

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    } 

    println!("adicionando 50...");
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;           // dereference operator (*)
    }

    for i in &v {
        println!("{}", i);
    } 
 
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
       
    // printing the result in SpreadsheetCell

    for s in &row {
        println!("Spreadsheet: {:?}", s);
    } 

    // Imprimir apenas os valores armazenados
    for s in &row {
        match s {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }

    //     When you’re writing a program, if you don’t know the exhaustive set of 
    // types the program will get at runtime to store in a vector, the enum tech-
    // nique won’t work. Instead, you can use a trait object, which we’ll cover in 
    // Chapter 17.
}
