// Import the fmt module to make it available for use in this file
use std::fmt;

// Define a structure for which fmt::Display will be implemented
// This is a tuple struct name Structure that contains an i32
struct Structure(i32);

// To use the {} marker the trait fmt::Display must be implemented manually
impl fmt::Display for Structure {
    // This trait requires fmt with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: f
        // Returns fmt::Result which indicates whether the operation succeeded or failed
        // Note that write! uses syntax which is very similar to println!
        write!(f, "{}", self.0)
    }
}

// fmt::Display is not implemented for generic containers, fmt::Debug must be used for these cases
// New containers which are not generic can implement fmt::Display

// A structure holding two numbers, deriving a Debug format string
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement a custom Display for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use self.N to refer to each posiitonal data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement Display for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only x and y are denoted
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Define a structure for a Complex number
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

// And give it a Display
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Complex numbers are formatted as Real + Imaginary, with the imaginary part followed by an i
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Display: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big_range} and the small range is {small_range}");

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}