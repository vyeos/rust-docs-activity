// Use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:

// Display: 3.3 +7.2i
// Debug: Complex { real: 3.3, imag: 7.2 }

// Display: 4.7 -2.3i
// Debug: Complex { real: 4.7, imag: -2.3 }
// Bonus: Add a space after the +/- signs.

use std::fmt;

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct ComplexPoint2D {
    x: f64,
    y: f64,
}

impl fmt::Display for ComplexPoint2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.y < 0.0 { "-" } else { "+" };
        write!(f, "{} {} {}", self.x, sign, self.y.abs())
    }
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    let positive_complex_point = ComplexPoint2D { x: 7.8, y: 2.5 };
    let negative_complex_point = ComplexPoint2D { x: 9.4, y: -1.7 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("\nDisplay: {}", positive_complex_point);
    println!("Debug: {:?}", positive_complex_point);

    println!("\nDisplay: {}", negative_complex_point);
    println!("Debug: {:?}", negative_complex_point);
}
