// Fix the issue in the above code (see FIXME) so that it runs without error.
// Try uncommenting the line that attempts to format the Structure struct (see TODO)
// Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)

use std::fmt;

fn main() {
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);
    // impl is explained in 1.2.2: Display
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{}` will print in Display", Structure(3));

    // You can also use the debug trait to print the struct as it is
    println!("This struct `{:?}` will print in Debug", Structure(3));

    let pi: f64 = 3.141592;
    let width: usize = 3;
    println!("Pi is roughly {pi:.width$}");
}
