use std::fmt;

// Define a structure name List containing a Vec
struct List(Vec<i32>);

// Implement Display for List
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing and create a reference to vec
        let vec = &self.0;

        // Beginning output string with an open square bracket
        write!(f, "[")?; // Use ? to not finish yet (unless we error)

        // Iterator over v in vec while enumerating the iteration count in count
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma
            // use the ? operator to return on errors
            if count != 0 {
                write!(f, ", ")?;
            }
            // add this value
            write!(f, "[{count}]: {}", v)?;
        }

        // Close the opened bracket and return
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
