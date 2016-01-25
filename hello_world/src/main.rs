use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // let hex = 
        
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

fn main() {
    // println! is a macro
    println!("Hello, world!");
    println!("I'm a Rustacian!");
    
    println!("{} days", 31);
    
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    
    println!("{subject} {verb} {predicate}",
              predicate="over the lazy dog",
              subject="the quick brown fox",
              verb="jumps");
    
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    
    println!("{number:>width$}", number=1, width=6);
    
    // add padding
    println!("{number:>0width$}", number=1, width=6);
    
    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    // Compiler complaining about Structure - not used
    // struct Structure(i32);
    
    // println!("This struct `{}` won't print...", Structure(3));
    
    println!("Pi is roughly {0:.3}", 22.0 / 7.0);
    
    let complex = Complex { real: 3.3, imag: 7.2 };
    
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
    
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}
