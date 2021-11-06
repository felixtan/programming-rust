struct Person { 
    name: String, 
    birth: i32 
}

fn main() {
    // lives in the stack frame
    let mut composers = Vec::new();

    // without .to_string(), get error: expected struct `String`, found `&str`
    // https://blog.thoughtram.io/string-vs-str-in-rust/
    //
    // field values live in the heap
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}
