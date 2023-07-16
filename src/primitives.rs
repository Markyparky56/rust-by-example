fn main()
{
    // Type annotations
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_int = 5i32;

    println!("{logical} {a_float} {an_int}");

    // Default inferrence
    let default_float = 3.0; // f64
    let default_int = 7; // i32

    println!("{default_float} {default_int}");

    // Context inferrence can extend to later uses
    let mut inferred_type = 12;

    println!("{inferred_type}");

    inferred_type = 4294967296i64;

    println!("{inferred_type}");

    // Mutable variables can be changed
    let mut mutable = 12;

    println!("{mutable}");

    mutable = 21;

    println!("{mutable}");

    // Variables can be shadowed, this can change their type
    let mutable = true;

    println!("{mutable}");
}