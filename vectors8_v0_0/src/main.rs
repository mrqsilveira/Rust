fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //  so we don’t need the Vec<i32> annotation


    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    let third2: Option<&i32> = v.get(2);

    println!("Third: {:?}", third2 );

    match v.get(2) {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value found at this index."),
    }

    match third2 {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value found at this index."),
    }

    // acessando um elemento que não existe

    let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100];   // esse dá panic
    let does_not_exist = v.get(100);


    // Attempting to add an element to a vector while holding a reference to an item
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6);
    v.extend(6..1000000000);  // Força a realocação adicionando muitos elementos.... MESMO ASSIM NÃO DEU ERRO!!!

    println!("Hello world!");
}
