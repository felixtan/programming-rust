// struct Person { name: String, birth: i32 }
struct Person { name: Option<String>, birth: i32 }

fn main() {
    let a = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let b = a;  // ownership of the vector is moved from a to b, a is uninitialized
    // let c = a;  // results in error: value used here after move

    // composers owns the new vector
    let mut composers = Vec::new();

    // Person struct owns the String and i32 values
    // composers owns the struct and indirectly owns the field values
    // composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: Some("Palestrina".to_string()), birth: 1525 });

    let d = vec!["liberte".to_string(), "egalite".to_string(), "fraternite".to_string()];

    // for loop takes ownership of the vector and dissects it into its elements
    for mut s in d {
        s.push('!');
        println!("{}", s);
    }
    // d is now uninitialized

    // error: move occurs because value has type `String`, which does not implement the `Copy` trait
    // rust doesnt track which elements of a vector have become uninitialized
    // let first_name = composers[0].name;

    // value of composers[0].name is transferred from composers to first_name
    // composers[0].name is replaced with None
    // let first_name = std::mem::replace(&mut composers[0].name, None);

    // same effect as above
    let first_name = composers[0].name.take();
}
