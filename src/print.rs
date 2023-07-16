fn main() {
    // In general, the {} braces will be automatically replaced with any arguments, these arguments will be stringified
    println!("{} days", 31);

    // Positional arguments can be used, specifying an integer inside {} determines which additional argument will be replaced
    // Arguments start at 0
    println!("{0}, this {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments can be used instead of indices
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character after a colon
    println!("Base 10:                  {}", 96420);
    println!("Base 2 (binary):          {:b}", 69420);
    println!("Base 8 (octal):           {:o}", 69420);
    println!("Base 16 (hex):            {:x}", 69420);
    println!("Base 16 (hex capitals):   {:x}", 69420);

    // You can right-justify text with a specified width. This will output "    1" (four spaces with a 1 for a total width of 5)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes
    println!("{number:0>5}", number = 1);
    // And left-adjust by flipping the sign (left-pad)
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a $ at the end
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust will check to make sure the correct number of arguments are used
    println!("My name is {1}, {0} {1}", "James", "Bond");

    // Only types that implement fmt::Display can be formatted with {}
    // User-defined types do not implement fmt::Display by default
    #[allow(dead_code)] // disable dead_code which warns against an unused module
    struct Structure(i32);

    // Since Structure does not implement fmt::Display we can't format it with {}
    //println!("This struct {} won't print...", Structure(3));

    // You can directly capture the argument from a surrounding variable as a named argument rather than having to pass it into the println macro
    // The below code will output 1 right-padded by 4 additional spaces to a width of 5
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
