use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn old_show(table: Table) {
    // outer loop takes ownership of the Table
    for (artist, works) in table {
        println!("works by {}:", artist);

        // inner loop takes ownership of Vec<String>
        for work in works {
            println!(" {}", work);
        }
    }
}

fn show(table: &Table) {
    // outer loop receives a shared reference to the Table
    // iterating over a shared reference to a HashMap produces shared references
    // to its keys and values, so artist is a &String and works is a &Vec<String>
    for (artist, works) in table {
        println!("works by {}:", artist);

        // iterating over a shared reference to a vector produces shared references
        // to its elements, so work is a &String
        for work in works {
            println!(" {}", work);
        }
    }
}

// &mut allows the fn to read and modify table without taking ownership
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn factorial(n: usize) -> usize {
    (1..n+1).product()
} 

// returns a reference to the smallest element of v
// v should have at least one element.
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}   

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
        vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
        vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
        vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    
    // ownership is transferred to the show function, leaving table uninitialized
    // show(table);

    // would produce error if above call to show was uncommented
    // println!("{:?}", table);

    // the fn doesn't need to modify table, jsut read
    show(&table);

    // works because ownership of the Table hasn't changed and table is still initialized
    println!("{:?}", table);

    sort_works(&mut table);

    println!("{:?}", table);

    // creating a reference and dereferencing is explicit, unlike in c++
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    // mutable ref
    let mut y = 32;
    let m = &mut y;
    *m += 32;   // modify y value
    assert!(*m == 64);
    assert!(y == 64);

    // . operator implicitly dereferences its left operand
    struct Anime { name: &'static str, bechdel_pass: bool }
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");   // equivalent to above

    // . operator can implicitly borrow a reference to its left operand for a method call
    let mut v = vec![1973, 1968];
    v.sort();   // implicitly borrows a mutable reference to v
    (&mut v).sort();    // equivalent

    // references to references
    // . operator follows as many references as needed to find the value
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
    assert_eq!(rrr.y, rr.y);

    // comparison operators also follow references
    let a = 10;
    let b = 10;
    let ra = &a;
    let rb = &b;
    let rra = &ra;
    let rrb = &rb;
    assert!(rra <= rrb);
    assert!(rra == rrb);

    // when you really want to compare references as addresses in memory
    assert!(!std::ptr::eq(ra, rb)); // not the same

    // operands of a comparison must be the same type, even for references
    // assert!(ra == rra); // error: comparing &i32 and &&i32
    assert!(ra == *rra);    // ok

    // borrowing a reference to an expression result
    let c = &factorial(6);
    assert_eq!(c + &1009, 1729);
}