// This structure cannot be printed either with fmt::Display or fmt::Debug
#[allow(dead_code)]
struct UnPrintable(i32);

// The drive attribute automatically creates the implementation require to make this struct printable with fmt::Debug (but not fmt::Display)
#[derive(Debug)]
struct DebugPrintable(i32);

// All std library types are automatically printable with {:?}

// Here we derive the fmt::Debug implementation for Structure, which contains a single i32
#[derive(Debug)]
struct Structure(i32);

// Putting Structure inside a of a second structure Deep which is also printable
#[derive(Debug)]
struct Deep(Structure);

// Rust also provides "pretty printing" with {:#?} (multi-line output with struct name and curly braces)
#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Printing with {:?} is similar to with {}
    println!("{:?} months in a year", 12); // Ordered arguments
    println!(
        "{1:?} {0:?} is the {actor:?} name",
        "Slater",
        "Christian",
        actor = "actor's"
    ); // indexed arguments and named arguments

    // Structure is printable
    println!("Now {:?} will print", Structure(3));

    // derived debug strings do not allow control over the output
    println!("Now {:?} will print", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{peter:#?}");
}
